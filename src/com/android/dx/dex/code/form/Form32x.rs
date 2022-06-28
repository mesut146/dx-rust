use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::SimpleInsn;
use crate::com::android::dx::dex::code::form::Form32x;

let static THE_ONE: InsnFormat = Form32x::new();
struct Form32x{
}
impl Form32x{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString()+", "+regs.get(1).regString();
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return "";
    }
    pub fn codeSize(&self) -> i32    {
        return 3;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        return (//insn instanceof SimpleInsn)&&(regs.size()==2)&&InsnFormat::unsignedFitsInShort(regs.get(0).getReg())&&InsnFormat::unsignedFitsInShort(regs.get(1).getReg());
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let bits: BitSet = BitSet::new(2);
        bits.set_int_boolean(0, InsnFormat::unsignedFitsInShort(regs.get(0).getReg()));
        bits.set_int_boolean(1, InsnFormat::unsignedFitsInShort(regs.get(1).getReg()));
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        InsnFormat::write_AnnotatedOutput_short_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, 0), regs.get(0).getReg() as i16, regs.get(1).getReg() as i16);
    }
}
