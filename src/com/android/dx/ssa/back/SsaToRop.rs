use crate::helper;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::ssa::back::LivenessAnalyzer;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::ssa::back::IdenticalBlockCombiner;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::ssa::back::SsaToRop;
use crate::com::android::dx::ssa::SsaBasicBlock::Visitor;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::ssa::back::InterferenceGraph;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::ssa::back::SsaToRop::PhiVisitor;
use crate::com::android::dx::ssa::back::FirstFitLocalCombiningAllocator;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::ssa::BasicRegisterMapper;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::ssa::back::RegisterAllocator;

struct SsaToRop{
    pub ssaMeth: SsaMethod,
    pub minimizeRegisters: boolean,
    pub interference: InterferenceGraph,
}
impl SsaToRop{
    pub const DEBUG: boolean = false;
    pub fn convertToRopMethod(ssaMeth: &SsaMethod, minimizeRegisters: boolean) -> RopMethod    {
        return SsaToRop::new(&ssaMeth, minimizeRegisters).convert();
    }
    pub fn new(&self, ssaMethod: &SsaMethod, minimizeRegisters: boolean)    {
        self->minimizeRegisters=minimizeRegisters;
        self->ssaMeth=ssaMethod;
        self->interference=LivenessAnalyzer::constructInterferenceGraph(&ssaMethod);
    }
    pub fn convert(&self) -> RopMethod    {
        if SsaToRop::DEBUG        {
            self.interference.dumpToStdout();
        }        
        let allocator: RegisterAllocator = FirstFitLocalCombiningAllocator::new(&self.ssaMeth, &self.interference, self.minimizeRegisters);
        let mapper: RegisterMapper = allocator.allocateRegisters();
        if SsaToRop::DEBUG        {
            System::out.println_String("Printing reg map");
            System::out.println_String(((BasicRegisterMapper*)mapper).toHuman());
        }        
        self.ssaMeth.setBackMode();
        self.ssaMeth.mapRegisters(&mapper);
        removePhiFunctions();
        if allocator.wantsParamsMovedHigh()        {
            moveParametersToHighRegisters();
        }        
        removeEmptyGotos();
        let ropMethod: RopMethod = RopMethod::new(convertBasicBlocks(), self.ssaMeth.blockIndexToRopLabel(self.ssaMeth.getEntryBlockIndex()));
        ropMethod=IdenticalBlockCombiner::new(&ropMethod).process();
        return ropMethod;
    }
    pub fn removeEmptyGotos(&self)    {
        let blocks: ArrayList<SsaBasicBlock> = self.ssaMeth.getBlocks();
        self.ssaMeth.forEachBlockDepthFirst(false, /*new SsaBasicBlock.Visitor(){
  @Override public void visitBlock(  SsaBasicBlock b,  SsaBasicBlock parent){
    ArrayList<SsaInsn> insns=b.getInsns();
    if ((insns.size() == 1) && (insns.get(0).getOpcode() == Rops.GOTO)) {
      BitSet preds=(BitSet)b.getPredecessors().clone();
      for (int i=preds.nextSetBit(0); i >= 0; i=preds.nextSetBit(i + 1)) {
        SsaBasicBlock pb=blocks.get(i);
        pb.replaceSuccessor(b.getIndex(),b.getPrimarySuccessorIndex());
      }
    }
  }
}
*/);
    }
    pub fn removePhiFunctions(&self)    {
        let blocks: ArrayList<SsaBasicBlock> = self.ssaMeth.getBlocks();
        for block in blocks        {
            block.forEachPhiInsn(PhiVisitor::new(&blocks));
            block.removeAllPhiInsns();
        }
        for block in blocks        {
            block.scheduleMovesFromPhis();
        }
    }
    pub fn moveParametersToHighRegisters(&self)    {
        let paramWidth: i32 = self.ssaMeth.getParamWidth();
        let mapper: BasicRegisterMapper = BasicRegisterMapper::new(self.ssaMeth.getRegCount());
        let regCount: i32 = self.ssaMeth.getRegCount();
        for(        let i: i32 = 0;;i<regCounti += 1)        {
            if i<paramWidth            {
                mapper.addMapping(i, regCount-paramWidth+i, 1);
            }            else             {
                mapper.addMapping(i, i-paramWidth, 1);
            }
        }
        if SsaToRop::DEBUG        {
            System::out.printf_String_Object[]("Moving %d registers from 0 to %d\n", paramWidth, regCount-paramWidth);
        }        
        self.ssaMeth.mapRegisters(&mapper);
    }
    pub fn convertBasicBlocks(&self) -> BasicBlockList    {
        let blocks: ArrayList<SsaBasicBlock> = self.ssaMeth.getBlocks();
        let exitBlock: SsaBasicBlock = self.ssaMeth.getExitBlock();
        let reachable: BitSet = self.ssaMeth.computeReachability();
        let ropBlockCount: i32 = reachable.cardinality();
        if exitBlock!=None&&reachable.get_int(exitBlock.getIndex())        {
            ropBlockCount-=1;
        }        
        let result: BasicBlockList = BasicBlockList::new(ropBlockCount);
        let ropBlockIndex: i32 = 0;
        for b in blocks        {
            if reachable.get_int(b.getIndex())&&b!=exitBlock            {
                result.set_int_BasicBlock(ropBlockIndex += 1, convertBasicBlock(&b));
            }            
        }
        if exitBlock!=None&&!exitBlock.getInsns().isEmpty()        {
            throw RuntimeException::new("Exit block must have no insns when leaving SSA form");
        }        
        return result;
    }
    pub fn verifyValidExitPredecessor(&self, b: &SsaBasicBlock)    {
        let insns: ArrayList<SsaInsn> = b.getInsns();
        let lastInsn: SsaInsn = insns.get(insns.size()-1);
        let opcode: Rop = lastInsn.getOpcode();
        if opcode.getBranchingness()!=Rop::BRANCH_RETURN&&opcode!=Rops::THROW        {
            throw RuntimeException::new("Exit predecessor must end"+" in valid exit statement.");
        }        
    }
    pub fn convertBasicBlock(&self, block: &SsaBasicBlock) -> BasicBlock    {
        let successorList: IntList = block.getRopLabelSuccessorList();
        let primarySuccessorLabel: i32 = block.getPrimarySuccessorRopLabel();
        let exitBlock: SsaBasicBlock = self.ssaMeth.getExitBlock();
        let exitRopLabel: i32 = if (exitBlock==None) { -1 } else { exitBlock.getRopLabel() };
                if successorList.contains(exitRopLabel)                {
                    if successorList.size()>1                    {
                        throw RuntimeException::new("Exit predecessor must have no other successors"+Hex::u2(block.getRopLabel()));
                    }                    else                     {
                        successorList=IntList::EMPTY;
                        primarySuccessorLabel=-1;
                        verifyValidExitPredecessor(&block);
                    }
                }                
                successorList.setImmutable();
                let result: BasicBlock = BasicBlock::new(block.getRopLabel(), convertInsns(block.getInsns()), &successorList, primarySuccessorLabel);
                return result;
            }
            pub fn convertInsns(&self, ssaInsns: &ArrayList<SsaInsn>) -> InsnList            {
                let insnCount: i32 = ssaInsns.size();
                let result: InsnList = InsnList::new(insnCount);
                for(                let i: i32 = 0;;i<insnCounti += 1)                {
                    result.set_int_Insn(i, ssaInsns.get(i).toRopInsn());
                }
                result.setImmutable();
                return result;
            }
            pub fn getRegistersByFrequency(&self) -> Vec<i32>            {
                let regCount: i32 = self.ssaMeth.getRegCount();
                let ret: Vec<Integer> = new Integer[regCount];
                for(                let i: i32 = 0;;i<regCounti += 1)                {
                    ret[i]=i;
                }
                Arrays::sort_Integer[]_Comparator<? super Integer>(&ret, /*new Comparator<Integer>(){
  @Override public int compare(  Integer o1,  Integer o2){
    return ssaMeth.getUseListForRegister(o2).size() - ssaMeth.getUseListForRegister(o1).size();
  }
}
*/);
                let result: i32 = new int[regCount];
                for(                let i: i32 = 0;;i<regCounti += 1)                {
                    result[i]=ret[i];
                }
                return result;
            }
}
