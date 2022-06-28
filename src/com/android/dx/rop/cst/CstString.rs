use crate::helper;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

let static EMPTY_STRING: CstString = CstString::new("");
struct CstString{
    pub string: String,
    pub bytes: ByteArray,
}
impl CstString{
    pub fn stringToUtf8Bytes(string: &String) -> Vec<i8>    {
        let len: i32 = string.length();
        let bytes: Vec<i8> = new byte[len * 3];
        let outAt: i32 = 0;
        for(        let i: i32 = 0;;i<leni += 1)        {
            let c: char = string.charAt(i);
            if (c!=0)&&(c<0x80)            {
                bytes[outAt]=c as i8;
                outAt += 1;
            }            else             if c<0x800            {
                bytes[outAt]=(((c>>6)&0x1f)|0xc0) as i8;
                bytes[outAt+1]=((c&0x3f)|0x80) as i8;
                outAt+=2;
            }            else             {
                bytes[outAt]=(((c>>12)&0x0f)|0xe0) as i8;
                bytes[outAt+1]=(((c>>6)&0x3f)|0x80) as i8;
                bytes[outAt+2]=((c&0x3f)|0x80) as i8;
                outAt+=3;
            }
        }
        let result: Vec<i8> = new byte[outAt];
        System::arraycopy(&bytes, 0, &result, 0, outAt);
        return result;
    }
    pub fn utf8BytesToString(bytes: &ByteArray) -> String    {
        let length: i32 = bytes.size();
        let chars: Vec<char> = new char[length];
        let outAt: i32 = 0;
        for(        let at: i32 = 0;;length>0)        {
            let v0: i32 = bytes.getUnsignedByte(at);
            let out: char;
            match v0>>4{0x00 => 0x01 => 0x02 => 0x03 => 0x04 => 0x05 => 0x06 => 0x07 =>                 {
                    length -= 1;
                    if v0==0                    {
                        return CstString::throwBadUtf8(v0, at);
                    }                    
                    out=v0 as char;
                    at += 1;
                    break;
                }0x0c => 0x0d =>                 {
                    length-=2;
                    if length<0                    {
                        return CstString::throwBadUtf8(v0, at);
                    }                    
                    let v1: i32 = bytes.getUnsignedByte(at+1);
                    if (v1&0xc0)!=0x80                    {
                        return CstString::throwBadUtf8(v1, at+1);
                    }                    
                    let value: i32 = ((v0&0x1f)<<6)|(v1&0x3f);
                    if (value!=0)&&(value<0x80)                    {
                        return CstString::throwBadUtf8(v1, at+1);
                    }                    
                    out=value as char;
                    at+=2;
                    break;
                }0x0e =>                 {
                    length-=3;
                    if length<0                    {
                        return CstString::throwBadUtf8(v0, at);
                    }                    
                    let v1: i32 = bytes.getUnsignedByte(at+1);
                    if (v1&0xc0)!=0x80                    {
                        return CstString::throwBadUtf8(v1, at+1);
                    }                    
                    let v2: i32 = bytes.getUnsignedByte(at+2);
                    if (v1&0xc0)!=0x80                    {
                        return CstString::throwBadUtf8(v2, at+2);
                    }                    
                    let value: i32 = ((v0&0x0f)<<12)|((v1&0x3f)<<6)|(v2&0x3f);
                    if value<0x800                    {
                        return CstString::throwBadUtf8(v2, at+2);
                    }                    
                    out=value as char;
                    at+=3;
                    break;
                }            _ => {}            {
                return CstString::throwBadUtf8(v0, at);
            }        }
        chars[outAt]=out;
        outAt += 1;
    }
    return String::new(&chars, 0, outAt);
}
pub fn throwBadUtf8(value: i32, offset: i32) -> String{
    throw IllegalArgumentException::new("bad utf-8 byte "+Hex::u1(value)+" at offset "+Hex::u4(offset));
}
pub fn new(&self, string: &String){
    if string==None    {
        throw NullPointerException::new("string == null");
    }    
    self->string=string.intern();
    self->bytes=ByteArray::new(CstString::stringToUtf8Bytes(&string));
}
pub fn new(&self, bytes: &ByteArray){
    if bytes==None    {
        throw NullPointerException::new("bytes == null");
    }    
    self->bytes=bytes;
    self->string=CstString::utf8BytesToString(&bytes).intern();
}
pub fn equals(&self, other: &Object) -> boolean{
    if !(//other instanceof CstString)    {
        return false;
    }    
    return self.string.equals(((CstString*)other)->string);
}
pub fn hashCode(&self) -> i32{
    return self.string.hashCode();
}
pub fn compareTo0(&self, other: &Constant) -> i32{
    return self.string.compareTo(((CstString*)other)->string);
}
pub fn toString(&self) -> String{
    return "string{\""+toHuman()+"\"}";
}
pub fn typeName(&self) -> String{
    return "utf8";
}
pub fn isCategory2(&self) -> boolean{
    return false;
}
pub fn toHuman(&self) -> String{
    let len: i32 = self.string.length();
    let sb: StringBuilder = StringBuilder::new(len*3/2);
    for(    let i: i32 = 0;;i<leni += 1)    {
        let c: char = self.string.charAt(i);
        if (c>=' ')&&(c<0x7f)        {
            if (c=='\'')||(c=='\"')||(c=='\\')            {
                sb.append_char('\\');
            }            
            sb.append_char(c);
        }        else         if c<=0x7f        {
            match c{'\n' =>                 sb.append_String("\\n");                break;'\r' =>                 sb.append_String("\\r");                break;'\t' =>                 sb.append_String("\\t");                break;            _ => {}            {
                let nextChar: char = if (i<(len-1)) { self.string.charAt(i+1) } else { 0 };
                        let displayZero: boolean = (nextChar>='0')&&(nextChar<='7');
                        sb.append_char('\\');
                        for(                        let shift: i32 = 6;;shift>=0shift-=3)                        {
                            let outChar: char = (((c>>shift)&7)+'0') as char;
                            if (outChar!='0')||displayZero                            {
                                sb.append_char(outChar);
                                displayZero=true;
                            }                            
                        }
                        if !displayZero                        {
                            sb.append_char('0');
                        }                        
                        break;
                    }                }
            }            else             {
                sb.append_String("\\u");
                sb.append_char(Character::forDigit(c>>12, 16));
                sb.append_char(Character::forDigit((c>>8)&0x0f, 16));
                sb.append_char(Character::forDigit((c>>4)&0x0f, 16));
                sb.append_char(Character::forDigit(c&0x0f, 16));
            }
        }
        return sb.toString();
    }
    pub fn toQuoted(&self) -> String    {
        return '\"'+toHuman()+'\"';
    }
    pub fn toQuoted(&self, maxLength: i32) -> String    {
        let string: String = toHuman();
        let length: i32 = string.length();
        let ellipses: String;
        if length<=(maxLength-2)        {
            ellipses="";
        }        else         {
            string=string.substring_int_int(0, maxLength-5);
            ellipses="...";
        }
        return '\"'+string+ellipses+'\"';
    }
    pub fn getString(&self) -> String    {
        return self.string;
    }
    pub fn getBytes(&self) -> ByteArray    {
        return self.bytes;
    }
    pub fn getUtf8Size(&self) -> i32    {
        return self.bytes.size();
    }
    pub fn getUtf16Size(&self) -> i32    {
        return self.string.length();
    }
    pub fn getType(&self) -> Type    {
        return Type::STRING;
    }
}
