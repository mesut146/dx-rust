use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::rop::code::Rop;

struct IdenticalBlockCombiner{
    pub ropMethod: RopMethod,
    pub blocks: BasicBlockList,
    pub newBlocks: BasicBlockList,
}
impl IdenticalBlockCombiner{
    pub fn new(&self, rm: &RopMethod)    {
        self.ropMethod=rm;
        self.blocks=self.ropMethod.getBlocks();
        self.newBlocks=self.blocks.getMutableCopy();
    }
    pub fn process(&self) -> RopMethod    {
        let szBlocks: i32 = self.blocks.size();
        let toDelete: BitSet = BitSet::new(self.blocks.getMaxLabel());
        for(        let bindex: i32 = 0;;bindex<szBlocksbindex += 1)        {
            let b: BasicBlock = self.blocks.get(bindex);
            if toDelete.get_int(b.getLabel())            {
                continue;
            }            
            let preds: IntList = self.ropMethod.labelToPredecessors(b.getLabel());
            let szPreds: i32 = preds.size();
            for(            let i: i32 = 0;;i<szPredsi += 1)            {
                let iLabel: i32 = preds.get(i);
                let iBlock: BasicBlock = self.blocks.labelToBlock(iLabel);
                if toDelete.get_int(iLabel)||iBlock.getSuccessors().size()>1||iBlock.getFirstInsn().getOpcode().getOpcode()==RegOps::MOVE_RESULT                {
                    continue;
                }                
                let toCombine: IntList = IntList::new();
                for(                let j: i32 = i+1;;j<szPredsj += 1)                {
                    let jLabel: i32 = preds.get(j);
                    let jBlock: BasicBlock = self.blocks.labelToBlock(jLabel);
                    if jBlock.getSuccessors().size()==1&&IdenticalBlockCombiner::compareInsns(&iBlock, &jBlock)                    {
                        toCombine.add(jLabel);
                        toDelete.set_int(jLabel);
                    }                    
                }
                combineBlocks(iLabel, &toCombine);
            }
        }
        for(        let i: i32 = szBlocks-1;;i>=0i -= 1)        {
            if toDelete.get_int(self.newBlocks.get(i).getLabel())            {
                self.newBlocks.set_int_BasicBlock(i, None);
            }            
        }
        self.newBlocks.shrinkToFit();
        self.newBlocks.setImmutable();
        return RopMethod::new(&self.newBlocks, self.ropMethod.getFirstLabel());
    }
    pub fn compareInsns(a: &BasicBlock, b: &BasicBlock) -> boolean    {
        return a.getInsns().contentEquals(b.getInsns());
    }
    pub fn combineBlocks(&self, alphaLabel: i32, betaLabels: &IntList)    {
        let szBetas: i32 = betaLabels.size();
        for(        let i: i32 = 0;;i<szBetasi += 1)        {
            let betaLabel: i32 = betaLabels.get(i);
            let bb: BasicBlock = self.blocks.labelToBlock(betaLabel);
            let preds: IntList = self.ropMethod.labelToPredecessors(bb.getLabel());
            let szPreds: i32 = preds.size();
            for(            let j: i32 = 0;;j<szPredsj += 1)            {
                let predBlock: BasicBlock = self.newBlocks.labelToBlock(preds.get(j));
                replaceSucc(&predBlock, betaLabel, alphaLabel);
            }
        }
    }
    pub fn replaceSucc(&self, block: &BasicBlock, oldLabel: i32, newLabel: i32)    {
        let newSuccessors: IntList = block.getSuccessors().mutableCopy();
        let newPrimarySuccessor: i32;
        newSuccessors.set(newSuccessors.indexOf(oldLabel), newLabel);
        newPrimarySuccessor=block.getPrimarySuccessor();
        if newPrimarySuccessor==oldLabel        {
            newPrimarySuccessor=newLabel;
        }        
        newSuccessors.setImmutable();
        let newBB: BasicBlock = BasicBlock::new(block.getLabel(), block.getInsns(), &newSuccessors, newPrimarySuccessor);
        self.newBlocks.set_int_BasicBlock(self.newBlocks.indexOfLabel(block.getLabel()), &newBB);
    }
}
