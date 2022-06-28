use crate::helper;
use crate::com::android::dx::dex::code::form::Form10t;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::TargetInsn;

let static THE_ONE: InsnFormat = Form10t::new();
struct Form10t{
}
impl Form10t{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        return InsnFormat::branchString(&insn);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return InsnFormat::branchComment(&insn);
    }
    pub fn codeSize(&self) -> i32    {
        return 1;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        if !((//insn instanceof TargetInsn)&&(insn.getRegisters().size()==0))        {
            return false;
        }        
        let ti: TargetInsn = (TargetInsn*)insn;
        return if ti.hasTargetOffset() { branchFits(&ti) } else { true };
            }
            pub fn branchFits(&self, insn: &TargetInsn) -> boolean            {
                let offset: i32 = insn.getTargetOffset();
                return (offset!=0)&&InsnFormat::signedFitsInByte_int(offset);
            }
            pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)            {
                let offset: i32 = ((TargetInsn*)insn).getTargetOffset();
                InsnFormat::write_AnnotatedOutput_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, (offset&0xff)));
            }
}
