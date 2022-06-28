use crate::helper;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::AnnotationSetItem;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct AnnotationSetRefItem{
    pub annotations: AnnotationSetItem,
}
impl AnnotationSetRefItem{
    pub const ALIGNMENT: i32 = 4;
    pub const WRITE_SIZE: i32 = 4;
    pub fn new(&self, annotations: &AnnotationSetItem)    {
        super(ALIGNMENT,WRITE_SIZE);

        if annotations==None        {
            throw NullPointerException::new("annotations == null");
        }        
        self->annotations=annotations;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_ANNOTATION_SET_REF_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let wordData: MixedItemSection = file.getWordData();
        self.annotations=wordData.intern(&self.annotations);
    }
    pub fn toHuman(&self) -> String    {
        return self.annotations.toHuman();
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotationsOff: i32 = self.annotations.getAbsoluteOffset();
        if out.annotates()        {
            out.annotate_int_String(4, "  annotations_off: "+Hex::u4(annotationsOff));
        }        
        out.writeInt(annotationsOff);
    }
}
