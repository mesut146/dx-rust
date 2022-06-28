use crate::helper;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::cf::direct::AttributeFactory;
use crate::com::android::dx::cf::iface::StdMethod;
use crate::com::android::dx::cf::iface::StdMethodList;

struct MethodListParser{
    pub methods: StdMethodList,
}
impl MethodListParser{
    pub fn new(&self, cf: &DirectClassFile, definer: &CstType, offset: i32, attributeFactory: &AttributeFactory)    {
        super(cf,definer,offset,attributeFactory);

        self.methods=StdMethodList::new(getCount());
    }
    pub fn getList(&self) -> StdMethodList    {
        parseIfNecessary();
        return self.methods;
    }
    pub fn humanName(&self) -> String    {
        return "method";
    }
    pub fn humanAccessFlags(&self, accessFlags: i32) -> String    {
        return AccessFlags::methodString(accessFlags);
    }
    pub fn getAttributeContext(&self) -> i32    {
        return AttributeFactory::CTX_METHOD;
    }
    pub fn set(&self, n: i32, accessFlags: i32, nat: &CstNat, attributes: &AttributeList) -> Member    {
        let meth: StdMethod = StdMethod::new(getDefiner(), accessFlags, &nat, &attributes);
        self.methods.set(n, &meth);
        return meth;
    }
}
