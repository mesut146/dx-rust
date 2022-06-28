use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::form::Form31i;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstLiteralBits;

let static THE_ONE: InsnFormat = Form31i::new();
struct Form31i{
}
impl Form31i{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return regs.get(0).regString()+", "+InsnFormat::literalBitsString(&value);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return InsnFormat::literalBitsComment(&value, 32);
    }
    pub fn codeSize(&self) -> i32    {
        return 3;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        if !((//insn instanceof CstInsn)&&(regs.size()==1)&&InsnFormat::unsignedFitsInByte(regs.get(0).getReg()))        {
            return false;
        }        
        let ci: CstInsn = (CstInsn*)insn;
        let cst: Constant = ci.getConstant();
        if !(//cst instanceof CstLiteralBits)        {
            return false;
        }        
        return ((CstLiteralBits*)cst).fitsInInt();
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let bits: BitSet = BitSet::new(1);
        bits.set_int_boolean(0, InsnFormat::unsignedFitsInByte(regs.get(0).getReg()));
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: i32 = ((CstLiteralBits*)((CstInsn*)insn).getConstant()).getIntBits();
        InsnFormat::write_AnnotatedOutput_short_int(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()), value);
    }
}
