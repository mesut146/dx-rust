use crate::helper;
use crate::com::android::dx::rop::cst::Constant;

struct Constant{
}
impl Constant{
    pub fn isCategory2(&self) -> boolean;
    pub fn typeName(&self) -> String;
    pub fn compareTo(&self, other: &Constant) -> i32    {
        let clazz: Class = getClass();
        let otherClazz: Class = other.getClass();
        if clazz!=otherClazz        {
            return clazz.getName().compareTo(otherClazz.getName());
        }        
        return compareTo0(&other);
    }
    pub fn compareTo0(&self, other: &Constant) -> i32;
}
