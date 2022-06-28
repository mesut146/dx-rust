use crate::helper;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct TypeIdItem{
}
impl TypeIdItem{
    pub fn new(&self, type: &CstType)    {
        super(type);

    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_TYPE_ID_ITEM;
    }
    pub fn writeSize(&self) -> i32    {
        return SizeOf::TYPE_ID_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        file.getStringIds().intern_CstString(getDefiningClass().getDescriptor());
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let type: CstType = getDefiningClass();
        let descriptor: CstString = type_renamed.getDescriptor();
        let idx: i32 = file.getStringIds().indexOf(&descriptor);
        if out.annotates()        {
            out.annotate_int_String(0, indexString()+' '+descriptor.toHuman());
            out.annotate_int_String(4, "  descriptor_idx: "+Hex::u4(idx));
        }        
        out.writeInt(idx);
    }
}
