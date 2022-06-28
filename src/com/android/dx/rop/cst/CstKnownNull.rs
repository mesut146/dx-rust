use crate::helper;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::rop::type::Type;

let static THE_ONE: CstKnownNull = CstKnownNull::new();
struct CstKnownNull{
}
impl CstKnownNull{
    pub fn new(&self)    {
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        return (//other instanceof CstKnownNull);
    }
    pub fn hashCode(&self) -> i32    {
        return 0x4466757a;
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        return 0;
    }
    pub fn toString(&self) -> String    {
        return "known-null";
    }
    pub fn getType(&self) -> Type    {
        return Type::KNOWN_NULL;
    }
    pub fn typeName(&self) -> String    {
        return "known-null";
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn toHuman(&self) -> String    {
        return "null";
    }
    pub fn fitsInInt(&self) -> boolean    {
        return true;
    }
    pub fn getIntBits(&self) -> i32    {
        return 0;
    }
    pub fn getLongBits(&self) -> i64    {
        return 0;
    }
}
