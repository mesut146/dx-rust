use crate::helper;
use crate::com::android::dex::util::ByteOutput;
use crate::com::android::dex::util::ByteInput;

struct EncodedValueCodec{
}
impl EncodedValueCodec{
    pub fn new(&self)    {
    }
    pub fn writeSignedIntegralValue(out: &ByteOutput, type: i32, value: i64)    {
        let requiredBits: i32 = 65-Long::numberOfLeadingZeros(value^(value>>63));
        let requiredBytes: i32 = (requiredBits+0x07)>>3;
        out.writeByte(type|((requiredBytes-1)<<5));
        while requiredBytes>0        {
            out.writeByte(value as i8);
            value>>=8;
            requiredBytes -= 1;
        }
    }
    pub fn writeUnsignedIntegralValue(out: &ByteOutput, type: i32, value: i64)    {
        let requiredBits: i32 = 64-Long::numberOfLeadingZeros(value);
        if requiredBits==0        {
            requiredBits=1;
        }        
        let requiredBytes: i32 = (requiredBits+0x07)>>3;
        out.writeByte(type|((requiredBytes-1)<<5));
        while requiredBytes>0        {
            out.writeByte(value as i8);
            value>>=8;
            requiredBytes -= 1;
        }
    }
    pub fn writeRightZeroExtendedValue(out: &ByteOutput, type: i32, value: i64)    {
        let requiredBits: i32 = 64-Long::numberOfTrailingZeros(value);
        if requiredBits==0        {
            requiredBits=1;
        }        
        let requiredBytes: i32 = (requiredBits+0x07)>>3;
        value>>=64-(requiredBytes*8);
        out.writeByte(type|((requiredBytes-1)<<5));
        while requiredBytes>0        {
            out.writeByte(value as i8);
            value>>=8;
            requiredBytes -= 1;
        }
    }
    pub fn readSignedInt(in: &ByteInput, zwidth: i32) -> i32    {
        let result: i32 = 0;
        for(        let i: i32 = zwidth;;i>=0i -= 1)        {
            result=(result>>>8)|((in.readByte()&0xff)<<24);
        }
        result>>=(3-zwidth)*8;
        return result;
    }
    pub fn readUnsignedInt(in: &ByteInput, zwidth: i32, fillOnRight: boolean) -> i32    {
        let result: i32 = 0;
        if !fillOnRight        {
            for(            let i: i32 = zwidth;;i>=0i -= 1)            {
                result=(result>>>8)|((in.readByte()&0xff)<<24);
            }
            result>>>=(3-zwidth)*8;
        }        else         {
            for(            let i: i32 = zwidth;;i>=0i -= 1)            {
                result=(result>>>8)|((in.readByte()&0xff)<<24);
            }
        }
        return result;
    }
    pub fn readSignedLong(in: &ByteInput, zwidth: i32) -> i64    {
        let result: i64 = 0;
        for(        let i: i32 = zwidth;;i>=0i -= 1)        {
            result=(result>>>8)|((in.readByte()&0xffL)<<56);
        }
        result>>=(7-zwidth)*8;
        return result;
    }
    pub fn readUnsignedLong(in: &ByteInput, zwidth: i32, fillOnRight: boolean) -> i64    {
        let result: i64 = 0;
        if !fillOnRight        {
            for(            let i: i32 = zwidth;;i>=0i -= 1)            {
                result=(result>>>8)|((in.readByte()&0xffL)<<56);
            }
            result>>>=(7-zwidth)*8;
        }        else         {
            for(            let i: i32 = zwidth;;i>=0i -= 1)            {
                result=(result>>>8)|((in.readByte()&0xffL)<<56);
            }
        }
        return result;
    }
}
