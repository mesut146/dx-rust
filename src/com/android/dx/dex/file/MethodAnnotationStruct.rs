use crate::helper;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::AnnotationSetItem;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::file::MethodAnnotationStruct;
use crate::com::android::dx::util::Hex;

struct MethodAnnotationStruct{
    pub method: CstMethodRef,
    pub annotations: AnnotationSetItem,
}
impl MethodAnnotationStruct{
    pub fn new(&self, method: &CstMethodRef, annotations: &AnnotationSetItem)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        if annotations==None        {
            throw NullPointerException::new("annotations == null");
        }        
        self->method=method;
        self->annotations=annotations;
    }
    pub fn hashCode(&self) -> i32    {
        return self.method.hashCode();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof MethodAnnotationStruct)        {
            return false;
        }        
        return self.method.equals(((MethodAnnotationStruct*)other)->method);
    }
    pub fn compareTo(&self, other: &MethodAnnotationStruct) -> i32    {
        return self.method.compareTo(&);
    }
    pub fn addContents(&self, file: &DexFile)    {
        let methodIds: MethodIdsSection = file.getMethodIds();
        let wordData: MixedItemSection = file.getWordData();
        methodIds.intern(&self.method);
        self.annotations=wordData.intern(&self.annotations);
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let methodIdx: i32 = file.getMethodIds().indexOf(&self.method);
        let annotationsOff: i32 = self.annotations.getAbsoluteOffset();
        if out.annotates()        {
            out.annotate_int_String(0, "    "+self.method.toHuman());
            out.annotate_int_String(4, "      method_idx:      "+Hex::u4(methodIdx));
            out.annotate_int_String(4, "      annotations_off: "+Hex::u4(annotationsOff));
        }        
        out.writeInt(methodIdx);
        out.writeInt(annotationsOff);
    }
    pub fn toHuman(&self) -> String    {
        return self.method.toHuman()+": "+self.annotations;
    }
    pub fn getMethod(&self) -> CstMethodRef    {
        return self.method;
    }
    pub fn getAnnotations(&self) -> Annotations    {
        return self.annotations.getAnnotations();
    }
}
