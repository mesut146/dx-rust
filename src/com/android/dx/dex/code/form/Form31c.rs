use crate::helper;
use crate::com::android::dx::dex::code::form::Form31c;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstFieldRef;

let static THE_ONE: InsnFormat = Form31c::new();
struct Form31c{
}
impl Form31c{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        return regs.get(0).regString()+", "+insn.cstString();
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
        let regs: RegisterSpecList = insn.getRegisters();
        let reg: RegisterSpec;
        match regs.size(){1 =>             {
                reg=regs.get(0);
                break;
            }2 =>             {
                reg=regs.get(0);
                if reg.getReg()!=regs.get(1).getReg()                {
                    return false;
                }                
                break;
            }        _ => {}        {
            return false;
        }    }
    if !InsnFormat::unsignedFitsInByte(reg.getReg())    {
        return false;
    }    
    let ci: CstInsn = (CstInsn*)insn;
    let cst: Constant = ci.getConstant();
    return (//cst instanceof CstType)||(//cst instanceof CstFieldRef)||(//cst instanceof CstString);
}
pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet{
    let regs: RegisterSpecList = insn.getRegisters();
    let sz: i32 = regs.size();
    let bits: BitSet = BitSet::new(sz);
    let compat: boolean = InsnFormat::unsignedFitsInByte(regs.get(0).getReg());
    if sz==1    {
        bits.set_int_boolean(0, compat);
    }    else     {
        if regs.get(0).getReg()==regs.get(1).getReg()        {
            bits.set_int_boolean(0, compat);
            bits.set_int_boolean(1, compat);
        }        
    }
    return bits;
}
pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn){
    let regs: RegisterSpecList = insn.getRegisters();
    let cpi: i32 = ((CstInsn*)insn).getIndex();
    InsnFormat::write_AnnotatedOutput_short_int(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, regs.get(0).getReg()), cpi);
}
}
