use crate::helper;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::rop::cst::CstFieldRef;

struct FieldIdItem{
}
impl FieldIdItem{
    pub fn new(&self, field: &CstFieldRef)    {
        super(field);

    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_FIELD_ID_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        super.addContents(file);
        let typeIds: TypeIdsSection = file.getTypeIds();
        typeIds.intern_Type(getFieldRef().getType());
    }
    pub fn getFieldRef(&self) -> CstFieldRef    {
        return (CstFieldRef*)getRef();
    }
    pub fn getTypoidIdx(&self, file: &DexFile) -> i32    {
        let typeIds: TypeIdsSection = file.getTypeIds();
        return typeIds.indexOf_Type(getFieldRef().getType());
    }
    pub fn getTypoidName(&self) -> String    {
        return "type_idx";
    }
}
