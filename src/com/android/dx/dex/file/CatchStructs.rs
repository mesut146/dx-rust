use crate::helper;
use crate::com::android::dx::dex::code::CatchHandlerList;
use crate::com::android::dx::dex::code::CatchTable::Entry;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::code::CatchTable;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::code::CatchHandlerList::Entry;

struct CatchStructs{
    pub code: DalvCode,
    pub table: CatchTable,
    pub encodedHandlers: Vec<i8>,
    pub encodedHandlerHeaderSize: i32,
    pub handlerOffsets: TreeMap<CatchHandlerList,Integer>,
}
impl CatchStructs{
    pub const TRY_ITEM_WRITE_SIZE: i32 = 4+(2*2);
    pub fn new(&self, code: &DalvCode)    {
        self->code=code;
        self->table=None;
        self->encodedHandlers=None;
        self->encodedHandlerHeaderSize=0;
        self->handlerOffsets=None;
    }
    pub fn finishProcessingIfNecessary(&self)    {
        if self.table==None        {
            self.table=self.code.getCatches();
        }        
    }
    pub fn triesSize(&self) -> i32    {
        finishProcessingIfNecessary();
        return self.table.size();
    }
    pub fn debugPrint(&self, out: &PrintWriter, prefix: &String)    {
        annotateEntries(&prefix, &out, None);
    }
    pub fn encode(&self, file: &DexFile)    {
        finishProcessingIfNecessary();
        let typeIds: TypeIdsSection = file.getTypeIds();
        let size: i32 = self.table.size();
        self.handlerOffsets=TreeMap<CatchHandlerList,Integer>::new();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            self.handlerOffsets.put(self.table.get(i).getHandlers(), None);
        }
        if self.handlerOffsets.size()>65535        {
            throw UnsupportedOperationException::new("too many catch handlers");
        }        
        let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new();
        self.encodedHandlerHeaderSize=out.writeUleb128(self.handlerOffsets.size());
        for mapping in self.handlerOffsets.entrySet()        {
            let list: CatchHandlerList = mapping.getKey();
            let listSize: i32 = list.size();
            let catchesAll: boolean = list.catchesAll();
            mapping.setValue(out.getCursor());
            if catchesAll            {
                out.writeSleb128(-(listSize-1));
                listSize -= 1;
            }            else             {
                out.writeSleb128(listSize);
            }
            for(            let i: i32 = 0;;i<listSizei += 1)            {
                let entry: Entry = list.get(i);
                out.writeUleb128(typeIds.indexOf_CstType(entry.getExceptionType()));
                out.writeUleb128(entry.getHandler());
            }
            if catchesAll            {
                out.writeUleb128(list.get(listSize).getHandler());
            }            
        }
        self.encodedHandlers=out.toByteArray();
    }
    pub fn writeSize(&self) -> i32    {
        return (triesSize()*CatchStructs::TRY_ITEM_WRITE_SIZE)++self.encodedHandlers.len();
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        finishProcessingIfNecessary();
        if out.annotates()        {
            annotateEntries("  ", None, &out);
        }        
        let tableSize: i32 = self.table.size();
        for(        let i: i32 = 0;;i<tableSizei += 1)        {
            let one: Entry = self.table.get(i);
            let start: i32 = one.getStart();
            let end: i32 = one.getEnd();
            let insnCount: i32 = end-start;
            if insnCount>=65536            {
                throw UnsupportedOperationException::new("bogus exception range: "+Hex::u4(start)+".."+Hex::u4(end));
            }            
            out.writeInt(start);
            out.writeShort(insnCount);
            out.writeShort(self.handlerOffsets.get(one.getHandlers()));
        }
        out.write_byte[](&self.encodedHandlers);
    }
    pub fn annotateEntries(&self, prefix: &String, printTo: &PrintWriter, annotateTo: &AnnotatedOutput)    {
        finishProcessingIfNecessary();
        let consume: boolean = (annotateTo!=None);
        let amt1: i32 = if consume { 6 } else { 0 };
                let amt2: i32 = if consume { 2 } else { 0 };
                        let size: i32 = self.table.size();
                        let subPrefix: String = prefix+"  ";
                        if consume                        {
                            annotateTo.annotate_int_String(0, prefix+"tries:");
                        }                        else                         {
                            printTo.println_String(prefix+"tries:");
                        }
                        for(                        let i: i32 = 0;;i<sizei += 1)                        {
                            let entry: Entry = self.table.get(i);
                            let handlers: CatchHandlerList = entry.getHandlers();
                            let s1: String = subPrefix+"try "+Hex::u2or4(entry.getStart())+".."+Hex::u2or4(entry.getEnd());
                            let s2: String = handlers.toHuman_String_String(&subPrefix, "");
                            if consume                            {
                                annotateTo.annotate_int_String(amt1, &s1);
                                annotateTo.annotate_int_String(amt2, &s2);
                            }                            else                             {
                                printTo.println_String(&s1);
                                printTo.println_String(&s2);
                            }
                        }
                        if !consume                        {
                            return;
                        }                        
                        annotateTo.annotate_int_String(0, prefix+"handlers:");
                        annotateTo.annotate_int_String(self.encodedHandlerHeaderSize, subPrefix+"size: "+Hex::u2(self.handlerOffsets.size()));
                        let lastOffset: i32 = 0;
                        let lastList: CatchHandlerList = None;
                        for mapping in self.handlerOffsets.entrySet()                        {
                            let list: CatchHandlerList = mapping.getKey();
                            let offset: i32 = mapping.getValue();
                            if lastList!=None                            {
                                CatchStructs::annotateAndConsumeHandlers(&lastList, lastOffset, offset-lastOffset, &subPrefix, &printTo, &annotateTo);
                            }                            
                            lastList=list;
                            lastOffset=offset;
                        }
                        CatchStructs::annotateAndConsumeHandlers(&lastList, lastOffset, self.encodedHandlers.len()-lastOffset, &subPrefix, &printTo, &annotateTo);
                    }
                    pub fn annotateAndConsumeHandlers(handlers: &CatchHandlerList, offset: i32, size: i32, prefix: &String, printTo: &PrintWriter, annotateTo: &AnnotatedOutput)                    {
                        let s: String = handlers.toHuman_String_String(&prefix, Hex::u2(offset)+": ");
                        if printTo!=None                        {
                            printTo.println_String(&s);
                        }                        
                        annotateTo.annotate_int_String(size, &s);
                    }
}
