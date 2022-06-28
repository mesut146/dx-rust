use crate::helper;
use crate::com::android::dx::dex::code::form::Form10x;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::SimpleInsn;

let static THE_ONE: InsnFormat = Form10x::new();
struct Form10x{
}
impl Form10x{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        return "";
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return "";
    }
    pub fn codeSize(&self) -> i32    {
        return 1;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        return (//insn instanceof SimpleInsn)&&(insn.getRegisters().size()==0);
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        InsnFormat::write_AnnotatedOutput_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, 0));
    }
}
