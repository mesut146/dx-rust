use crate::helper;
use crate::com::android::dx::rop::cst::CstString;

let static ATTRIBUTE_NAME: String = "SourceDebugExtension";
struct AttSourceDebugExtension{
    pub smapString: CstString,
}
impl AttSourceDebugExtension{
    pub fn new(&self, smapString: &CstString)    {
        super(ATTRIBUTE_NAME);

        if smapString==None        {
            throw NullPointerException::new("smapString == null");
        }        
        self->smapString=smapString;
    }
    pub fn byteLength(&self) -> i32    {
        return 6+self.smapString.getUtf8Size();
    }
    pub fn getSmapString(&self) -> CstString    {
        return self.smapString;
    }
}
