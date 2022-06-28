use crate::helper;
use crate::com::android::dx::dex::code::form::Form21h;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstLiteralBits;

let static THE_ONE: InsnFormat = Form21h::new();
struct Form21h{
}
impl Form21h{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return regs.get(0).regString()+", "+InsnFormat::literalBitsString(&value);
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        let value: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
        return InsnFormat::literalBitsComment(&value, if (regs.get(0).getCategory()==1) { 32 } else { 64 });
            }
            pub fn codeSize(&self) -> i32            {
                return 2;
            }
            pub fn isCompatible(&self, insn: &DalvInsn) -> boolean            {
                let regs: RegisterSpecList = insn.getRegisters();
                if !((//insn instanceof CstInsn)&&(regs.size()==1)&&InsnFormat::unsignedFitsInByte(regs.get(0).getReg()))                {
                    return false;
                }                
                let ci: CstInsn = (CstInsn*)insn;
                let cst: Constant = ci.getConstant();
                if !(//cst instanceof CstLiteralBits)                {
                    return false;
                }                
                let cb: CstLiteralBits = (CstLiteralBits*)cst;
                if regs.get(0).getCategory()==1                {
                    let bits: i32 = cb.getIntBits();
                    return ((bits&0xffff)==0);
                }                else                 {
                    let bits: i64 = cb.getLongBits();
                    return ((bits&0xffffffffffffL)==0);
                }
            }
            pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet            {
                let regs: RegisterSpecList = insn.getRegisters();
                let bits: BitSet = BitSet::new(1);
                bits.set_int_boolean(0, InsnFormat::unsignedFitsInByte(regs.get(0).getReg()));
                return bits;
            }
            pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)            {
                let regs: RegisterSpecList = insn.getRegisters();
                let cb: CstLiteralBits = (CstLiteralBits*)((CstInsn*)insn).getConstant();
                let bits: i16;
                if regs.get(0).getCategory()==1                {
                    bits=(cb.getIntBits()>>>16) as i16;
                }                else                 {
                    bits=(cb.getLongBits()>>>48) as i16;
                }
                InsnFormat::write_AnnotatedOutput_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()), bits);
            }
}
