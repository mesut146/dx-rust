use crate::helper;

trait Field{
    pub fn getConstantValue(&self) -> TypedConstant;
}
