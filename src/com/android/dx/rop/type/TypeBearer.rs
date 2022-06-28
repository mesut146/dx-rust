use crate::helper;

trait TypeBearer{
    pub fn getType(&self) -> Type;
    pub fn getFrameType(&self) -> TypeBearer;
    pub fn getBasicType(&self) -> i32;
    pub fn getBasicFrameType(&self) -> i32;
    pub fn isConstant(&self) -> boolean;
}
