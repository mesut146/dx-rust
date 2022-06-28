use crate::helper;
use crate::com::android::dx::rop::cst::CstMethodRef;

struct CstInterfaceMethodRef{
    pub methodRef: CstMethodRef,
}
impl CstInterfaceMethodRef{
    pub fn new(&self, definingClass: &CstType, nat: &CstNat)    {
        super(definingClass,nat);

        self.methodRef=None;
    }
    pub fn typeName(&self) -> String    {
        return "ifaceMethod";
    }
    pub fn toMethodRef(&self) -> CstMethodRef    {
        if self.methodRef==None        {
            self.methodRef=CstMethodRef::new(getDefiningClass(), getNat());
        }        
        return self.methodRef;
    }
}
