use crate::helper;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::rop::cst::CstCallSite;

struct CstCallSiteRef{
    pub invokeDynamic: CstInvokeDynamic,
    pub id: i32,
}
impl CstCallSiteRef{
    pub fn new(&self, invokeDynamic: &CstInvokeDynamic, id: i32)    {
        if invokeDynamic==None        {
            throw NullPointerException::new("invokeDynamic == null");
        }        
        self->invokeDynamic=invokeDynamic;
        self->id=id;
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn typeName(&self) -> String    {
        return "CallSiteRef";
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let o: CstCallSiteRef = (CstCallSiteRef*)other;
        let result: i32 = self.invokeDynamic.compareTo(&);
        if result!=0        {
            return result;
        }        
        return Integer::compare(self.id, );
    }
    pub fn toHuman(&self) -> String    {
        return getCallSite().toHuman();
    }
    pub fn toString(&self) -> String    {
        return getCallSite().toString();
    }
    pub fn getPrototype(&self) -> Prototype    {
        return self.invokeDynamic.getPrototype();
    }
    pub fn getReturnType(&self) -> Type    {
        return self.invokeDynamic.getReturnType();
    }
    pub fn getCallSite(&self) -> CstCallSite    {
        return self.invokeDynamic.getCallSite();
    }
}
