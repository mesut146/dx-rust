use crate::helper;
use crate::com::android::dx::dex::file::UniformListItem<com::android::dx::dex::file::AnnotationSetRefItem>;
use crate::com::android::dx::dex::file::AnnotationSetRefItem;
use crate::com::android::dx::rop::annotation::AnnotationsList;
use crate::com::android::dx::dex::file::ParameterAnnotationStruct;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::AnnotationSetItem;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::dex::file::DexFile;

struct ParameterAnnotationStruct{
    pub method: CstMethodRef,
    pub annotationsList: AnnotationsList,
    pub annotationsItem: UniformListItem<AnnotationSetRefItem>,
}
impl ParameterAnnotationStruct{
    pub fn new(&self, method: &CstMethodRef, annotationsList: &AnnotationsList, dexFile: &DexFile)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        if annotationsList==None        {
            throw NullPointerException::new("annotationsList == null");
        }        
        self->method=method;
        self->annotationsList=annotationsList;
        let size: i32 = annotationsList.size();
        let arrayList: ArrayList<AnnotationSetRefItem> = ArrayList<AnnotationSetRefItem>::new(size);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let annotations: Annotations = annotationsList.get(i);
            let item: AnnotationSetItem = AnnotationSetItem::new(&annotations, &dexFile);
            arrayList.add_AnnotationSetRefItem(AnnotationSetRefItem::new(&item));
        }
        self->annotationsItem=UniformListItem<AnnotationSetRefItem>::new(&ItemType::TYPE_ANNOTATION_SET_REF_LIST, &arrayList);
    }
    pub fn hashCode(&self) -> i32    {
        return self.method.hashCode();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof ParameterAnnotationStruct)        {
            return false;
        }        
        return self.method.equals(((ParameterAnnotationStruct*)other)->method);
    }
    pub fn compareTo(&self, other: &ParameterAnnotationStruct) -> i32    {
        return self.method.compareTo(&);
    }
    pub fn addContents(&self, file: &DexFile)    {
        let methodIds: MethodIdsSection = file.getMethodIds();
        let wordData: MixedItemSection = file.getWordData();
        methodIds.intern(&self.method);
        wordData.add(&self.annotationsItem);
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let methodIdx: i32 = file.getMethodIds().indexOf(&self.method);
        let annotationsOff: i32 = self.annotationsItem.getAbsoluteOffset();
        if out.annotates()        {
            out.annotate_int_String(0, "    "+self.method.toHuman());
            out.annotate_int_String(4, "      method_idx:      "+Hex::u4(methodIdx));
            out.annotate_int_String(4, "      annotations_off: "+Hex::u4(annotationsOff));
        }        
        out.writeInt(methodIdx);
        out.writeInt(annotationsOff);
    }
    pub fn toHuman(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        sb.append_String(self.method.toHuman());
        sb.append_String(": ");
        let first: boolean = true;
        for item in self.annotationsItem.getItems()        {
            if first            {
                first=false;
            }            else             {
                sb.append_String(", ");
            }
            sb.append_String(item.toHuman());
        }
        return sb.toString();
    }
    pub fn getMethod(&self) -> CstMethodRef    {
        return self.method;
    }
    pub fn getAnnotationsList(&self) -> AnnotationsList    {
        return self.annotationsList;
    }
}
