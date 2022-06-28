use crate::helper;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::ProtoIdsSection;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;

struct MethodIdItem{
}
impl MethodIdItem{
    pub fn new(&self, method: &CstBaseMethodRef)    {
        super(method);

    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_METHOD_ID_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        super.addContents(file);
        let protoIds: ProtoIdsSection = file.getProtoIds();
        protoIds.intern(getMethodRef().getPrototype());
    }
    pub fn getMethodRef(&self) -> CstBaseMethodRef    {
        return (CstBaseMethodRef*)getRef();
    }
    pub fn getTypoidIdx(&self, file: &DexFile) -> i32    {
        let protoIds: ProtoIdsSection = file.getProtoIds();
        return protoIds.indexOf(getMethodRef().getPrototype());
    }
    pub fn getTypoidName(&self) -> String    {
        return "proto_idx";
    }
}
