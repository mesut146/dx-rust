use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::Dops;

struct ZeroSizeInsn{
}
impl ZeroSizeInsn{
    pub fn new(&self, position: &SourcePosition)    {
        super(Dops.SPECIAL_FORMAT,position,RegisterSpecList.EMPTY);

    }
    pub fn codeSize(&self) -> i32    {
        return 0;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
    }
    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn    {
        throw RuntimeException::new("unsupported");
    }
    pub fn withRegisterOffset(&self, delta: i32) -> DalvInsn    {
        return withRegisters(getRegisters().withOffset(delta));
    }
}
