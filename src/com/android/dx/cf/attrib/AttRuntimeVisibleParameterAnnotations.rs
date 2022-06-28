use crate::helper;

let static ATTRIBUTE_NAME: String = "RuntimeVisibleParameterAnnotations";
struct AttRuntimeVisibleParameterAnnotations{
}
impl AttRuntimeVisibleParameterAnnotations{
    pub fn new(&self, annotations: &AnnotationsList, byteLength: i32)    {
        super(ATTRIBUTE_NAME,annotations,byteLength);

    }
}
