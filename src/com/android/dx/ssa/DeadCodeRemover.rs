use crate::helper;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::ssa::DeadCodeRemover::NoSideEffectVisitor;
use crate::com::android::dx::ssa::DeadCodeRemover;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;

struct DeadCodeRemover{
    pub ssaMeth: SsaMethod,
    pub regCount: i32,
    pub worklist: BitSet,
    pub useList: Vec<ArrayList<SsaInsn>>,
}
impl DeadCodeRemover{
    pub fn process(ssaMethod: &SsaMethod)    {
        let dc: DeadCodeRemover = DeadCodeRemover::new(&ssaMethod);
        dc.run();
    }
    pub fn new(&self, ssaMethod: &SsaMethod)    {
        self->ssaMeth=ssaMethod;
        self.regCount=ssaMethod.getRegCount();
        self.worklist=BitSet::new(self.regCount);
        self.useList=self.ssaMeth.getUseListCopy();
    }
    pub fn run(&self)    {
        pruneDeadInstructions();
        let deletedInsns: HashSet<SsaInsn> = HashSet<SsaInsn>::new();
        self.ssaMeth.forEachInsn(NoSideEffectVisitor::new(&self.worklist));
        let regV: i32;
        while 0<=(regV=self.worklist.nextSetBit_int(0))        {
            self.worklist.clear_int(regV);
            if self.useList[regV].size()==0||isCircularNoSideEffect(regV, None)            {
                let insnS: SsaInsn = self.ssaMeth.getDefinitionForRegister(regV);
                if deletedInsns.contains(&insnS)                {
                    continue;
                }                
                let sources: RegisterSpecList = insnS.getSources();
                let sz: i32 = sources.size();
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let source: RegisterSpec = sources.get(i);
                    self.useList[source.getReg()].remove_Object(&insnS);
                    if !DeadCodeRemover::hasSideEffect(self.ssaMeth.getDefinitionForRegister(source.getReg()))                    {
                        self.worklist.set_int(source.getReg());
                    }                    
                }
                deletedInsns.add(&insnS);
            }            
        }
        self.ssaMeth.deleteInsns(&deletedInsns);
    }
    pub fn pruneDeadInstructions(&self)    {
        let deletedInsns: HashSet<SsaInsn> = HashSet<SsaInsn>::new();
        let reachable: BitSet = self.ssaMeth.computeReachability();
        let blocks: ArrayList<SsaBasicBlock> = self.ssaMeth.getBlocks();
        let blockIndex: i32 = 0;
        while (blockIndex=reachable.nextClearBit(blockIndex))<blocks.size()        {
            let block: SsaBasicBlock = blocks.get(blockIndex);
            blockIndex += 1;
            for(            let i: i32 = 0;;i<block.getInsns().size()i += 1)            {
                let insn: SsaInsn = block.getInsns().get(i);
                let sources: RegisterSpecList = insn.getSources();
                let sourcesSize: i32 = sources.size();
                if sourcesSize!=0                {
                    deletedInsns.add(&insn);
                }                
                for(                let j: i32 = 0;;j<sourcesSizej += 1)                {
                    let source: RegisterSpec = sources.get(j);
                    self.useList[source.getReg()].remove_Object(&insn);
                }
                let result: RegisterSpec = insn.getResult();
                if result==None{                    continue;                }                
                for use in self.useList[result.getReg()]                {
                    if //use instanceof PhiInsn                    {
                        let phiUse: PhiInsn = (PhiInsn*)use_renamed;
                        phiUse.removePhiRegister(&result);
                    }                    
                }
            }
        }
        self.ssaMeth.deleteInsns(&deletedInsns);
    }
    pub fn isCircularNoSideEffect(&self, regV: i32, set: &BitSet) -> boolean    {
        if (set!=None)&&set.get_int(regV)        {
            return true;
        }        
        for use in self.useList[regV]        {
            if DeadCodeRemover::hasSideEffect(&use_renamed)            {
                return false;
            }            
        }
        if set==None        {
            set=BitSet::new(self.regCount);
        }        
        set.set_int(regV);
        for use in self.useList[regV]        {
            let result: RegisterSpec = use_renamed.getResult();
            if result==None||!isCircularNoSideEffect(result.getReg(), &set)            {
                return false;
            }            
        }
        return true;
    }
    pub fn hasSideEffect(insn: &SsaInsn) -> boolean    {
        if insn==None        {
            return true;
        }        
        return insn.hasSideEffect();
    }
}
