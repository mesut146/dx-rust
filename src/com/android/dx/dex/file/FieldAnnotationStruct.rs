use crate::helper;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::AnnotationSetItem;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dx::dex::file::FieldAnnotationStruct;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstFieldRef;

struct FieldAnnotationStruct{
    pub field: CstFieldRef,
    pub annotations: AnnotationSetItem,
}
impl FieldAnnotationStruct{
    pub fn new(&self, field: &CstFieldRef, annotations: &AnnotationSetItem)    {
        if field==None        {
            throw NullPointerException::new("field == null");
        }        
        if annotations==None        {
            throw NullPointerException::new("annotations == null");
        }        
        self->field=field;
        self->annotations=annotations;
    }
    pub fn hashCode(&self) -> i32    {
        return self.field.hashCode();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof FieldAnnotationStruct)        {
            return false;
        }        
        return self.field.equals(((FieldAnnotationStruct*)other)->field);
    }
    pub fn compareTo(&self, other: &FieldAnnotationStruct) -> i32    {
        return self.field.compareTo(&);
    }
    pub fn addContents(&self, file: &DexFile)    {
        let fieldIds: FieldIdsSection = file.getFieldIds();
        let wordData: MixedItemSection = file.getWordData();
        fieldIds.intern(&self.field);
        self.annotations=wordData.intern(&self.annotations);
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let fieldIdx: i32 = file.getFieldIds().indexOf(&self.field);
        let annotationsOff: i32 = self.annotations.getAbsoluteOffset();
        if out.annotates()        {
            out.annotate_int_String(0, "    "+self.field.toHuman());
            out.annotate_int_String(4, "      field_idx:       "+Hex::u4(fieldIdx));
            out.annotate_int_String(4, "      annotations_off: "+Hex::u4(annotationsOff));
        }        
        out.writeInt(fieldIdx);
        out.writeInt(annotationsOff);
    }
    pub fn toHuman(&self) -> String    {
        return self.field.toHuman()+": "+self.annotations;
    }
    pub fn getField(&self) -> CstFieldRef    {
        return self.field;
    }
    pub fn getAnnotations(&self) -> Annotations    {
        return self.annotations.getAnnotations();
    }
}
