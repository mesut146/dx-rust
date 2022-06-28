use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::form::Form45cc;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::code::MultiCstInsn;

let static THE_ONE: InsnFormat = Form45cc::new();
struct Form45cc{
}
impl Form45cc{
    pub const MAX_NUM_OPS: i32 = 5;
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = Form45cc::explicitize(insn.getRegisters());
        return InsnFormat::regListString(&regs)+", "+insn.cstString();
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
        if mci.getNumberOfConstants()!=2        {
            return false;
        }        
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
        return (Form45cc::wordCount(&regs)>=0);
    }
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        let regs: RegisterSpecList = insn.getRegisters();
        let sz: i32 = regs.size();
        let bits: BitSet = BitSet::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let reg: RegisterSpec = regs.get(i);
            bits.set_int_boolean(i, InsnFormat::unsignedFitsInNibble(reg.getReg()+reg.getCategory()-1));
        }
        return bits;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn)    {
        let mci: MultiCstInsn = (MultiCstInsn*)insn;
        let regB: i16 = mci.getIndex(0) as i16;
        let regH: i16 = mci.getIndex(1) as i16;
        let regs: RegisterSpecList = Form45cc::explicitize(insn.getRegisters());
        let regA: i32 = regs.size();
        let regC: i32 = if (regA>0) { regs.get(0).getReg() } else { 0 };
                let regD: i32 = if (regA>1) { regs.get(1).getReg() } else { 0 };
                        let regE: i32 = if (regA>2) { regs.get(2).getReg() } else { 0 };
                                let regF: i32 = if (regA>3) { regs.get(3).getReg() } else { 0 };
                                        let regG: i32 = if (regA>4) { regs.get(4).getReg() } else { 0 };
                                                InsnFormat::write_AnnotatedOutput_short_short_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, InsnFormat::makeByte(regG, regA)), regB, InsnFormat::codeUnit_int_int_int_int(regC, regD, regE, regF), regH);
                                            }
                                            pub fn wordCount(regs: &RegisterSpecList) -> i32                                            {
                                                let sz: i32 = regs.size();
                                                if sz>Form45cc::MAX_NUM_OPS                                                {
                                                    return -1;
                                                }                                                
                                                let result: i32 = 0;
                                                for(                                                let i: i32 = 0;;i<szi += 1)                                                {
                                                    let one: RegisterSpec = regs.get(i);
                                                    result+=one.getCategory();
                                                    if !InsnFormat::unsignedFitsInNibble(one.getReg()+one.getCategory()-1)                                                    {
                                                        return -1;
                                                    }                                                    
                                                }
                                                return if (result<=Form45cc::MAX_NUM_OPS) { result } else { -1 };
                                                    }
                                                    pub fn explicitize(orig: &RegisterSpecList) -> RegisterSpecList                                                    {
                                                        let wordCount: i32 = Form45cc::wordCount(&orig);
                                                        let sz: i32 = orig.size();
                                                        if wordCount==sz                                                        {
                                                            return orig;
                                                        }                                                        
                                                        let result: RegisterSpecList = RegisterSpecList::new(wordCount);
                                                        let wordAt: i32 = 0;
                                                        for(                                                        let i: i32 = 0;;i<szi += 1)                                                        {
                                                            let one: RegisterSpec = orig.get(i);
                                                            result.set_int_RegisterSpec(wordAt, &one);
                                                            if one.getCategory()==2                                                            {
                                                                result.set_int_RegisterSpec(wordAt+1, RegisterSpec::make_int_TypeBearer(one.getReg()+1, &Type::VOID));
                                                                wordAt+=2;
                                                            }                                                            else                                                             {
                                                                wordAt += 1;
                                                            }
                                                        }
                                                        result.setImmutable();
                                                        return result;
                                                    }
}
