use crate::helper;

trait IntSet{
    pub fn add(&self, value: i32);
    pub fn remove(&self, value: i32);
    pub fn has(&self, value: i32) -> boolean;
    pub fn merge(&self, other: &IntSet);
    pub fn elements(&self) -> i32;
    pub fn iterator(&self) -> IntIterator;
}
