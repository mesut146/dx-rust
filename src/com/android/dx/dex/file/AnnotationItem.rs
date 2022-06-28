use crate::helper;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::rop::annotation::AnnotationVisibility;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::ValueEncoder;
use crate::com::android::dx::dex::file::AnnotationItem::TypeIdSorter;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::dex::file::TypeIdItem;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::file::AnnotationItem;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::DexFile;

let static TYPE_ID_SORTER: TypeIdSorter = TypeIdSorter::new();
struct AnnotationItem{
    pub annotation: Annotation,
    pub type: TypeIdItem,
    pub encodedForm: Vec<i8>,
}
impl AnnotationItem{
    pub const VISIBILITY_BUILD: i32 = 0;
    pub const VISIBILITY_RUNTIME: i32 = 1;
    pub const VISIBILITY_SYSTEM: i32 = 2;
    pub const ALIGNMENT: i32 = 1;
    pub fn sortByTypeIdIndex(array: &Vec<AnnotationItem>)    {
        Arrays::sort_AnnotationItem[]_Comparator<? super AnnotationItem>(&array, &AnnotationItem::TYPE_ID_SORTER);
    }
    pub fn new(&self, annotation: &Annotation, dexFile: &DexFile)    {
        super(ALIGNMENT,-1);

        if annotation==None        {
            throw NullPointerException::new("annotation == null");
        }        
        self->annotation=annotation;
        self->type=None;
        self->encodedForm=None;
        addContents(&dexFile);
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_ANNOTATION_ITEM;
    }
    pub fn hashCode(&self) -> i32    {
        return self.annotation.hashCode();
    }
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        let otherAnnotation: AnnotationItem = (AnnotationItem*)other;
        return self.annotation.compareTo(&);
    }
    pub fn toHuman(&self) -> String    {
        return self.annotation.toHuman();
    }
    pub fn addContents(&self, file: &DexFile)    {
        self.type_renamed=file.getTypeIds().intern_CstType(self.annotation.getType());
        ValueEncoder::addContents_DexFile_Annotation(&file, &self.annotation);
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new();
        let encoder: ValueEncoder = ValueEncoder::new(addedTo.getFile(), &out);
        encoder.writeAnnotation(&self.annotation, false);
        self.encodedForm=out.toByteArray();
        setWriteSize(self.encodedForm.len()+1);
    }
    pub fn annotateTo(&self, out: &AnnotatedOutput, prefix: &String)    {
        out.annotate_int_String(0, prefix+"visibility: "+self.annotation.getVisibility().toHuman());
        out.annotate_int_String(0, prefix+"type: "+self.annotation.getType().toHuman());
        for pair in self.annotation.getNameValuePairs()        {
            let name: CstString = pair.getName();
            let value: Constant = pair.getValue();
            out.annotate_int_String(0, prefix+name.toHuman()+": "+ValueEncoder::constantToHuman(&value));
        }
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        let visibility: AnnotationVisibility = self.annotation.getVisibility();
        if annotates        {
            out.annotate_int_String(0, offsetString()+" annotation");
            out.annotate_int_String(1, "  visibility: VISBILITY_"+visibility);
        }        
        match visibility{AnnotationVisibility::BUILD =>             out.writeByte(AnnotationItem::VISIBILITY_BUILD);            break;AnnotationVisibility::RUNTIME =>             out.writeByte(AnnotationItem::VISIBILITY_RUNTIME);            break;AnnotationVisibility::SYSTEM =>             out.writeByte(AnnotationItem::VISIBILITY_SYSTEM);            break;        _ => {}        {
            throw RuntimeException::new("shouldn't happen");
        }    }
    if annotates    {
        let encoder: ValueEncoder = ValueEncoder::new(&file, &out);
        encoder.writeAnnotation(&self.annotation, true);
    }    else     {
        out.write_byte[](&self.encodedForm);
    }
}
}
