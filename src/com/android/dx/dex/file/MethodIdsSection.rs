use crate::helper;
use crate::com::android::dx::dex::file::MethodIdItem;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct MethodIdsSection{
    pub methodIds: TreeMap<CstBaseMethodRef,MethodIdItem>,
}
impl MethodIdsSection{
    pub fn new(&self, file: &DexFile)    {
        super("method_ids",file);

        self.methodIds=TreeMap<CstBaseMethodRef,MethodIdItem>::new();
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.methodIds.values();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let result: IndexedItem = self.methodIds.get((CstBaseMethodRef*)cst);
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        let sz: i32 = self.methodIds.size();
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                if out.annotates()                {
                    out.annotate_int_String(4, "method_ids_size: "+Hex::u4(sz));
                    out.annotate_int_String(4, "method_ids_off:  "+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn intern(&self, method: &CstBaseMethodRef) -> MethodIdItem            {
                if method==None                {
                    throw NullPointerException::new("method == null");
                }                
                throwIfPrepared();
                let result: MethodIdItem = self.methodIds.get(&method);
                if result==None                {
                    result=MethodIdItem::new(&method);
                    self.methodIds.put(&method, &result);
                }                
                return result;
            }
            pub fn indexOf(&self, ref: &CstBaseMethodRef) -> i32            {
                if ref==None                {
                    throw NullPointerException::new("ref == null");
                }                
                throwIfNotPrepared();
                let item: MethodIdItem = self.methodIds.get(&ref);
                if item==None                {
                    throw IllegalArgumentException::new("not found");
                }                
                return item.getIndex();
            }
}
