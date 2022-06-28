use crate::helper;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::code::TargetInsn;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstLiteral64;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::cst::CstLiteralBits;

struct InsnFormat{
}
impl InsnFormat{
    pub const ALLOW_EXTENDED_OPCODES: boolean = true;
    pub fn listingString(&self, insn: &DalvInsn, noteIndices: boolean) -> String    {
        let op: String = insn.getOpcode().getName();
        let arg: String = insnArgString(&insn);
        let comment: String = insnCommentString(&insn, noteIndices);
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String(&op);
        if arg.length()!=0        {
            sb.append_char(' ');
            sb.append_String(&arg);
        }        
        if comment.length()!=0        {
            sb.append_String(" // ");
            sb.append_String(&comment);
        }        
        return sb.toString();
    }
    pub fn insnArgString(&self, insn: &DalvInsn) -> String;
    pub fn insnCommentString(&self, insn: &DalvInsn, noteIndices: boolean) -> String;
    pub fn codeSize(&self) -> i32;
    pub fn isCompatible(&self, insn: &DalvInsn) -> boolean;
    pub fn compatibleRegs(&self, insn: &DalvInsn) -> BitSet    {
        return BitSet::new();
    }
    pub fn branchFits(&self, insn: &TargetInsn) -> boolean    {
        return false;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput, insn: &DalvInsn);
    pub fn regListString(list: &RegisterSpecList) -> String    {
        let sz: i32 = list.size();
        let sb: StringBuilder = StringBuilder::new(sz*5+2);
        sb.append_char('{');
        for(        let i: i32 = 0;;i<szi += 1)        {
            if i!=0            {
                sb.append_String(", ");
            }            
            sb.append_String(list.get(i).regString());
        }
        sb.append_char('}');
        return sb.toString();
    }
    pub fn regRangeString(list: &RegisterSpecList) -> String    {
        let size: i32 = list.size();
        let sb: StringBuilder = StringBuilder::new(30);
        sb.append_String("{");
        match size{0 =>             {
                break;
            }1 =>             {
                sb.append_String(list.get(0).regString());
                break;
            }        _ => {}        {
            let lastReg: RegisterSpec = list.get(size-1);
            if lastReg.getCategory()==2            {
                lastReg=lastReg.withOffset(1);
            }            
            sb.append_String(list.get(0).regString());
            sb.append_String("..");
            sb.append_String(lastReg.regString());
        }    }
    sb.append_String("}");
    return sb.toString();
}
pub fn literalBitsString(value: &CstLiteralBits) -> String{
    let sb: StringBuilder = StringBuilder::new(100);
    sb.append_char('#');
    if //value instanceof CstKnownNull    {
        sb.append_String("null");
    }    else     {
        sb.append_String(value.typeName());
        sb.append_char(' ');
        sb.append_String(value.toHuman());
    }
    return sb.toString();
}
pub fn literalBitsComment(value: &CstLiteralBits, width: i32) -> String{
    let sb: StringBuilder = StringBuilder::new(20);
    sb.append_String("#");
    let bits: i64;
    if //value instanceof CstLiteral64    {
        bits=((CstLiteral64*)value).getLongBits();
    }    else     {
        bits=value.getIntBits();
    }
    match width{4 =>         sb.append_String(Hex::uNibble(bits as i32));        break;8 =>         sb.append_String(Hex::u1(bits as i32));        break;16 =>         sb.append_String(Hex::u2(bits as i32));        break;32 =>         sb.append_String(Hex::u4(bits as i32));        break;64 =>         sb.append_String(Hex::u8(bits));        break;    _ => {}    {
        throw RuntimeException::new("shouldn't happen");
    }}
return sb.toString();
}
pub fn branchString(insn: &DalvInsn) -> String{
let ti: TargetInsn = (TargetInsn*)insn;
let address: i32 = ti.getTargetAddress();
return if (address==address as char) { Hex::u2(address) } else { Hex::u4(address) };
    }
    pub fn branchComment(insn: &DalvInsn) -> String    {
        let ti: TargetInsn = (TargetInsn*)insn;
        let offset: i32 = ti.getTargetOffset();
        return if (offset==offset as i16) { Hex::s2(offset) } else { Hex::s4(offset) };
            }
            pub fn signedFitsInNibble(value: i32) -> boolean            {
                return (value>=-8)&&(value<=7);
            }
            pub fn unsignedFitsInNibble(value: i32) -> boolean            {
                return value==(value&0xf);
            }
            pub fn signedFitsInByte(value: i32) -> boolean            {
                return value as i8==value;
            }
            pub fn unsignedFitsInByte(value: i32) -> boolean            {
                return value==(value&0xff);
            }
            pub fn signedFitsInShort(value: i32) -> boolean            {
                return value as i16==value;
            }
            pub fn unsignedFitsInShort(value: i32) -> boolean            {
                return value==(value&0xffff);
            }
            pub fn isRegListSequential(list: &RegisterSpecList) -> boolean            {
                let sz: i32 = list.size();
                if sz<2                {
                    return true;
                }                
                let first: i32 = list.get(0).getReg();
                let next: i32 = first;
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let one: RegisterSpec = list.get(i);
                    if one.getReg()!=next                    {
                        return false;
                    }                    
                    next+=one.getCategory();
                }
                return true;
            }
            pub fn argIndex(insn: &DalvInsn) -> i32            {
                let arg: i32 = ((CstInteger*)((CstInsn*)insn).getConstant()).getValue();
                if arg<0                {
                    throw IllegalArgumentException::new("bogus insn");
                }                
                return arg;
            }
            pub fn opcodeUnit(insn: &DalvInsn, arg: i32) -> i16            {
                if (arg&0xff)!=arg                {
                    throw IllegalArgumentException::new("arg out of range 0..255");
                }                
                let opcode: i32 = insn.getOpcode().getOpcode();
                if (opcode&0xff)!=opcode                {
                    throw IllegalArgumentException::new("opcode out of range 0..255");
                }                
                return (opcode|(arg<<8)) as i16;
            }
            pub fn opcodeUnit(insn: &DalvInsn) -> i16            {
                let opcode: i32 = insn.getOpcode().getOpcode();
                if (opcode<0x100)||(opcode>0xffff)                {
                    throw IllegalArgumentException::new("opcode out of range 0..65535");
                }                
                return opcode as i16;
            }
            pub fn codeUnit(low: i32, high: i32) -> i16            {
                if (low&0xff)!=low                {
                    throw IllegalArgumentException::new("low out of range 0..255");
                }                
                if (high&0xff)!=high                {
                    throw IllegalArgumentException::new("high out of range 0..255");
                }                
                return (low|(high<<8)) as i16;
            }
            pub fn codeUnit(n0: i32, n1: i32, n2: i32, n3: i32) -> i16            {
                if (n0&0xf)!=n0                {
                    throw IllegalArgumentException::new("n0 out of range 0..15");
                }                
                if (n1&0xf)!=n1                {
                    throw IllegalArgumentException::new("n1 out of range 0..15");
                }                
                if (n2&0xf)!=n2                {
                    throw IllegalArgumentException::new("n2 out of range 0..15");
                }                
                if (n3&0xf)!=n3                {
                    throw IllegalArgumentException::new("n3 out of range 0..15");
                }                
                return (n0|(n1<<4)|(n2<<8)|(n3<<12)) as i16;
            }
            pub fn makeByte(low: i32, high: i32) -> i32            {
                if (low&0xf)!=low                {
                    throw IllegalArgumentException::new("low out of range 0..15");
                }                
                if (high&0xf)!=high                {
                    throw IllegalArgumentException::new("high out of range 0..15");
                }                
                return low|(high<<4);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16)            {
                out.writeShort(c0);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1: i16)            {
                out.writeShort(c0);
                out.writeShort(c1);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1: i16, c2: i16)            {
                out.writeShort(c0);
                out.writeShort(c1);
                out.writeShort(c2);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1: i16, c2: i16, c3: i16)            {
                out.writeShort(c0);
                out.writeShort(c1);
                out.writeShort(c2);
                out.writeShort(c3);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1: i16, c2: i16, c3: i16, c4: i16)            {
                out.writeShort(c0);
                out.writeShort(c1);
                out.writeShort(c2);
                out.writeShort(c3);
                out.writeShort(c4);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1c2: i32)            {
                InsnFormat::write_AnnotatedOutput_short_short_short(&out, c0, c1c2 as i16, (c1c2>>16) as i16);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1c2: i32, c3: i16)            {
                InsnFormat::write_AnnotatedOutput_short_short_short_short(&out, c0, c1c2 as i16, (c1c2>>16) as i16, c3);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1c2: i32, c3: i16, c4: i16)            {
                InsnFormat::write_AnnotatedOutput_short_short_short_short_short(&out, c0, c1c2 as i16, (c1c2>>16) as i16, c3, c4);
            }
            pub fn write(out: &AnnotatedOutput, c0: i16, c1c2c3c4: i64)            {
                InsnFormat::write_AnnotatedOutput_short_short_short_short_short(&out, c0, c1c2c3c4 as i16, (c1c2c3c4>>16) as i16, (c1c2c3c4>>32) as i16, (c1c2c3c4>>48) as i16);
            }
}
