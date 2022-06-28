use crate::helper;
use crate::com::android::dx::dex::file::FieldIdItem;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstFieldRef;

struct FieldIdsSection{
    pub fieldIds: TreeMap<CstFieldRef,FieldIdItem>,
}
impl FieldIdsSection{
    pub fn new(&self, file: &DexFile)    {
        super("field_ids",file);

        self.fieldIds=TreeMap<CstFieldRef,FieldIdItem>::new();
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.fieldIds.values();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let result: IndexedItem = self.fieldIds.get((CstFieldRef*)cst);
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        let sz: i32 = self.fieldIds.size();
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                if out.annotates()                {
                    out.annotate_int_String(4, "field_ids_size:  "+Hex::u4(sz));
                    out.annotate_int_String(4, "field_ids_off:   "+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn intern(&self, field: &CstFieldRef) -> FieldIdItem            {
                if field==None                {
                    throw NullPointerException::new("field == null");
                }                
                throwIfPrepared();
                let result: FieldIdItem = self.fieldIds.get(&field);
                if result==None                {
                    result=FieldIdItem::new(&field);
                    self.fieldIds.put(&field, &result);
                }                
                return result;
            }
            pub fn indexOf(&self, ref: &CstFieldRef) -> i32            {
                if ref==None                {
                    throw NullPointerException::new("ref == null");
                }                
                throwIfNotPrepared();
                let item: FieldIdItem = self.fieldIds.get(&ref);
                if item==None                {
                    throw IllegalArgumentException::new("not found");
                }                
                return item.getIndex();
            }
}
