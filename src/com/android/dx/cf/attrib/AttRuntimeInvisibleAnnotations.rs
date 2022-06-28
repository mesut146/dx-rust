use crate::helper;

let static ATTRIBUTE_NAME: String = "RuntimeInvisibleAnnotations";
struct AttRuntimeInvisibleAnnotations{
}
impl AttRuntimeInvisibleAnnotations{
    pub fn new(&self, annotations: &Annotations, byteLength: i32)    {
        super(ATTRIBUTE_NAME,annotations,byteLength);

    }
}
