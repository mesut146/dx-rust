use crate::helper;
use crate::com::android::dex::EncodedValueCodec;
use crate::com::android::dex::util::ByteInput;
use crate::com::android::dex::Leb128;
use crate::com::android::dex::EncodedValue;
use crate::com::android::dex::DexException;

struct EncodedValueReader{
    pub in: ByteInput,
    pub type: i32,
    pub annotationType: i32,
    pub arg: i32,
}
impl EncodedValueReader{
    pub const ENCODED_BYTE: i32 = 0x00;
    pub const ENCODED_SHORT: i32 = 0x02;
    pub const ENCODED_CHAR: i32 = 0x03;
    pub const ENCODED_INT: i32 = 0x04;
    pub const ENCODED_LONG: i32 = 0x06;
    pub const ENCODED_FLOAT: i32 = 0x10;
    pub const ENCODED_DOUBLE: i32 = 0x11;
    pub const ENCODED_METHOD_TYPE: i32 = 0x15;
    pub const ENCODED_METHOD_HANDLE: i32 = 0x16;
    pub const ENCODED_STRING: i32 = 0x17;
    pub const ENCODED_TYPE: i32 = 0x18;
    pub const ENCODED_FIELD: i32 = 0x19;
    pub const ENCODED_ENUM: i32 = 0x1b;
    pub const ENCODED_METHOD: i32 = 0x1a;
    pub const ENCODED_ARRAY: i32 = 0x1c;
    pub const ENCODED_ANNOTATION: i32 = 0x1d;
    pub const ENCODED_NULL: i32 = 0x1e;
    pub const ENCODED_BOOLEAN: i32 = 0x1f;
    pub const MUST_READ: i32 = -1;
    pub fn new(&self, in: &ByteInput)    {
        self->in=in;
    }
    pub fn new(&self, in: &EncodedValue)    {
        this(in.asByteInput());

    }
    pub fn new(&self, in: &ByteInput, knownType: i32)    {
        self->in=in;
        self->type=knownType;
    }
    pub fn new(&self, in: &EncodedValue, knownType: i32)    {
        this(in.asByteInput(),knownType);

    }
    pub fn peek(&self) -> i32    {
        if self.type_renamed==EncodedValueReader::MUST_READ        {
            let argAndType: i32 = self.in_renamed.readByte()&0xff;
            self.type_renamed=argAndType&0x1f;
            self.arg=(argAndType&0xe0)>>5;
        }        
        return self.type_renamed;
    }
    pub fn readArray(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_ARRAY);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return Leb128::readUnsignedLeb128(&self.in_renamed);
    }
    pub fn readAnnotation(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_ANNOTATION);
        self.type_renamed=EncodedValueReader::MUST_READ;
        self.annotationType=Leb128::readUnsignedLeb128(&self.in_renamed);
        return Leb128::readUnsignedLeb128(&self.in_renamed);
    }
    pub fn getAnnotationType(&self) -> i32    {
        return self.annotationType;
    }
    pub fn readAnnotationName(&self) -> i32    {
        return Leb128::readUnsignedLeb128(&self.in_renamed);
    }
    pub fn readByte(&self) -> i8    {
        checkType(EncodedValueReader::ENCODED_BYTE);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readSignedInt(&self.in_renamed, self.arg) as i8;
    }
    pub fn readShort(&self) -> i16    {
        checkType(EncodedValueReader::ENCODED_SHORT);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readSignedInt(&self.in_renamed, self.arg) as i16;
    }
    pub fn readChar(&self) -> char    {
        checkType(EncodedValueReader::ENCODED_CHAR);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false) as char;
    }
    pub fn readInt(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_INT);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readSignedInt(&self.in_renamed, self.arg);
    }
    pub fn readLong(&self) -> i64    {
        checkType(EncodedValueReader::ENCODED_LONG);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readSignedLong(&self.in_renamed, self.arg);
    }
    pub fn readFloat(&self) -> f32    {
        checkType(EncodedValueReader::ENCODED_FLOAT);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return Float::intBitsToFloat(EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, true));
    }
    pub fn readDouble(&self) -> f64    {
        checkType(EncodedValueReader::ENCODED_DOUBLE);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return Double::longBitsToDouble(EncodedValueCodec::readUnsignedLong(&self.in_renamed, self.arg, true));
    }
    pub fn readMethodType(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_METHOD_TYPE);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readMethodHandle(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_METHOD_HANDLE);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readString(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_STRING);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readType(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_TYPE);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readField(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_FIELD);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readEnum(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_ENUM);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readMethod(&self) -> i32    {
        checkType(EncodedValueReader::ENCODED_METHOD);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return EncodedValueCodec::readUnsignedInt(&self.in_renamed, self.arg, false);
    }
    pub fn readNull(&self)    {
        checkType(EncodedValueReader::ENCODED_NULL);
        self.type_renamed=EncodedValueReader::MUST_READ;
    }
    pub fn readBoolean(&self) -> boolean    {
        checkType(EncodedValueReader::ENCODED_BOOLEAN);
        self.type_renamed=EncodedValueReader::MUST_READ;
        return self.arg!=0;
    }
    pub fn skipValue(&self)    {
        match peek(){EncodedValueReader::ENCODED_BYTE =>             readByte();            break;EncodedValueReader::ENCODED_SHORT =>             readShort();            break;EncodedValueReader::ENCODED_CHAR =>             readChar();            break;EncodedValueReader::ENCODED_INT =>             readInt();            break;EncodedValueReader::ENCODED_LONG =>             readLong();            break;EncodedValueReader::ENCODED_FLOAT =>             readFloat();            break;EncodedValueReader::ENCODED_DOUBLE =>             readDouble();            break;EncodedValueReader::ENCODED_METHOD_TYPE =>             readMethodType();            break;EncodedValueReader::ENCODED_METHOD_HANDLE =>             readMethodHandle();            break;EncodedValueReader::ENCODED_STRING =>             readString();            break;EncodedValueReader::ENCODED_TYPE =>             readType();            break;EncodedValueReader::ENCODED_FIELD =>             readField();            break;EncodedValueReader::ENCODED_ENUM =>             readEnum();            break;EncodedValueReader::ENCODED_METHOD =>             readMethod();            break;EncodedValueReader::ENCODED_ARRAY =>             for(            let i: i32 = 0;            let size: i32 = readArray();;i<sizei += 1)            {
                skipValue();
            }            break;EncodedValueReader::ENCODED_ANNOTATION =>             for(            let i: i32 = 0;            let size: i32 = readAnnotation();;i<sizei += 1)            {
                readAnnotationName();
                skipValue();
            }            break;EncodedValueReader::ENCODED_NULL =>             readNull();            break;EncodedValueReader::ENCODED_BOOLEAN =>             readBoolean();            break;        _ => {}        throw DexException::new("Unexpected type: "+Integer::toHexString(self.type_renamed));    }
}
pub fn checkType(&self, expected: i32){
    if peek()!=expected    {
        throw IllegalStateException::new(String::format_String_Object[]("Expected %x but was %x", expected, peek()));
    }    
}
}
