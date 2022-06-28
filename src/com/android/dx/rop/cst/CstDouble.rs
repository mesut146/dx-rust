use crate::helper;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstDouble;

let static VALUE_0: CstDouble = CstDouble::new(Double::doubleToLongBits(0.0));
let static VALUE_1: CstDouble = CstDouble::new(Double::doubleToLongBits(1.0));
struct CstDouble{
}
impl CstDouble{
    pub fn make(bits: i64) -> CstDouble    {
        return CstDouble::new(bits);
    }
    pub fn new(&self, bits: i64)    {
        super(bits);

    }
    pub fn toString(&self) -> String    {
        let bits: i64 = getLongBits();
        return "double{0x"+Hex::u8(bits)+" / "+Double::longBitsToDouble(bits)+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::DOUBLE;
    }
    pub fn typeName(&self) -> String    {
        return "double";
    }
    pub fn toHuman(&self) -> String    {
        return Double::toString_double(Double::longBitsToDouble(getLongBits()));
    }
    pub fn getValue(&self) -> f64    {
        return Double::longBitsToDouble(getLongBits());
    }
}
