use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::form::Form21t;
use crate::com::android::dx::dex::code::TargetInsn;

let static THE_ONE: InsnFormat = Form21t::new();
struct Form21t{
}
impl Form21t{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString()+", "+InsnFormat::branchString(&insn);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return InsnFormat::branchComment(&insn);
    }
    pub fn codeSize(&self) -> i32    {
        return 2;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        if !((//insn instanceof TargetInsn)&&(regs.size()==1)&&InsnFormat::unsignedFitsInByte(regs.get(0).getReg()))        {
            return false;
        }        
        let ti: TargetInsn = (TargetInsn*)insn;
        return if ti.hasTargetOffset() { branchFits(&ti) } else { true };
            }
            pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet            {
                let regs: RegisterSpecList = insn.getRegisters();
                let bits: BitSet = BitSet::new(1);
                bits.set_int_boolean(0, InsnFormat::unsignedFitsInByte(regs.get(0).getReg()));
                return bits;
            }
            pub fn branchFits(&self, insn: &TargetInsn) -> boolean            {
                let offset: i32 = insn.getTargetOffset();
                return (offset!=0)&&InsnFormat::signedFitsInShort_int(offset);
            }
            pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)            {
                let regs: RegisterSpecList = insn.getRegisters();
                let offset: i32 = ((TargetInsn*)insn).getTargetOffset();
                InsnFormat::write_AnnotatedOutput_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()), offset as i16);
            }
}
