use crate::helper;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

let static VALUE_0: CstLong = CstLong::make(0);
let static VALUE_1: CstLong = CstLong::make(1);
struct CstLong{
}
impl CstLong{
    pub fn make(value: i64) -> CstLong    {
        return CstLong::new(value);
    }
    pub fn new(&self, value: i64)    {
        super(value);

    }
    pub fn toString(&self) -> String    {
        let value: i64 = getLongBits();
        return "long{0x"+Hex::u8(value)+" / "+value+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::LONG;
    }
    pub fn typeName(&self) -> String    {
        return "long";
    }
    pub fn toHuman(&self) -> String    {
        return Long::toString_long(getLongBits());
    }
    pub fn getValue(&self) -> i64    {
        return getLongBits();
    }
}
