use crate::helper;
use crate::com::android::dx::rop::cst::CstShort;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

let static VALUE_0: CstShort = CstShort::make_short(0 as i16);
struct CstShort{
}
impl CstShort{
    pub fn make(value: i16) -> CstShort    {
        return CstShort::new(value);
    }
    pub fn make(value: i32) -> CstShort    {
        let cast: i16 = value as i16;
        if cast!=value        {
            throw IllegalArgumentException::new("bogus short value: "+value);
        }        
        return CstShort::make_short(cast);
    }
    pub fn new(&self, value: i16)    {
        super(value);

    }
    pub fn toString(&self) -> String    {
        let value: i32 = getIntBits();
        return "short{0x"+Hex::u2(value)+" / "+value+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::SHORT;
    }
    pub fn typeName(&self) -> String    {
        return "short";
    }
    pub fn toHuman(&self) -> String    {
        return Integer::toString_int(getIntBits());
    }
    pub fn getValue(&self) -> i16    {
        return getIntBits() as i16;
    }
}
