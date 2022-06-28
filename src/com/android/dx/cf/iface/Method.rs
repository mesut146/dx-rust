use crate::helper;

trait Method{
    pub fn getEffectiveDescriptor(&self) -> Prototype;
}
