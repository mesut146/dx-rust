use crate::helper;
use crate::com::android::dx::dex::file::ProtoIdsSection;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dex::DexFormat;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::file::ClassDefsSection;

struct HeaderItem{
}
impl HeaderItem{
    pub fn new(&self)    {
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_HEADER_ITEM;
    }
    pub fn writeSize(&self) -> i32    {
        return SizeOf::HEADER_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let mapOff: i32 = file.getMap().getFileOffset();
        let firstDataSection: Section = file.getFirstDataSection();
        let lastDataSection: Section = file.getLastDataSection();
        let dataOff: i32 = firstDataSection.getFileOffset();
        let dataSize: i32 = lastDataSection.getFileOffset()+lastDataSection.writeSize()-dataOff;
        let magic: String = file.getDexOptions().getMagic();
        if out.annotates()        {
            out.annotate_int_String(8, "magic: "+CstString::new(&magic).toQuoted());
            out.annotate_int_String(4, "checksum");
            out.annotate_int_String(20, "signature");
            out.annotate_int_String(4, "file_size:       "+Hex::u4(file.getFileSize()));
            out.annotate_int_String(4, "header_size:     "+Hex::u4(SizeOf::HEADER_ITEM));
            out.annotate_int_String(4, "endian_tag:      "+Hex::u4(DexFormat::ENDIAN_TAG));
            out.annotate_int_String(4, "link_size:       0");
            out.annotate_int_String(4, "link_off:        0");
            out.annotate_int_String(4, "map_off:         "+Hex::u4(mapOff));
        }        
        for(        let i: i32 = 0;;i<8i += 1)        {
            out.writeByte(magic.charAt(i));
        }
        out.writeZeroes(24);
        out.writeInt(file.getFileSize());
        out.writeInt(SizeOf::HEADER_ITEM);
        out.writeInt(DexFormat::ENDIAN_TAG);
        out.writeZeroes(8);
        out.writeInt(mapOff);
        file.getStringIds().writeHeaderPart(&out);
        file.getTypeIds().writeHeaderPart(&out);
        file.getProtoIds().writeHeaderPart(&out);
        file.getFieldIds().writeHeaderPart(&out);
        file.getMethodIds().writeHeaderPart(&out);
        file.getClassDefs().writeHeaderPart(&out);
        if out.annotates()        {
            out.annotate_int_String(4, "data_size:       "+Hex::u4(dataSize));
            out.annotate_int_String(4, "data_off:        "+Hex::u4(dataOff));
        }        
        out.writeInt(dataSize);
        out.writeInt(dataOff);
    }
}
