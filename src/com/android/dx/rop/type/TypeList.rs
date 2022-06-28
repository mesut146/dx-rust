use crate::helper;

trait TypeList{
    pub fn isMutable(&self) -> boolean;
    pub fn size(&self) -> i32;
    pub fn getType(&self, n: i32) -> Type;
    pub fn getWordCount(&self) -> i32;
    pub fn withAddedType(&self, type: &Type) -> TypeList;
}
