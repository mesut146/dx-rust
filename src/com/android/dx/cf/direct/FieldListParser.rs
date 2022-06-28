use crate::helper;
use crate::com::android::dx::cf::iface::StdFieldList;
use crate::com::android::dx::cf::iface::StdField;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::cf::direct::AttributeFactory;

struct FieldListParser{
    pub fields: StdFieldList,
}
impl FieldListParser{
    pub fn new(&self, cf: &DirectClassFile, definer: &CstType, offset: i32, attributeFactory: &AttributeFactory)    {
        super(cf,definer,offset,attributeFactory);

        self.fields=StdFieldList::new(getCount());
    }
    pub fn getList(&self) -> StdFieldList    {
        parseIfNecessary();
        return self.fields;
    }
    pub fn humanName(&self) -> String    {
        return "field";
    }
    pub fn humanAccessFlags(&self, accessFlags: i32) -> String    {
        return AccessFlags::fieldString(accessFlags);
    }
    pub fn getAttributeContext(&self) -> i32    {
        return AttributeFactory::CTX_FIELD;
    }
    pub fn set(&self, n: i32, accessFlags: i32, nat: &CstNat, attributes: &AttributeList) -> Member    {
        let field: StdField = StdField::new(getDefiner(), accessFlags, &nat, &attributes);
        self.fields.set(n, &field);
        return field;
    }
}
