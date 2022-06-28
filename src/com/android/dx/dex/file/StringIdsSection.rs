use crate::helper;
use crate::com::android::dx::dex::file::StringIdItem;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstNat;

struct StringIdsSection{
    pub strings: TreeMap<CstString,StringIdItem>,
}
impl StringIdsSection{
    pub fn new(&self, file: &DexFile)    {
        super("string_ids",file,4);

        self.strings=TreeMap<CstString,StringIdItem>::new();
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.strings.values();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let result: IndexedItem = self.strings.get((CstString*)cst);
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        let sz: i32 = self.strings.size();
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                if out.annotates()                {
                    out.annotate_int_String(4, "string_ids_size: "+Hex::u4(sz));
                    out.annotate_int_String(4, "string_ids_off:  "+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn intern(&self, string: &String) -> StringIdItem            {
                return intern_StringIdItem(StringIdItem::new(CstString::new(&string)));
            }
            pub fn intern(&self, string: &CstString) -> StringIdItem            {
                return intern_StringIdItem(StringIdItem::new(&string));
            }
            pub fn intern(&self, string: &StringIdItem) -> StringIdItem            {
                if string==None                {
                    throw NullPointerException::new("string == null");
                }                
                throwIfPrepared();
                let value: CstString = string.getValue();
                let already: StringIdItem = self.strings.get(&value);
                if already!=None                {
                    return already;
                }                
                self.strings.put(&value, &string);
                return string;
            }
            pub fn intern(&self, nat: &CstNat)            {
                intern_CstString(nat.getName());
                intern_CstString(nat.getDescriptor());
            }
            pub fn indexOf(&self, string: &CstString) -> i32            {
                if string==None                {
                    throw NullPointerException::new("string == null");
                }                
                throwIfNotPrepared();
                let s: StringIdItem = self.strings.get(&string);
                if s==None                {
                    throw IllegalArgumentException::new("not found");
                }                
                return s.getIndex();
            }
            pub fn orderItems(&self)            {
                let idx: i32 = 0;
                for s in self.strings.values()                {
                    s.setIndex(idx);
                    idx += 1;
                }
            }
}
