use crate::helper;

let static ATTRIBUTE_NAME: String = "RuntimeInvisibleParameterAnnotations";
struct AttRuntimeInvisibleParameterAnnotations{
}
impl AttRuntimeInvisibleParameterAnnotations{
    pub fn new(&self, parameterAnnotations: &AnnotationsList, byteLength: i32)    {
        super(ATTRIBUTE_NAME,parameterAnnotations,byteLength);

    }
}
