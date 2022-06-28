use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstLiteral64;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::dex::code::form::Form51l;

let static THE_ONE: InsnFormat = Form51l::new();
struct Form51l{
}
impl Form51l{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return regs.get(0).regString()+", "+InsnFormat::literalBitsString(&value);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return InsnFormat::literalBitsComment(&value, 64);
    }
    pub fn codeSize(&self) -> i32    {
        return 5;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        if !((//insn instanceof CstInsn)&&(regs.size()==1)&&InsnFormat::unsignedFitsInByte(regs.get(0).getReg()))        {
            return false;
        }        
        let ci: CstInsn = (CstInsn*)insn;
        let cst: Constant = ci.getConstant();
        return (//cst instanceof CstLiteral64);
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let bits: BitSet = BitSet::new(1);
        bits.set_int_boolean(0, InsnFormat::unsignedFitsInByte(regs.get(0).getReg()));
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: i64 = ((CstLiteral64*)((CstInsn*)insn).getConstant()).getLongBits();
        InsnFormat::write_AnnotatedOutput_short_long(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()), value);
    }
}
