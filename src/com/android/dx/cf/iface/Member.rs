use crate::helper;

trait Member{
    pub fn getDefiningClass(&self) -> CstType;
    pub fn getAccessFlags(&self) -> i32;
    pub fn getName(&self) -> CstString;
    pub fn getDescriptor(&self) -> CstString;
    pub fn getNat(&self) -> CstNat;
    pub fn getAttributes(&self) -> AttributeList;
}
