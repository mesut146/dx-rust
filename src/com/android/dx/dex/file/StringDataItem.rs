use crate::helper;
use crate::com::android::dex::Leb128;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::StringDataItem;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct StringDataItem{
    pub value: CstString,
}
impl StringDataItem{
    pub fn new(&self, value: &CstString)    {
        super(1,writeSize(value));

        self->value=value;
    }
    pub fn writeSize(value: &CstString) -> i32    {
        let utf16Size: i32 = value.getUtf16Size();
        return Leb128::unsignedLeb128Size(utf16Size)+value.getUtf8Size()+1;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_STRING_DATA_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let bytes: ByteArray = self.value.getBytes();
        let utf16Size: i32 = self.value.getUtf16Size();
        if out.annotates()        {
            out.annotate_int_String(Leb128::unsignedLeb128Size(utf16Size), "utf16_size: "+Hex::u4(utf16Size));
            out.annotate_int_String(bytes.size()+1, self.value.toQuoted());
        }        
        out.writeUleb128(utf16Size);
        out.write_ByteArray(&bytes);
        out.writeByte(0);
    }
    pub fn toHuman(&self) -> String    {
        return self.value.toQuoted();
    }
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        let otherData: StringDataItem = (StringDataItem*)other;
        return self.value.compareTo(&);
    }
}
