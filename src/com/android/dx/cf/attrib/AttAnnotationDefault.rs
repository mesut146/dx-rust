use crate::helper;
use crate::com::android::dx::rop::cst::Constant;

let static ATTRIBUTE_NAME: String = "AnnotationDefault";
struct AttAnnotationDefault{
    pub value: Constant,
    pub byteLength: i32,
}
impl AttAnnotationDefault{
    pub fn new(&self, value: &Constant, byteLength: i32)    {
        super(ATTRIBUTE_NAME);

        if value==None        {
            throw NullPointerException::new("value == null");
        }        
        self->value=value;
        self->byteLength=byteLength;
    }
    pub fn byteLength(&self) -> i32    {
        return self.byteLength+6;
    }
    pub fn getValue(&self) -> Constant    {
        return self.value;
    }
}
