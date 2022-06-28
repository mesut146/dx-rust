use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::form::Form23x;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::SimpleInsn;

let static THE_ONE: InsnFormat = Form23x::new();
struct Form23x{
}
impl Form23x{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString()+", "+regs.get(1).regString()+", "+regs.get(2).regString();
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return "";
    }
    pub fn codeSize(&self) -> i32    {
        return 2;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        return (//insn instanceof SimpleInsn)&&(regs.size()==3)&&InsnFormat::unsignedFitsInByte(regs.get(0).getReg())&&InsnFormat::unsignedFitsInByte(regs.get(1).getReg())&&InsnFormat::unsignedFitsInByte(regs.get(2).getReg());
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let bits: BitSet = BitSet::new(3);
        bits.set_int_boolean(0, InsnFormat::unsignedFitsInByte(regs.get(0).getReg()));
        bits.set_int_boolean(1, InsnFormat::unsignedFitsInByte(regs.get(1).getReg()));
        bits.set_int_boolean(2, InsnFormat::unsignedFitsInByte(regs.get(2).getReg()));
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        InsnFormat::write_AnnotatedOutput_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()), InsnFormat::codeUnit_int_int(regs.get(1).getReg(), regs.get(2).getReg()));
    }
}
