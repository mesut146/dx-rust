use crate::helper;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::dex::file::ProtoIdItem;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct ProtoIdsSection{
    pub protoIds: TreeMap<Prototype,ProtoIdItem>,
}
impl ProtoIdsSection{
    pub fn new(&self, file: &DexFile)    {
        super("proto_ids",file,4);

        self.protoIds=TreeMap<Prototype,ProtoIdItem>::new();
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.protoIds.values();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        if !(//cst instanceof CstProtoRef)        {
            throw IllegalArgumentException::new("cst not instance of CstProtoRef");
        }        
        throwIfNotPrepared();
        let protoRef: CstProtoRef = (CstProtoRef*)cst;
        let result: IndexedItem = self.protoIds.get(protoRef.getPrototype());
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        let sz: i32 = self.protoIds.size();
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                if sz>65536                {
                    throw UnsupportedOperationException::new("too many proto ids");
                }                
                if out.annotates()                {
                    out.annotate_int_String(4, "proto_ids_size:  "+Hex::u4(sz));
                    out.annotate_int_String(4, "proto_ids_off:   "+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn intern(&self, prototype: &Prototype) -> ProtoIdItem            {
                if prototype==None                {
                    throw NullPointerException::new("prototype == null");
                }                
                throwIfPrepared();
                let result: ProtoIdItem = self.protoIds.get(&prototype);
                if result==None                {
                    result=ProtoIdItem::new(&prototype);
                    self.protoIds.put(&prototype, &result);
                }                
                return result;
            }
            pub fn indexOf(&self, prototype: &Prototype) -> i32            {
                if prototype==None                {
                    throw NullPointerException::new("prototype == null");
                }                
                throwIfNotPrepared();
                let item: ProtoIdItem = self.protoIds.get(&prototype);
                if item==None                {
                    throw IllegalArgumentException::new("not found");
                }                
                return item.getIndex();
            }
            pub fn orderItems(&self)            {
                let idx: i32 = 0;
                for i in items()                {
                    ((ProtoIdItem*)i).setIndex(idx);
                    idx += 1;
                }
            }
}
