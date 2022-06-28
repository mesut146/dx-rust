use crate::helper;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::rop::cst::CstNat;

struct CstEnumRef{
    pub fieldRef: CstFieldRef,
}
impl CstEnumRef{
    pub fn new(&self, nat: &CstNat)    {
        super(new CstType(nat.getFieldType()),nat);

        self.fieldRef=None;
    }
    pub fn typeName(&self) -> String    {
        return "enum";
    }
    pub fn getType(&self) -> Type    {
        return getDefiningClass().getClassType();
    }
    pub fn getFieldRef(&self) -> CstFieldRef    {
        if self.fieldRef==None        {
            self.fieldRef=CstFieldRef::new(getDefiningClass(), getNat());
        }        
        return self.fieldRef;
    }
}
