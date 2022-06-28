use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::form::Form22t;
use crate::com::android::dx::dex::code::TargetInsn;

let static THE_ONE: InsnFormat = Form22t::new();
struct Form22t{
}
impl Form22t{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString()+", "+regs.get(1).regString()+", "+InsnFormat::branchString(&insn);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return InsnFormat::branchComment(&insn);
    }
    pub fn codeSize(&self) -> i32    {
        return 2;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        if !((//insn instanceof TargetInsn)&&(regs.size()==2)&&InsnFormat::unsignedFitsInNibble(regs.get(0).getReg())&&InsnFormat::unsignedFitsInNibble(regs.get(1).getReg()))        {
            return false;
        }        
        let ti: TargetInsn = (TargetInsn*)insn;
        return if ti.hasTargetOffset() { branchFits(&ti) } else { true };
            }
            pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet            {
                let regs: RegisterSpecList = insn.getRegisters();
                let bits: BitSet = BitSet::new(2);
                bits.set_int_boolean(0, InsnFormat::unsignedFitsInNibble(regs.get(0).getReg()));
                bits.set_int_boolean(1, InsnFormat::unsignedFitsInNibble(regs.get(1).getReg()));
                return bits;
            }
            pub fn branchFits(&self, insn: &TargetInsn) -> boolean            {
                let offset: i32 = insn.getTargetOffset();
                return (offset!=0)&&InsnFormat::signedFitsInShort_int(offset);
            }
            pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)            {
                let regs: RegisterSpecList = insn.getRegisters();
                let offset: i32 = ((TargetInsn*)insn).getTargetOffset();
                InsnFormat::write_AnnotatedOutput_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, InsnFormat::makeByte(regs.get(0).getReg(), regs.get(1).getReg())), offset as i16);
            }
}
