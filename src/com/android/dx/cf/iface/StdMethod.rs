use crate::helper;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::cst::CstNat;

struct StdMethod{
    pub effectiveDescriptor: Prototype,
}
impl StdMethod{
    pub fn new(&self, definingClass: &CstType, accessFlags: i32, nat: &CstNat, attributes: &AttributeList)    {
        super(definingClass,accessFlags,nat,attributes);

        let descStr: String = getDescriptor().getString();
        self.effectiveDescriptor=Prototype::intern_String_Type_boolean_boolean(&descStr, definingClass.getClassType(), AccessFlags::isStatic(accessFlags), nat.isInstanceInit());
    }
    pub fn getEffectiveDescriptor(&self) -> Prototype    {
        return self.effectiveDescriptor;
    }
}
