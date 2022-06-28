use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::ssa::SsaBasicBlock;

struct SsaInsn{
    pub block: SsaBasicBlock,
    pub result: RegisterSpec,
}
impl SsaInsn{
    pub fn new(&self, result: &RegisterSpec, block: &SsaBasicBlock)    {
        if block==None        {
            throw NullPointerException::new("block == null");
        }        
        self->block=block;
        self->result=result;
    }
    pub fn makeFromRop(insn: &Insn, block: &SsaBasicBlock) -> SsaInsn    {
        return NormalSsaInsn::new(&insn, &block);
    }
    pub fn clone(&self) -> SsaInsn    {
        try        {
            return (SsaInsn*)super.clone();
        }        catch(        let ex: CloneNotSupportedException)        {
            throw RuntimeException::new("unexpected", &ex);
        }
    }
    pub fn getResult(&self) -> RegisterSpec    {
        return self.result;
    }
    pub fn setResult(&self, result: &RegisterSpec)    {
        if result==None        {
            throw NullPointerException::new("result == null");
        }        
        self->result=result;
    }
    pub fn getSources(&self) -> RegisterSpecList;
    pub fn getBlock(&self) -> SsaBasicBlock    {
        return self.block;
    }
    pub fn isResultReg(&self, reg: i32) -> boolean    {
        return self.result!=None&&self.result.getReg()==reg;
    }
    pub fn changeResultReg(&self, reg: i32)    {
        if self.result!=None        {
            self.result=self.result.withReg(reg);
        }        
    }
    pub fn setResultLocal(&self, local: &LocalItem)    {
        let oldItem: LocalItem = self.result.getLocalItem();
        if local!=oldItem&&(local==None||!local.equals(self.result.getLocalItem()))        {
            self.result=RegisterSpec::makeLocalOptional(self.result.getReg(), self.result.getType(), &local);
        }        
    }
    pub fn mapRegisters(&self, mapper: &RegisterMapper)    {
        let oldResult: RegisterSpec = self.result;
        self.result=mapper.map_RegisterSpec(&self.result);
        self.block.getParent().updateOneDefinition(self, &oldResult);
        mapSourceRegisters(&mapper);
    }
    pub fn mapSourceRegisters(&self, mapper: &RegisterMapper);
    pub fn getOpcode(&self) -> Rop;
    pub fn getOriginalRopInsn(&self) -> Insn;
    pub fn getLocalAssignment(&self) -> RegisterSpec    {
        if self.result!=None&&self.result.getLocalItem()!=None        {
            return self.result;
        }        
        return None;
    }
    pub fn isRegASource(&self, reg: i32) -> boolean    {
        return None!=getSources().specForRegister(reg);
    }
    pub fn toRopInsn(&self) -> Insn;
    pub fn isPhiOrMove(&self) -> boolean;
    pub fn hasSideEffect(&self) -> boolean;
    pub fn isNormalMoveInsn(&self) -> boolean    {
        return false;
    }
    pub fn isMoveException(&self) -> boolean    {
        return false;
    }
    pub fn canThrow(&self) -> boolean;
    pub fn accept(&self, v: &Visitor);
}
