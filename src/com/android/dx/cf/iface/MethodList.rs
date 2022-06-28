use crate::helper;

trait MethodList{
    pub fn isMutable(&self) -> boolean;
    pub fn size(&self) -> i32;
    pub fn get(&self, n: i32) -> Method;
}
