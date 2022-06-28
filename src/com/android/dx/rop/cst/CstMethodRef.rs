use crate::helper;

struct CstMethodRef{
}
impl CstMethodRef{
    pub fn new(&self, definingClass: &CstType, nat: &CstNat)    {
        super(definingClass,nat);

    }
    pub fn typeName(&self) -> String    {
        return "method";
    }
}
