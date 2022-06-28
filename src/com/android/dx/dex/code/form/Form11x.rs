use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::SimpleInsn;
use crate::com::android::dx::dex::code::form::Form11x;

let static THE_ONE: InsnFormat = Form11x::new();
struct Form11x{
}
impl Form11x{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString();
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return "";
    }
    pub fn codeSize(&self) -> i32    {
        return 1;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        return (//insn instanceof SimpleInsn)&&(regs.size()==1)&&InsnFormat::unsignedFitsInByte(regs.get(0).getReg());
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let bits: BitSet = BitSet::new(1);
        bits.set_int_boolean(0, InsnFormat::unsignedFitsInByte(regs.get(0).getReg()));
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        InsnFormat::write_AnnotatedOutput_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()));
    }
}
