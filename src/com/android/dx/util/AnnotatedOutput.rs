use crate::helper;

trait AnnotatedOutput{
    pub fn annotates(&self) -> boolean;
    pub fn isVerbose(&self) -> boolean;
    pub fn annotate(&self, msg: &String);
    pub fn annotate(&self, amt: i32, msg: &String);
    pub fn endAnnotation(&self);
    pub fn getAnnotationWidth(&self) -> i32;
}
