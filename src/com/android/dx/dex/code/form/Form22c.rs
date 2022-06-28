use crate::helper;
use crate::com::android::dx::dex::code::form::Form22c;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstFieldRef;

let static THE_ONE: InsnFormat = Form22c::new();
struct Form22c{
}
impl Form22c{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString()+", "+regs.get(1).regString()+", "+insn.cstString();
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        if noteIndices        {
            return insn.cstComment();
        }        else         {
            return "";
        }
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
        let cpi: i32 = ci.getIndex();
        if !InsnFormat::unsignedFitsInShort(cpi)        {
            return false;
        }        
        let cst: Constant = ci.getConstant();
        return (//cst instanceof CstType)||(//cst instanceof CstFieldRef);
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
        let cpi: i32 = ((CstInsn*)insn).getIndex();
        InsnFormat::write_AnnotatedOutput_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, InsnFormat::makeByte(regs.get(0).getReg(), regs.get(1).getReg())), cpi as i16);
    }
}
