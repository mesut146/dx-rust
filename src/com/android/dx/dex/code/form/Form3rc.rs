use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::dex::code::form::Form3rc;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstCallSiteRef;

let static THE_ONE: InsnFormat = Form3rc::new();
struct Form3rc{
}
impl Form3rc{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        return InsnFormat::regRangeString(insn.getRegisters())+", "+insn.cstString();
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        if noteIndices        {
            return insn.cstComment();
        }        else         {
            return "";
        }
    }
    pub fn codeSize(&self) -> i32    {
        return 3;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        if !(//insn instanceof CstInsn)        {
            return false;
        }        
        let ci: CstInsn = (CstInsn*)insn;
        let cpi: i32 = ci.getIndex();
        let cst: Constant = ci.getConstant();
        if !InsnFormat::unsignedFitsInShort(cpi)        {
            return false;
        }        
        if !((//cst instanceof CstMethodRef)||(//cst instanceof CstType)||(//cst instanceof CstCallSiteRef))        {
            return false;
        }        
        let regs: RegisterSpecList = ci.getRegisters();
        let sz: i32 = regs.size();
        return (regs.size()==0)||(InsnFormat::isRegListSequential(&regs)&&InsnFormat::unsignedFitsInShort(regs.get(0).getReg())&&InsnFormat::unsignedFitsInByte(regs.getWordCount()));
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let regs: RegisterSpecList = insn.getRegisters();
        let cpi: i32 = ((CstInsn*)insn).getIndex();
        let firstReg: i32 = if (regs.size()==0) { 0 } else { regs.get(0).getReg() };
                let count: i32 = regs.getWordCount();
                InsnFormat::write_AnnotatedOutput_short_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, count), cpi as i16, firstReg as i16);
            }
}
