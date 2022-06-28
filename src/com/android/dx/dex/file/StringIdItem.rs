use crate::helper;
use crate::com::android::dx::dex::file::StringIdItem;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::StringDataItem;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct StringIdItem{
    pub value: CstString,
    pub data: StringDataItem,
}
impl StringIdItem{
    pub fn new(&self, value: &CstString)    {
        if value==None        {
            throw NullPointerException::new("value == null");
        }        
        self->value=value;
        self->data=None;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof StringIdItem)        {
            return false;
        }        
        let otherString: StringIdItem = (StringIdItem*)other;
        return self.value.equals(&);
    }
    pub fn hashCode(&self) -> i32    {
        return self.value.hashCode();
    }
    pub fn compareTo(&self, other: &Object) -> i32    {
        let otherString: StringIdItem = (StringIdItem*)other;
        return self.value.compareTo(&);
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_STRING_ID_ITEM;
    }
    pub fn writeSize(&self) -> i32    {
        return SizeOf::STRING_ID_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        if self.data==None        {
            let stringData: MixedItemSection = file.getStringData();
            self.data=StringDataItem::new(&self.value);
            stringData.add(&self.data);
        }        
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let dataOff: i32 = self.data.getAbsoluteOffset();
        if out.annotates()        {
            out.annotate_int_String(0, indexString()+' '+self.value.toQuoted_int(100));
            out.annotate_int_String(4, "  string_data_off: "+Hex::u4(dataOff));
        }        
        out.writeInt(dataOff);
    }
    pub fn getValue(&self) -> CstString    {
        return self.value;
    }
    pub fn getData(&self) -> StringDataItem    {
        return self.data;
    }
}
