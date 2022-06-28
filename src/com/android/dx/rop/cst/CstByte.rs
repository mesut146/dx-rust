use crate::helper;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstByte;

let static VALUE_0: CstByte = CstByte::make_byte(0 as i8);
struct CstByte{
}
impl CstByte{
    pub fn make(value: i8) -> CstByte    {
        return CstByte::new(value);
    }
    pub fn make(value: i32) -> CstByte    {
        let cast: i8 = value as i8;
        if cast!=value        {
            throw IllegalArgumentException::new("bogus byte value: "+value);
        }        
        return CstByte::make_byte(cast);
    }
    pub fn new(&self, value: i8)    {
        super(value);

    }
    pub fn toString(&self) -> String    {
        let value: i32 = getIntBits();
        return "byte{0x"+Hex::u1(value)+" / "+value+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::BYTE;
    }
    pub fn typeName(&self) -> String    {
        return "byte";
    }
    pub fn toHuman(&self) -> String    {
        return Integer::toString_int(getIntBits());
    }
    pub fn getValue(&self) -> i8    {
        return getIntBits() as i8;
    }
}
