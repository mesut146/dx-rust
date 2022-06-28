use crate::helper;
use crate::com::android::dx::rop::type::Type;

struct TypedConstant{
}
impl TypedConstant{
    pub fn getFrameType(&self) -> TypeBearer    {
        return self;
    }
    pub fn getBasicType(&self) -> i32    {
        return getType().getBasicType();
    }
    pub fn getBasicFrameType(&self) -> i32    {
        return getType().getBasicFrameType();
    }
    pub fn isConstant(&self) -> boolean    {
        return true;
    }
}
