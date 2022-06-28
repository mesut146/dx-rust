use crate::helper;

trait ConstantPool{
    pub fn size(&self) -> i32;
    pub fn get(&self, n: i32) -> Constant;
    pub fn get0Ok(&self, n: i32) -> Constant;
    pub fn getOrNull(&self, n: i32) -> Constant;
    pub fn getEntries(&self) -> Vec<Constant>;
}
