use crate::helper;

trait FieldList{
    pub fn isMutable(&self) -> boolean;
    pub fn size(&self) -> i32;
    pub fn get(&self, n: i32) -> Field;
}
