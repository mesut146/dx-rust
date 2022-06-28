use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::dex::code::form::Form35c;
use crate::com::android::dx::rop::type::Type;

let static THE_ONE: InsnFormat = Form35c::new();
struct Form35c{
}
impl Form35c{
    pub const MAX_NUM_OPS: i32 = 5;
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = Form35c::explicitize(insn.getRegisters());
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
        return 3;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        if !(//insn instanceof CstInsn)        {
            return false;
        }        
        let ci: CstInsn = (CstInsn*)insn;
        let cpi: i32 = ci.getIndex();
        if !InsnFormat::unsignedFitsInShort(cpi)        {
            return false;
        }        
        let cst: Constant = ci.getConstant();
        if !((//cst instanceof CstMethodRef)||(//cst instanceof CstType)||(//cst instanceof CstCallSiteRef))        {
            return false;
        }        
        let regs: RegisterSpecList = ci.getRegisters();
        return (Form35c::wordCount(&regs)>=0);
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
        let cpi: i32 = ((CstInsn*)insn).getIndex();
        let regs: RegisterSpecList = Form35c::explicitize(insn.getRegisters());
        let sz: i32 = regs.size();
        let r0: i32 = if (sz>0) { regs.get(0).getReg() } else { 0 };
                let r1: i32 = if (sz>1) { regs.get(1).getReg() } else { 0 };
                        let r2: i32 = if (sz>2) { regs.get(2).getReg() } else { 0 };
                                let r3: i32 = if (sz>3) { regs.get(3).getReg() } else { 0 };
                                        let r4: i32 = if (sz>4) { regs.get(4).getReg() } else { 0 };
                                                InsnFormat::write_AnnotatedOutput_short_short_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, InsnFormat::makeByte(r4, sz)), cpi as i16, InsnFormat::codeUnit_int_int_int_int(r0, r1, r2, r3));
                                            }
                                            pub fn wordCount(regs: &RegisterSpecList) -> i32                                            {
                                                let sz: i32 = regs.size();
                                                if sz>Form35c::MAX_NUM_OPS                                                {
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
                                                return if (result<=Form35c::MAX_NUM_OPS) { result } else { -1 };
                                                    }
                                                    pub fn explicitize(orig: &RegisterSpecList) -> RegisterSpecList                                                    {
                                                        let wordCount: i32 = Form35c::wordCount(&orig);
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
