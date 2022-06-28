use crate::helper;
use crate::com::android::dx::dex::code::form::Form12x;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::SimpleInsn;

let static THE_ONE: InsnFormat = Form12x::new();
struct Form12x{
}
impl Form12x{
    pub fn new(&self)    {
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String    {
        let regs: RegisterSpecList = insn.getRegisters();
        let sz: i32 = regs.size();
        return regs.get(sz-2).regString()+", "+regs.get(sz-1).regString();
    }
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        return "";
    }
    pub fn codeSize(&self) -> i32    {
        return 1;
    }
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean    {
        if !(//insn instanceof SimpleInsn)        {
            return false;
        }        
        let regs: RegisterSpecList = insn.getRegisters();
        let rs1: RegisterSpec;
        let rs2: RegisterSpec;
        match regs.size(){2 =>             {
                rs1=regs.get(0);
                rs2=regs.get(1);
                break;
            }3 =>             {
                rs1=regs.get(1);
                rs2=regs.get(2);
                if rs1.getReg()!=regs.get(0).getReg()                {
                    return false;
                }                
                break;
            }        _ => {}        {
            return false;
        }    }
    return InsnFormat::unsignedFitsInNibble(rs1.getReg())&&InsnFormat::unsignedFitsInNibble(rs2.getReg());
}
pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet{
    let regs: RegisterSpecList = insn.getRegisters();
    let bits: BitSet = BitSet::new(2);
    let r0: i32 = regs.get(0).getReg();
    let r1: i32 = regs.get(1).getReg();
    match regs.size(){2 =>         {
            bits.set_int_boolean(0, InsnFormat::unsignedFitsInNibble(r0));
            bits.set_int_boolean(1, InsnFormat::unsignedFitsInNibble(r1));
            break;
        }3 =>         {
            if r0!=r1            {
                bits.set_int_boolean(0, false);
                bits.set_int_boolean(1, false);
            }            else             {
                let dstRegComp: boolean = InsnFormat::unsignedFitsInNibble(r1);
                bits.set_int_boolean(0, dstRegComp);
                bits.set_int_boolean(1, dstRegComp);
            }
            bits.set_int_boolean(2, InsnFormat::unsignedFitsInNibble(regs.get(2).getReg()));
            break;
        }    _ => {}    {
        throw AssertionError::new();
    }}
return bits;
}
pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn){
let regs: RegisterSpecList = insn.getRegisters();
let sz: i32 = regs.size();
InsnFormat::write_AnnotatedOutput_short(&out, InsnFormat::opcodeUnit_DalvInsn_int(&insn, InsnFormat::makeByte(regs.get(sz-2).getReg(), regs.get(sz-1).getReg())));
}
}
