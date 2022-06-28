use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::TypedConstant;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::rop::cst::CstDouble;

let static ATTRIBUTE_NAME: String = "ConstantValue";
struct AttConstantValue{
    pub constantValue: TypedConstant,
}
impl AttConstantValue{
    pub fn new(&self, constantValue: &TypedConstant)    {
        super(ATTRIBUTE_NAME);

        if !((//constantValue instanceof CstString)||(//constantValue instanceof CstInteger)||(//constantValue instanceof CstLong)||(//constantValue instanceof CstFloat)||(//constantValue instanceof CstDouble))        {
            if constantValue==None            {
                throw NullPointerException::new("constantValue == null");
            }            
            throw IllegalArgumentException::new("bad type for constantValue");
        }        
        self->constantValue=constantValue;
    }
    pub fn byteLength(&self) -> i32    {
        return 8;
    }
    pub fn getConstantValue(&self) -> TypedConstant    {
        return self.constantValue;
    }
}
