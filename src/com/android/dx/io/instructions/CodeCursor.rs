use crate::helper;

trait CodeCursor{
    pub fn cursor(&self) -> i32;
    pub fn baseAddressForCursor(&self) -> i32;
    pub fn setBaseAddress(&self, targetAddress: i32, baseAddress: i32);
}
