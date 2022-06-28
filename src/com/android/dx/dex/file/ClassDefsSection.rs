use crate::helper;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::ClassDefItem;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct ClassDefsSection{
    pub classDefs: TreeMap<Type,ClassDefItem>,
    pub orderedDefs: ArrayList<ClassDefItem>,
}
impl ClassDefsSection{
    pub fn new(&self, file: &DexFile)    {
        super("class_defs",file,4);

        self.classDefs=TreeMap<Type,ClassDefItem>::new();
        self.orderedDefs=None;
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        if self.orderedDefs!=None        {
            return self.orderedDefs;
        }        
        return self.classDefs.values();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let type: Type = ((CstType*)cst).getClassType();
        let result: IndexedItem = self.classDefs.get(&type_renamed);
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        let sz: i32 = self.classDefs.size();
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                if out.annotates()                {
                    out.annotate_int_String(4, "class_defs_size: "+Hex::u4(sz));
                    out.annotate_int_String(4, "class_defs_off:  "+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn add(&self, clazz: &ClassDefItem)            {
                let type: Type;
                try                {
                    type_renamed=clazz.getThisClass().getClassType();
                }                catch(                let ex: NullPointerException)                {
                    throw NullPointerException::new("clazz == null");
                }
                throwIfPrepared();
                if self.classDefs.get(&type_renamed)!=None                {
                    throw IllegalArgumentException::new("already added: "+type_renamed);
                }                
                self.classDefs.put(&type_renamed, &clazz);
            }
            pub fn orderItems(&self)            {
                let sz: i32 = self.classDefs.size();
                let idx: i32 = 0;
                self.orderedDefs=ArrayList<ClassDefItem>::new(sz);
                for type in self.classDefs.keySet()                {
                    idx=orderItems0(&type_renamed, idx, sz-idx);
                }
            }
            pub fn orderItems0(&self, type: &Type, idx: i32, maxDepth: i32) -> i32            {
                let c: ClassDefItem = self.classDefs.get(&type);
                if (c==None)||(c.hasIndex())                {
                    return idx;
                }                
                if maxDepth<0                {
                    throw RuntimeException::new("class circularity with "+type);
                }                
                maxDepth -= 1;
                let superclassCst: CstType = c.getSuperclass();
                if superclassCst!=None                {
                    let superclass: Type = superclassCst.getClassType();
                    idx=orderItems0(&superclass, idx, maxDepth);
                }                
                let interfaces: TypeList = c.getInterfaces();
                let sz: i32 = interfaces.size();
                for(                let i: i32 = 0;;i<szi += 1)                {
                    idx=orderItems0(interfaces.getType(i), idx, maxDepth);
                }
                c.setIndex(idx);
                self.orderedDefs.add_ClassDefItem(&c);
                return idx+1;
            }
}
