use crate::helper;
use crate::com::android::dx::dex::code::form::Form30t;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::TargetInsn;

let static THE_ONE: InsnFormat = Form30t::new();
struct Form30t{
}
impl Form30t{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        return InsnFormat::branchString(&insn);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return InsnFormat::branchComment(&insn);
    }
    pub fn codeSize(&self) -> i32    {
        return 3;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        if !((//insn instanceof TargetInsn)&&(insn.getRegisters().size()==0))        {
            return false;
        }        
        return true;
    }
    pub fn branchFits(&self, insn: &TargetInsn) -> boolean    {
        return true;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let offset: i32 = ((TargetInsn*)insn).getTargetOffset();
        InsnFormat::write_AnnotatedOutput_short_int(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, 0), offset);
    }
}
