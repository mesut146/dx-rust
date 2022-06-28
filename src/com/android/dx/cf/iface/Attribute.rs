use crate::helper;

trait Attribute{
    pub fn getName(&self) -> String;
    pub fn byteLength(&self) -> i32;
}
