use crate::helper;

trait AttributeList{
    pub fn isMutable(&self) -> boolean;
    pub fn size(&self) -> i32;
    pub fn get(&self, n: i32) -> Attribute;
    pub fn byteLength(&self) -> i32;
    pub fn findFirst(&self, name: &String) -> Attribute;
    pub fn findNext(&self, attrib: &Attribute) -> Attribute;
}
