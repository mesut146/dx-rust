use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::dex::code::form::Form22s;

let static THE_ONE: InsnFormat = Form22s::new();
struct Form22s{
}
impl Form22s{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return regs.get(0).regString()+", "+regs.get(1).regString()+", "+InsnFormat::literalBitsString(&value);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return InsnFormat::literalBitsComment(&value, 16);
    }
    pub fn codeSize(&self) -> i32    {
        return 2;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        let regs: RegisterSpecList = insn.getRegisters();
        if !((//insn instanceof CstInsn)&&(regs.size()==2)&&InsnFormat::unsignedFitsInNibble(regs.get(0).getReg())&&InsnFormat::unsignedFitsInNibble(regs.get(1).getReg()))        {
            return false;
        }        
        let ci: CstInsn = (CstInsn*)insn;
        let cst: Constant = ci.getConstant();
        if !(//cst instanceof CstLiteralBits)        {
            return false;
        }        
        let cb: CstLiteralBits = (CstLiteralBits*)cst;
        return cb.fitsInInt()&&InsnFormat::signedFitsInShort_int(cb.getIntBits());
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let bits: BitSet = BitSet::new(2);
        bits.set_int_boolean(0, InsnFormat::unsignedFitsInNibble(regs.get(0).getReg()));
        bits.set_int_boolean(1, InsnFormat::unsignedFitsInNibble(regs.get(1).getReg()));
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: i32 = ((CstLiteralBits*)((CstInsn*)insn).getConstant()).getIntBits();
        InsnFormat::write_AnnotatedOutput_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, InsnFormat::makeByte(regs.get(0).getReg(), regs.get(1).getReg())), value as i16);
    }
}
