use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::ssa::LocalVariableInfo;
use crate::com::android::dx::ssa::LocalVariableExtractor;

struct LocalVariableExtractor{
    pub method: SsaMethod,
    pub blocks: ArrayList<SsaBasicBlock>,
    pub resultInfo: LocalVariableInfo,
    pub workSet: BitSet,
}
impl LocalVariableExtractor{
    pub fn extract(method: &SsaMethod) -> LocalVariableInfo    {
        let lve: LocalVariableExtractor = LocalVariableExtractor::new(&method);
        return lve.doit();
    }
    pub fn new(&self, method: &SsaMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        let blocks: ArrayList<SsaBasicBlock> = method.getBlocks();
        self->method=method;
        self->blocks=blocks;
        self->resultInfo=LocalVariableInfo::new(&method);
        self->workSet=BitSet::new(blocks.size());
    }
    pub fn doit(&self) -> LocalVariableInfo    {
        if self.method.getRegCount()>0        {
            for(            let bi: i32 = self.method.getEntryBlockIndex();;bi>=0bi=self.workSet.nextSetBit_int(0))            {
                self.workSet.clear_int(bi);
                processBlock(bi);
            }
        }        
        self.resultInfo.setImmutable();
        return self.resultInfo;
    }
    pub fn processBlock(&self, blockIndex: i32)    {
        let primaryState: RegisterSpecSet = self.resultInfo.mutableCopyOfStarts(blockIndex);
        let block: SsaBasicBlock = self.blocks.get(blockIndex);
        let insns: List<SsaInsn> = block.getInsns();
        let insnSz: i32 = insns.size();
        if blockIndex==self.method.getExitBlockIndex()        {
            return;
        }        
        let lastInsn: SsaInsn = insns.get(insnSz-1);
        let hasExceptionHandlers: boolean = lastInsn.getOriginalRopInsn().getCatches().size()!=0;
        let canThrowDuringLastInsn: boolean = hasExceptionHandlers&&(lastInsn.getResult()!=None);
        let freezeSecondaryStateAt: i32 = insnSz-1;
        let secondaryState: RegisterSpecSet = primaryState;
        for(        let i: i32 = 0;;i<insnSzi += 1)        {
            if canThrowDuringLastInsn&&(i==freezeSecondaryStateAt)            {
                primaryState.setImmutable();
                primaryState=primaryState.mutableCopy();
            }            
            let insn: SsaInsn = insns.get(i);
            let result: RegisterSpec;
            result=insn.getLocalAssignment();
            if result==None            {
                result=insn.getResult();
                if result!=None&&primaryState.get_int(result.getReg())!=None                {
                    primaryState.remove(primaryState.get_int(result.getReg()));
                }                
                continue;
            }            
            result=result.withSimpleType();
            let already: RegisterSpec = primaryState.get_RegisterSpec(&result);
            if !result.equals_Object(&already)            {
                let previous: RegisterSpec = primaryState.localItemToSpec(result.getLocalItem());
                if previous!=None&&(previous.getReg()!=result.getReg())                {
                    primaryState.remove(&previous);
                }                
                self.resultInfo.addAssignment(&insn, &result);
                primaryState.put(&result);
            }            
        }
        primaryState.setImmutable();
        let successors: IntList = block.getSuccessorList();
        let succSz: i32 = successors.size();
        let primarySuccessor: i32 = block.getPrimarySuccessorIndex();
        for(        let i: i32 = 0;;i<succSzi += 1)        {
            let succ: i32 = successors.get(i);
            let state: RegisterSpecSet = if (succ==primarySuccessor) { primaryState } else { secondaryState };
                    if self.resultInfo.mergeStarts(succ, &state)                    {
                        self.workSet.set_int(succ);
                    }                    
                }
            }
}
