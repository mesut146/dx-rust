use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::LocalVariableInfo;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::util::Bits;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::rop::code::LocalVariableExtractor;

struct LocalVariableExtractor{
    pub method: RopMethod,
    pub blocks: BasicBlockList,
    pub resultInfo: LocalVariableInfo,
    pub workSet: Vec<i32>,
}
impl LocalVariableExtractor{
    pub fn extract(method: &RopMethod) -> LocalVariableInfo    {
        let lve: LocalVariableExtractor = LocalVariableExtractor::new(&method);
        return lve.doit();
    }
    pub fn new(&self, method: &RopMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        let blocks: BasicBlockList = method.getBlocks();
        let maxLabel: i32 = blocks.getMaxLabel();
        self->method=method;
        self->blocks=blocks;
        self->resultInfo=LocalVariableInfo::new(&method);
        self->workSet=Bits::makeBitSet(maxLabel);
    }
    pub fn doit(&self) -> LocalVariableInfo    {
        for(        let label: i32 = self.method.getFirstLabel();;label>=0label=Bits::findFirst_int[]_int(&self.workSet, 0))        {
            Bits::clear(&self.workSet, label);
            processBlock(label);
        }
        self.resultInfo.setImmutable();
        return self.resultInfo;
    }
    pub fn processBlock(&self, label: i32)    {
        let primaryState: RegisterSpecSet = self.resultInfo.mutableCopyOfStarts(label);
        let block: BasicBlock = self.blocks.labelToBlock(label);
        let insns: InsnList = block.getInsns();
        let insnSz: i32 = insns.size();
        let canThrowDuringLastInsn: boolean = block.hasExceptionHandlers()&&(insns.getLast().getResult()!=None);
        let freezeSecondaryStateAt: i32 = insnSz-1;
        let secondaryState: RegisterSpecSet = primaryState;
        for(        let i: i32 = 0;;i<insnSzi += 1)        {
            if canThrowDuringLastInsn&&(i==freezeSecondaryStateAt)            {
                primaryState.setImmutable();
                primaryState=primaryState.mutableCopy();
            }            
            let insn: Insn = insns.get(i);
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
        let successors: IntList = block.getSuccessors();
        let succSz: i32 = successors.size();
        let primarySuccessor: i32 = block.getPrimarySuccessor();
        for(        let i: i32 = 0;;i<succSzi += 1)        {
            let succ: i32 = successors.get(i);
            let state: RegisterSpecSet = if (succ==primarySuccessor) { primaryState } else { secondaryState };
                    if self.resultInfo.mergeStarts(succ, &state)                    {
                        Bits::set_int[]_int(&self.workSet, succ);
                    }                    
                }
            }
}
