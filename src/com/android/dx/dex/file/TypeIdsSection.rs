use crate::helper;
use crate::com::android::dex::DexFormat;
use crate::com::android::dex::DexIndexOverflowException;
use crate::com::android::dx::dex::file::TypeIdItem;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct TypeIdsSection{
    pub typeIds: TreeMap<Type,TypeIdItem>,
}
impl TypeIdsSection{
    pub fn new(&self, file: &DexFile)    {
        super("type_ids",file,4);

        self.typeIds=TreeMap<Type,TypeIdItem>::new();
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.typeIds.values();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let type: Type = ((CstType*)cst).getClassType();
        let result: IndexedItem = self.typeIds.get(&type_renamed);
        if result==None        {
            throw IllegalArgumentException::new("not found: "+cst);
        }        
        return result;
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        let sz: i32 = self.typeIds.size();
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                if sz>DexFormat::MAX_TYPE_IDX+1                {
                    throw DexIndexOverflowException::new(String::format_String_Object[]("Too many type identifiers to fit in one dex file: %1$d; max is %2$d.%n"+"You may try using multi-dex. If multi-dex is enabled then the list of "+"classes for the main dex list is too large.", items().size(), DexFormat::MAX_MEMBER_IDX+1));
                }                
                if out.annotates()                {
                    out.annotate_int_String(4, "type_ids_size:   "+Hex::u4(sz));
                    out.annotate_int_String(4, "type_ids_off:    "+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn intern(&self, type: &Type) -> TypeIdItem            {
                if type==None                {
                    throw NullPointerException::new("type == null");
                }                
                throwIfPrepared();
                let result: TypeIdItem = self.typeIds.get(&type);
                if result==None                {
                    result=TypeIdItem::new(CstType::new(&type));
                    self.typeIds.put(&type, &result);
                }                
                return result;
            }
            pub fn intern(&self, type: &CstType) -> TypeIdItem            {
                if type==None                {
                    throw NullPointerException::new("type == null");
                }                
                throwIfPrepared();
                let typePerSe: Type = type.getClassType();
                let result: TypeIdItem = self.typeIds.get(&typePerSe);
                if result==None                {
                    result=TypeIdItem::new(&type);
                    self.typeIds.put(&typePerSe, &result);
                }                
                return result;
            }
            pub fn indexOf(&self, type: &Type) -> i32            {
                if type==None                {
                    throw NullPointerException::new("type == null");
                }                
                throwIfNotPrepared();
                let item: TypeIdItem = self.typeIds.get(&type);
                if item==None                {
                    throw IllegalArgumentException::new("not found: "+type);
                }                
                return item.getIndex();
            }
            pub fn indexOf(&self, type: &CstType) -> i32            {
                if type==None                {
                    throw NullPointerException::new("type == null");
                }                
                return indexOf_Type(type.getClassType());
            }
            pub fn orderItems(&self)            {
                let idx: i32 = 0;
                for i in items()                {
                    ((TypeIdItem*)i).setIndex(idx);
                    idx += 1;
                }
            }
}
