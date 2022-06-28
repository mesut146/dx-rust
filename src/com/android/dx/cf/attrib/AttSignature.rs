use crate::helper;
use crate::com::android::dx::rop::cst::CstString;

let static ATTRIBUTE_NAME: String = "Signature";
struct AttSignature{
    pub signature: CstString,
}
impl AttSignature{
    pub fn new(&self, signature: &CstString)    {
        super(ATTRIBUTE_NAME);

        if signature==None        {
            throw NullPointerException::new("signature == null");
        }        
        self->signature=signature;
    }
    pub fn byteLength(&self) -> i32    {
        return 8;
    }
    pub fn getSignature(&self) -> CstString    {
        return self.signature;
    }
}
