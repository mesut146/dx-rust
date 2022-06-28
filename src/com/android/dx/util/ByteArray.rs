use crate::helper;
use crate::com::android::dx::util::ByteArray::MyInputStream;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::ByteArray::MyDataInputStream;

struct ByteArray{
    pub bytes: Vec<i8>,
    pub start: i32,
    pub size: i32,
}
impl ByteArray{
    pub fn new(&self, bytes: &Vec<i8>, start: i32, end: i32)    {
        if bytes==None        {
            throw NullPointerException::new("bytes == null");
        }        
        if start<0        {
            throw IllegalArgumentException::new("start < 0");
        }        
        if end<start        {
            throw IllegalArgumentException::new("end < start");
        }        
        if end>bytes.len()        {
            throw IllegalArgumentException::new("end > bytes.length");
        }        
        self->bytes=bytes;
        self->start=start;
        self->size=end-start;
    }
    pub fn new(&self, bytes: &Vec<i8>)    {
        this(bytes,0,bytes.length);

    }
    pub fn size(&self) -> i32    {
        return self.size;
    }
    pub fn slice(&self, start: i32, end: i32) -> ByteArray    {
        checkOffsets(start, end);
        let slicedOut: Vec<i8> = Arrays::copyOfRange_byte[]_int_int(&self.bytes, start, end);
        return ByteArray::new(&slicedOut);
    }
    pub fn underlyingOffset(&self, offset: i32) -> i32    {
        return self.start+offset;
    }
    pub fn getByte(&self, off: i32) -> i32    {
        checkOffsets(off, off+1);
        return getByte0(off);
    }
    pub fn getShort(&self, off: i32) -> i32    {
        checkOffsets(off, off+2);
        return (getByte0(off)<<8)|getUnsignedByte0(off+1);
    }
    pub fn getInt(&self, off: i32) -> i32    {
        checkOffsets(off, off+4);
        return (getByte0(off)<<24)|(getUnsignedByte0(off+1)<<16)|(getUnsignedByte0(off+2)<<8)|getUnsignedByte0(off+3);
    }
    pub fn getLong(&self, off: i32) -> i64    {
        checkOffsets(off, off+8);
        let part1: i32 = (getByte0(off)<<24)|(getUnsignedByte0(off+1)<<16)|(getUnsignedByte0(off+2)<<8)|getUnsignedByte0(off+3);
        let part2: i32 = (getByte0(off+4)<<24)|(getUnsignedByte0(off+5)<<16)|(getUnsignedByte0(off+6)<<8)|getUnsignedByte0(off+7);
        return (part2&0xffffffffL)|(part1 as i64)<<32;
    }
    pub fn getUnsignedByte(&self, off: i32) -> i32    {
        checkOffsets(off, off+1);
        return getUnsignedByte0(off);
    }
    pub fn getUnsignedShort(&self, off: i32) -> i32    {
        checkOffsets(off, off+2);
        return (getUnsignedByte0(off)<<8)|getUnsignedByte0(off+1);
    }
    pub fn getBytes(&self, out: &Vec<i8>, offset: i32)    {
        if (out.len()-offset)<self.size        {
            throw IndexOutOfBoundsException::new("(out.length - offset) < "+"size()");
        }        
        System::arraycopy(&self.bytes, self.start, &out, offset, self.size);
    }
    pub fn checkOffsets(&self, s: i32, e: i32)    {
        if (s<0)||(e<s)||(e>self.size)        {
            throw IllegalArgumentException::new("bad range: "+s+".."+e+"; actual size "+self.size);
        }        
    }
    pub fn getByte0(&self, off: i32) -> i32    {
        return self.bytes[self.start+off];
    }
    pub fn getUnsignedByte0(&self, off: i32) -> i32    {
        return self.bytes[self.start+off]&0xff;
    }
    pub fn makeDataInputStream(&self) -> MyDataInputStream    {
        return MyDataInputStream::new(makeInputStream());
    }
    pub fn makeInputStream(&self) -> MyInputStream    {
        return MyInputStream::new(, self);
    }
}
