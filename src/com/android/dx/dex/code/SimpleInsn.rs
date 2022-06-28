use crate::helper;
use crate::com::android::dx::dex::code::SimpleInsn;

struct SimpleInsn{
}
impl SimpleInsn{
    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList)    {
        super(opcode,position,registers);

    }
    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn    {
        return SimpleInsn::new(&opcode, getPosition(), getRegisters());
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return SimpleInsn::new(getOpcode(), getPosition(), &registers);
    }
    pub fn argString(&self) -> String    {
        return None;
    }
}
