use crate::helper;
use crate::com::android::dx::dex::code::InsnFormat;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::Dop;

struct FixedSizeInsn{
}
impl FixedSizeInsn{
    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList)    {
        super(opcode,position,registers);

    }
    pub fn codeSize(&self) -> i32    {
        return getOpcode().getFormat().codeSize();
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
        getOpcode().getFormat().writeTo(&out, self);
    }
    pub fn withRegisterOffset(&self, delta: i32) -> DalvInsn    {
        return withRegisters(getRegisters().withOffset(delta));
    }
    pub fn listingString0(&self, noteIndices: boolean) -> String    {
        return getOpcode().getFormat().listingString(self, noteIndices);
    }
}
