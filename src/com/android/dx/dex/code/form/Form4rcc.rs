use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::form::Form4rcc;
use crate::com::android::dx::dex::code::MultiCstInsn;

let static THE_ONE: InsnFormat = Form4rcc::new();
struct Form4rcc{
}
impl Form4rcc{
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
        return 4;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        if !(//insn instanceof MultiCstInsn)        {
            return false;
        }        
        let mci: MultiCstInsn = (MultiCstInsn*)insn;
        let methodIdx: i32 = mci.getIndex(0);
        let protoIdx: i32 = mci.getIndex(1);
        if !InsnFormat::unsignedFitsInShort(methodIdx)||!InsnFormat::unsignedFitsInShort(protoIdx)        {
            return false;
        }        
        let methodRef: Constant = mci.getConstant(0);
        if !(//methodRef instanceof CstMethodRef)        {
            return false;
        }        
        let protoRef: Constant = mci.getConstant(1);
        if !(//protoRef instanceof CstProtoRef)        {
            return false;
        }        
        let regs: RegisterSpecList = mci.getRegisters();
        let sz: i32 = regs.size();
        if sz==0        {
            return true;
        }        
        return (InsnFormat::unsignedFitsInByte(regs.getWordCount())&&InsnFormat::unsignedFitsInShort(sz)&&InsnFormat::unsignedFitsInShort(regs.get(0).getReg())&&InsnFormat::isRegListSequential(&regs));
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let mci: MultiCstInsn = (MultiCstInsn*)insn;
        let regB: i16 = mci.getIndex(0) as i16;
        let regH: i16 = mci.getIndex(1) as i16;
        let regs: RegisterSpecList = insn.getRegisters();
        let regC: i16 = 0;
        if regs.size()>0        {
            regC=regs.get(0).getReg() as i16;
        }        
        let regA: i32 = regs.getWordCount();
        InsnFormat::write_AnnotatedOutput_short_short_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regA), regB, regC, regH);
    }
}
