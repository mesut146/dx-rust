use crate::helper;
use crate::com::android::dx::dex::code::form::SpecialFormat;

let static THE_ONE: InsnFormat = SpecialFormat::new();
struct SpecialFormat{
}
impl SpecialFormat{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        throw RuntimeException::new("unsupported");
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        throw RuntimeException::new("unsupported");
    }
    pub fn codeSize(&self) -> i32    {
        throw RuntimeException::new("unsupported");
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        return true;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        throw RuntimeException::new("unsupported");
    }
}
