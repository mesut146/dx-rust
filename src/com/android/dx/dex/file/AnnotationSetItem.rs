use crate::helper;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::AnnotationSetItem;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::AnnotationItem;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::util::Hex;

struct AnnotationSetItem{
    pub annotations: Annotations,
    pub items: Vec<AnnotationItem>,
}
impl AnnotationSetItem{
    pub const ALIGNMENT: i32 = 4;
    pub const ENTRY_WRITE_SIZE: i32 = 4;
    pub fn new(&self, annotations: &Annotations, dexFile: &DexFile)    {
        super(ALIGNMENT,writeSize(annotations));

        self->annotations=annotations;
        self->items=new AnnotationItem[annotations.size()];
        let at: i32 = 0;
        for a in annotations.getAnnotations()        {
            self.items[at]=AnnotationItem::new(&a, &dexFile);
            at += 1;
        }
    }
    pub fn writeSize(annotations: &Annotations) -> i32    {
        try        {
            return (annotations.size()*AnnotationSetItem::ENTRY_WRITE_SIZE)+4;
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("list == null");
        }
    }
    pub fn getAnnotations(&self) -> Annotations    {
        return self.annotations;
    }
    pub fn hashCode(&self) -> i32    {
        return self.annotations.hashCode();
    }
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        let otherSet: AnnotationSetItem = (AnnotationSetItem*)other;
        return self.annotations.compareTo(&);
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_ANNOTATION_SET_ITEM;
    }
    pub fn toHuman(&self) -> String    {
        return self.annotations.toString();
    }
    pub fn addContents(&self, file: &DexFile)    {
        let byteData: MixedItemSection = file.getByteData();
        let size: i32 = self.items.len();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            self.items[i]=byteData.intern(self.items[i]);
        }
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        AnnotationItem::sortByTypeIdIndex(&self.items);
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        let size: i32 = self.items.len();
        if annotates        {
            out.annotate_int_String(0, offsetString()+" annotation set");
            out.annotate_int_String(4, "  size: "+Hex::u4(size));
        }        
        out.writeInt(size);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let item: AnnotationItem = self.items[i];
            let offset: i32 = item.getAbsoluteOffset();
            if annotates            {
                out.annotate_int_String(4, "  entries["+Integer::toHexString(i)+"]: "+Hex::u4(offset));
                self.items[i].annotateTo(&out, "    ");
            }            
            out.writeInt(offset);
        }
    }
}
