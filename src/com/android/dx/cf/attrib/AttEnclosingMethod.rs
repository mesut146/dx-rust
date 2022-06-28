use crate::helper;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstNat;

let static ATTRIBUTE_NAME: String = "EnclosingMethod";
struct AttEnclosingMethod{
    pub type: CstType,
    pub method: CstNat,
}
impl AttEnclosingMethod{
    pub fn new(&self, type: &CstType, method: &CstNat)    {
        super(ATTRIBUTE_NAME);

        if type==None        {
            throw NullPointerException::new("type == null");
        }        
        self->type=type;
        self->method=method;
    }
    pub fn byteLength(&self) -> i32    {
        return 10;
    }
    pub fn getEnclosingClass(&self) -> CstType    {
        return self.type_renamed;
    }
    pub fn getMethod(&self) -> CstNat    {
        return self.method;
    }
}
