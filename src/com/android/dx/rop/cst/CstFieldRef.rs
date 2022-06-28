use crate::helper;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::rop::cst::CstNat;

struct CstFieldRef{
}
impl CstFieldRef{
    pub fn forPrimitiveType(primitiveType: &Type) -> CstFieldRef    {
        return CstFieldRef::new(CstType::forBoxedPrimitiveType(&primitiveType), &CstNat::PRIMITIVE_TYPE_NAT);
    }
    pub fn new(&self, definingClass: &CstType, nat: &CstNat)    {
        super(definingClass,nat);

    }
    pub fn typeName(&self) -> String    {
        return "field";
    }
    pub fn getType(&self) -> Type    {
        return getNat().getFieldType();
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let cmp: i32 = super.compareTo0(other);
        if cmp!=0        {
            return cmp;
        }        
        let otherField: CstFieldRef = (CstFieldRef*)other;
        let thisDescriptor: CstString = getNat().getDescriptor();
        let otherDescriptor: CstString = otherField.getNat().getDescriptor();
        return thisDescriptor.compareTo(&otherDescriptor);
    }
}
