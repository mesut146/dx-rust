use crate::helper;
use crate::com::android::dex::util::ByteInput;

struct Mutf8{
}
impl Mutf8{
    pub fn new(&self)    {
    }
    pub fn decode(in: &ByteInput, out: &Vec<char>) -> String    {
        let s: i32 = 0;
        while true        {
            let a: char = (in.readByte()&0xff) as char;
            if a==0            {
                return String::new(&out, 0, s);
            }            
            out[s]=a;
            if a<'\u0080'            {
                s += 1;
            }            else             if (a&0xe0)==0xc0            {
                let b: i32 = in.readByte()&0xff;
                if (b&0xC0)!=0x80                {
                    throw UTFDataFormatException::new("bad second byte");
                }                
                out[s += 1]=(((a&0x1F)<<6)|(b&0x3F)) as char;
            }            else             if (a&0xf0)==0xe0            {
                let b: i32 = in.readByte()&0xff;
                let c: i32 = in.readByte()&0xff;
                if ((b&0xC0)!=0x80)||((c&0xC0)!=0x80)                {
                    throw UTFDataFormatException::new("bad second or third byte");
                }                
                out[s += 1]=(((a&0x0F)<<12)|((b&0x3F)<<6)|(c&0x3F)) as char;
            }            else             {
                throw UTFDataFormatException::new("bad byte");
            }
        }
    }
    pub fn countBytes(s: &String, shortLength: boolean) -> i64    {
        let result: i64 = 0;
        let length: i32 = s.length();
        for(        let i: i32 = 0;;i<length++i)        {
            let ch: char = s.charAt(i);
            if ch!=0&&ch<=127            {
                ++result;
            }            else             if ch<=2047            {
                result+=2;
            }            else             {
                result+=3;
            }
            if shortLength&&result>65535            {
                throw UTFDataFormatException::new("String more than 65535 UTF bytes long");
            }            
        }
        return result;
    }
    pub fn encode(dst: &Vec<i8>, offset: i32, s: &String)    {
        let length: i32 = s.length();
        for(        let i: i32 = 0;;i<lengthi += 1)        {
            let ch: char = s.charAt(i);
            if ch!=0&&ch<=127            {
                dst[offset += 1]=ch as i8;
            }            else             if ch<=2047            {
                dst[offset += 1]=(0xc0|(0x1f&(ch>>6))) as i8;
                dst[offset += 1]=(0x80|(0x3f&ch)) as i8;
            }            else             {
                dst[offset += 1]=(0xe0|(0x0f&(ch>>12))) as i8;
                dst[offset += 1]=(0x80|(0x3f&(ch>>6))) as i8;
                dst[offset += 1]=(0x80|(0x3f&ch)) as i8;
            }
        }
    }
    pub fn encode(s: &String) -> Vec<i8>    {
        let utfCount: i32 = Mutf8::countBytes(&s, true) as i32;
        let result: Vec<i8> = new byte[utfCount];
        Mutf8::encode_byte[]_int_String(&result, 0, &s);
        return result;
    }
}
