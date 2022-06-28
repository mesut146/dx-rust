use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::Dops;

struct VariableSizeInsn{
}
impl VariableSizeInsn{
    pub fn new(&self, position: &SourcePosition, registers: &RegisterSpecList)    {
        super(Dops.SPECIAL_FORMAT,position,registers);

    }
    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn    {
        throw RuntimeException::new("unsupported");
    }
    pub fn withRegisterOffset(&self, delta: i32) -> DalvInsn    {
        return withRegisters(getRegisters().withOffset(delta));
    }
}
