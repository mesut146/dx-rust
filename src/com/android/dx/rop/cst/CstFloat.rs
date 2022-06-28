use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

let static VALUE_0: CstFloat = CstFloat::make(Float::floatToIntBits(0.0f));
let static VALUE_1: CstFloat = CstFloat::make(Float::floatToIntBits(1.0f));
let static VALUE_2: CstFloat = CstFloat::make(Float::floatToIntBits(2.0f));
struct CstFloat{
}
impl CstFloat{
    pub fn make(bits: i32) -> CstFloat    {
        return CstFloat::new(bits);
    }
    pub fn new(&self, bits: i32)    {
        super(bits);

    }
    pub fn toString(&self) -> String    {
        let bits: i32 = getIntBits();
        return "float{0x"+Hex::u4(bits)+" / "+Float::intBitsToFloat(bits)+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::FLOAT;
    }
    pub fn typeName(&self) -> String    {
        return "float";
    }
    pub fn toHuman(&self) -> String    {
        return Float::toString_float(Float::intBitsToFloat(getIntBits()));
    }
    pub fn getValue(&self) -> f32    {
        return Float::intBitsToFloat(getIntBits());
    }
}
