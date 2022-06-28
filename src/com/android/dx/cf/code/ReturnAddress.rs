use crate::helper;
use crate::com::android::dx::cf::code::ReturnAddress;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

struct ReturnAddress{
    pub subroutineAddress: i32,
}
impl ReturnAddress{
    pub fn new(&self, subroutineAddress: i32)    {
        if subroutineAddress<0        {
            throw IllegalArgumentException::new("subroutineAddress < 0");
        }        
        self->subroutineAddress=subroutineAddress;
    }
    pub fn toString(&self) -> String    {
        return ("<addr:"+Hex::u2(self.subroutineAddress)+">");
    }
    pub fn toHuman(&self) -> String    {
        return toString();
    }
    pub fn getType(&self) -> Type    {
        return Type::RETURN_ADDRESS;
    }
    pub fn getFrameType(&self) -> TypeBearer    {
        return self;
    }
    pub fn getBasicType(&self) -> i32    {
        return Type::RETURN_ADDRESS.getBasicType();
    }
    pub fn getBasicFrameType(&self) -> i32    {
        return Type::RETURN_ADDRESS.getBasicFrameType();
    }
    pub fn isConstant(&self) -> boolean    {
        return false;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof ReturnAddress)        {
            return false;
        }        
        return self.subroutineAddress==((ReturnAddress*)other)->subroutineAddress;
    }
    pub fn hashCode(&self) -> i32    {
        return self.subroutineAddress;
    }
    pub fn getSubroutineAddress(&self) -> i32    {
        return self.subroutineAddress;
    }
}
