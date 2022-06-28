use crate::helper;

trait Output{
    pub fn getCursor(&self) -> i32;
    pub fn assertCursor(&self, expectedCursor: i32);
    pub fn writeByte(&self, value: i32);
    pub fn writeShort(&self, value: i32);
    pub fn writeInt(&self, value: i32);
    pub fn writeLong(&self, value: i64);
    pub fn writeUleb128(&self, value: i32) -> i32;
    pub fn writeSleb128(&self, value: i32) -> i32;
    pub fn write(&self, bytes: &ByteArray);
    pub fn write(&self, bytes: &Vec<i8>, offset: i32, length: i32);
    pub fn write(&self, bytes: &Vec<i8>);
    pub fn writeZeroes(&self, count: i32);
    pub fn alignTo(&self, alignment: i32);
}
