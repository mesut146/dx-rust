use crate::helper;

let static ATTRIBUTE_NAME: String = "RuntimeVisibleAnnotations";
struct AttRuntimeVisibleAnnotations{
}
impl AttRuntimeVisibleAnnotations{
    pub fn new(&self, annotations: &Annotations, byteLength: i32)    {
        super(ATTRIBUTE_NAME,annotations,byteLength);

    }
}
