use crate::helper;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::cf::attrib::AttConstantValue;

struct StdField{
}
impl StdField{
    pub fn new(&self, definingClass: &CstType, accessFlags: i32, nat: &CstNat, attributes: &AttributeList)    {
        super(definingClass,accessFlags,nat,attributes);

    }
    pub fn getConstantValue(&self) -> TypedConstant    {
        let attribs: AttributeList = getAttributes();
        let cval: AttConstantValue = (AttConstantValue*)attribs.findFirst(&AttConstantValue::ATTRIBUTE_NAME);
        if cval==None        {
            return None;
        }        
        return cval.getConstantValue();
    }
}
