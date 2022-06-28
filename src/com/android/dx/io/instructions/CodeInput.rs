use crate::helper;

trait CodeInput{
    pub fn hasMore(&self) -> boolean;
    pub fn read(&self) -> i32;
    pub fn readInt(&self) -> i32;
    pub fn readLong(&self) -> i64;
}
