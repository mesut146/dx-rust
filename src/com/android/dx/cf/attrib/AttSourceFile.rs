use crate::helper;
use crate::com::android::dx::rop::cst::CstString;

let static ATTRIBUTE_NAME: String = "SourceFile";
struct AttSourceFile{
    pub sourceFile: CstString,
}
impl AttSourceFile{
    pub fn new(&self, sourceFile: &CstString)    {
        super(ATTRIBUTE_NAME);

        if sourceFile==None        {
            throw NullPointerException::new("sourceFile == null");
        }        
        self->sourceFile=sourceFile;
    }
    pub fn byteLength(&self) -> i32    {
        return 8;
    }
    pub fn getSourceFile(&self) -> CstString    {
        return self.sourceFile;
    }
}
