use crate::helper;
use crate::com::android::dex::util::ByteOutput;
use crate::com::android::dex::DexException;
use crate::com::android::dex::util::ByteInput;

struct Leb128{
}
impl Leb128{
    pub fn new(&self)    {
    }
    pub fn unsignedLeb128Size(value: i32) -> i32    {
        let remaining: i32 = value>>7;
        let count: i32 = 0;
        while remaining!=0        {
            remaining>>=7;
            count += 1;
        }
        return count+1;
    }
    pub fn readSignedLeb128(in: &ByteInput) -> i32    {
        let result: i32 = 0;
        let cur: i32;
        let count: i32 = 0;
        let signBits: i32 = -1;
        loop        {
            cur=in.readByte()&0xff;            result|=(cur&0x7f)<<(count*7);            signBits<<=7;            count += 1;            if !((cur&0x80)==0x80)&&count<5{ break; }        }
        if (cur&0x80)==0x80        {
            throw DexException::new("invalid LEB128 sequence");
        }        
        if ((signBits>>1)&result)!=0        {
            result|=signBits;
        }        
        return result;
    }
    pub fn readUnsignedLeb128(in: &ByteInput) -> i32    {
        let result: i32 = 0;
        let cur: i32;
        let count: i32 = 0;
        loop        {
            cur=in.readByte()&0xff;            result|=(cur&0x7f)<<(count*7);            count += 1;            if !((cur&0x80)==0x80)&&count<5{ break; }        }
        if (cur&0x80)==0x80        {
            throw DexException::new("invalid LEB128 sequence");
        }        
        return result;
    }
    pub fn writeUnsignedLeb128(out: &ByteOutput, value: i32)    {
        let remaining: i32 = value>>>7;
        while remaining!=0        {
            out.writeByte(((value&0x7f)|0x80) as i8);
            value=remaining;
            remaining>>>=7;
        }
        out.writeByte((value&0x7f) as i8);
    }
    pub fn writeSignedLeb128(out: &ByteOutput, value: i32)    {
        let remaining: i32 = value>>7;
        let hasMore: boolean = true;
        let end: i32 = if ((value&Integer::MIN_VALUE)==0) { 0 } else { -1 };
                while hasMore                {
                    hasMore=(remaining!=end)||((remaining&1)!=((value>>6)&1));
                    out.writeByte(((value&0x7f)|(if hasMore { 0x80 } else { 0 })) as i8);
                            value=remaining;
                            remaining>>=7;
                        }
                    }
}
