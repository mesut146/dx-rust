use crate::helper;
use crate::com::android::dx::dex::file::DexFile::Storage;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::dex::file::Statistics;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::MethodHandlesSection;
use crate::com::android::dx::rop::cst::CstEnumRef;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::dex::file::ProtoIdsSection;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::dex::file::ClassDefItem;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::dex::file::HeaderSection;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dx::dex::file::MapItem;
use crate::com::android::dx::dex::file::MixedItemSection::SortType;
use crate::com::android::dex::DexFormat;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::file::CallSiteIdsSection;
use crate::com::android::dx::dex::file::ClassDefsSection;
use crate::com::android::dx::rop::cst::CstMethodHandle;

struct DexFile{
    pub dexOptions: DexOptions,
    pub wordData: MixedItemSection,
    pub typeLists: MixedItemSection,
    pub map: MixedItemSection,
    pub stringData: MixedItemSection,
    pub stringIds: StringIdsSection,
    pub typeIds: TypeIdsSection,
    pub protoIds: ProtoIdsSection,
    pub fieldIds: FieldIdsSection,
    pub methodIds: MethodIdsSection,
    pub classDefs: ClassDefsSection,
    pub classData: MixedItemSection,
    pub callSiteIds: CallSiteIdsSection,
    pub methodHandles: MethodHandlesSection,
    pub byteData: MixedItemSection,
    pub header: HeaderSection,
    pub sections: Vec<Section>,
    pub fileSize: i32,
    pub dumpWidth: i32,
}
impl DexFile{
    pub fn new(&self, dexOptions: &DexOptions)    {
        self->dexOptions=dexOptions;
        self.header=HeaderSection::new(self);
        self.typeLists=MixedItemSection::new(None, self, 4, &SortType::NONE);
        self.wordData=MixedItemSection::new("word_data", self, 4, &SortType::TYPE);
        self.stringData=MixedItemSection::new("string_data", self, 1, &SortType::INSTANCE);
        self.classData=MixedItemSection::new(None, self, 1, &SortType::NONE);
        self.byteData=MixedItemSection::new("byte_data", self, 1, &SortType::TYPE);
        self.stringIds=StringIdsSection::new(self);
        self.typeIds=TypeIdsSection::new(self);
        self.protoIds=ProtoIdsSection::new(self);
        self.fieldIds=FieldIdsSection::new(self);
        self.methodIds=MethodIdsSection::new(self);
        self.classDefs=ClassDefsSection::new(self);
        self.map=MixedItemSection::new("map", self, 4, &SortType::NONE);
        if dexOptions.apiIsSupported(DexFormat::API_METHOD_HANDLES)        {
            self.callSiteIds=CallSiteIdsSection::new(self);
            self.methodHandles=MethodHandlesSection::new(self);
            self.sections=vec![self.header, self.stringIds, self.typeIds, self.protoIds, self.fieldIds, self.methodIds, self.classDefs, self.callSiteIds, self.methodHandles, self.wordData, self.typeLists, self.stringData, self.byteData, self.classData, self.map];
        }        else         {
            self.callSiteIds=None;
            self.methodHandles=None;
            self.sections=vec![self.header, self.stringIds, self.typeIds, self.protoIds, self.fieldIds, self.methodIds, self.classDefs, self.wordData, self.typeLists, self.stringData, self.byteData, self.classData, self.map];
        }
        self.fileSize=-1;
        self.dumpWidth=79;
    }
    pub fn calcSignature(bytes: &Vec<i8>, len: i32)    {
        let md: MessageDigest;
        try        {
            md=MessageDigest::getInstance_String("SHA-1");
        }        catch(        let ex: NoSuchAlgorithmException)        {
            throw RuntimeException::new(&ex);
        }
        md.update_byte[]_int_int(&bytes, 32, len-32);
        try        {
            let amt: i32 = md.digest_byte[]_int_int(&bytes, 12, 20);
            if amt!=20            {
                throw RuntimeException::new("unexpected digest write: "+amt+" bytes");
            }            
        }        catch(        let ex: DigestException)        {
            throw RuntimeException::new(&ex);
        }
    }
    pub fn calcChecksum(bytes: &Vec<i8>, len: i32)    {
        let a32: Adler32 = Adler32::new();
        a32.update_byte[]_int_int(&bytes, 12, len-12);
        let sum: i32 = a32.getValue() as i32;
        bytes[8]=sum as i8;
        bytes[9]=(sum>>8) as i8;
        bytes[10]=(sum>>16) as i8;
        bytes[11]=(sum>>24) as i8;
    }
    pub fn isEmpty(&self) -> boolean    {
        return self.classDefs.items().isEmpty();
    }
    pub fn getDexOptions(&self) -> DexOptions    {
        return self.dexOptions;
    }
    pub fn add(&self, clazz: &ClassDefItem)    {
        self.classDefs.add(&clazz);
    }
    pub fn getClassOrNull(&self, name: &String) -> ClassDefItem    {
        try        {
            let type: Type = Type::internClassName(&name);
            return (ClassDefItem*)self.classDefs.get(CstType::new(&type_renamed));
        }        catch(        let ex: IllegalArgumentException)        {
            return None;
        }
    }
    pub fn writeTo(&self, out: &OutputStream, humanOut: &Writer, verbose: boolean)    {
        writeTo_OutputStream_Storage_Writer_boolean(&out, None, &humanOut, verbose);
    }
    pub fn writeTo(&self, out: &OutputStream, storage: &Storage, humanOut: &Writer, verbose: boolean)    {
        let annotate: boolean = (humanOut!=None);
        let result: ByteArrayAnnotatedOutput = toDex0(annotate, verbose, &storage);
        if out!=None        {
            out.write_byte[](result.getArray());
        }        
        if annotate        {
            result.writeAnnotationsTo(&humanOut);
        }        
    }
    pub fn writeTo(&self, storage: &Storage) -> ByteArrayAnnotatedOutput    {
        return toDex0(false, false, &storage);
    }
    pub fn toDex(&self, humanOut: &Writer, verbose: boolean) -> Vec<i8>    {
        let annotate: boolean = (humanOut!=None);
        let result: ByteArrayAnnotatedOutput = toDex0(annotate, verbose, None);
        if annotate        {
            result.writeAnnotationsTo(&humanOut);
        }        
        return result.getArray();
    }
    pub fn setDumpWidth(&self, dumpWidth: i32)    {
        if dumpWidth<40        {
            throw IllegalArgumentException::new("dumpWidth < 40");
        }        
        self->dumpWidth=dumpWidth;
    }
    pub fn getFileSize(&self) -> i32    {
        if self.fileSize<0        {
            throw RuntimeException::new("file size not yet known");
        }        
        return self.fileSize;
    }
    pub fn getStringData(&self) -> MixedItemSection    {
        return self.stringData;
    }
    pub fn getWordData(&self) -> MixedItemSection    {
        return self.wordData;
    }
    pub fn getTypeLists(&self) -> MixedItemSection    {
        return self.typeLists;
    }
    pub fn getMap(&self) -> MixedItemSection    {
        return self.map;
    }
    pub fn getStringIds(&self) -> StringIdsSection    {
        return self.stringIds;
    }
    pub fn getClassDefs(&self) -> ClassDefsSection    {
        return self.classDefs;
    }
    pub fn getClassData(&self) -> MixedItemSection    {
        return self.classData;
    }
    pub fn getTypeIds(&self) -> TypeIdsSection    {
        return self.typeIds;
    }
    pub fn getProtoIds(&self) -> ProtoIdsSection    {
        return self.protoIds;
    }
    pub fn getFieldIds(&self) -> FieldIdsSection    {
        return self.fieldIds;
    }
    pub fn getMethodIds(&self) -> MethodIdsSection    {
        return self.methodIds;
    }
    pub fn getMethodHandles(&self) -> MethodHandlesSection    {
        return self.methodHandles;
    }
    pub fn getCallSiteIds(&self) -> CallSiteIdsSection    {
        return self.callSiteIds;
    }
    pub fn getByteData(&self) -> MixedItemSection    {
        return self.byteData;
    }
    pub fn getFirstDataSection(&self) -> Section    {
        return self.wordData;
    }
    pub fn getLastDataSection(&self) -> Section    {
        return self.map;
    }
    pub fn internIfAppropriate(&self, cst: &Constant)    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        if //cst instanceof CstString        {
            self.stringIds.intern_CstString((CstString*)cst);
        }        else         if //cst instanceof CstType        {
            self.typeIds.intern_CstType((CstType*)cst);
        }        else         if //cst instanceof CstBaseMethodRef        {
            self.methodIds.intern((CstBaseMethodRef*)cst);
        }        else         if //cst instanceof CstFieldRef        {
            self.fieldIds.intern((CstFieldRef*)cst);
        }        else         if //cst instanceof CstEnumRef        {
            self.fieldIds.intern(((CstEnumRef*)cst).getFieldRef());
        }        else         if //cst instanceof CstProtoRef        {
            self.protoIds.intern(((CstProtoRef*)cst).getPrototype());
        }        else         if //cst instanceof CstMethodHandle        {
            self.methodHandles.intern((CstMethodHandle*)cst);
        }        
    }
    pub fn findItemOrNull(&self, cst: &Constant) -> IndexedItem    {
        if //cst instanceof CstString        {
            return self.stringIds.get(&cst);
        }        else         if //cst instanceof CstType        {
            return self.typeIds.get(&cst);
        }        else         if //cst instanceof CstBaseMethodRef        {
            return self.methodIds.get(&cst);
        }        else         if //cst instanceof CstFieldRef        {
            return self.fieldIds.get(&cst);
        }        else         if //cst instanceof CstEnumRef        {
            return self.fieldIds.intern(((CstEnumRef*)cst).getFieldRef());
        }        else         if //cst instanceof CstProtoRef        {
            return self.protoIds.get(&cst);
        }        else         if //cst instanceof CstMethodHandle        {
            return self.methodHandles.get(&cst);
        }        else         if //cst instanceof CstCallSiteRef        {
            return self.callSiteIds.get(&cst);
        }        else         {
            return None;
        }
    }
    pub fn toDex0(&self, annotate: boolean, verbose: boolean, storage: &Storage) -> ByteArrayAnnotatedOutput    {
        self.classDefs.prepare();
        self.classData.prepare();
        self.wordData.prepare();
        if self.dexOptions.apiIsSupported(DexFormat::API_METHOD_HANDLES)        {
            self.callSiteIds.prepare();
        }        
        self.byteData.prepare();
        if self.dexOptions.apiIsSupported(DexFormat::API_METHOD_HANDLES)        {
            self.methodHandles.prepare();
        }        
        self.methodIds.prepare();
        self.fieldIds.prepare();
        self.protoIds.prepare();
        self.typeLists.prepare();
        self.typeIds.prepare();
        self.stringIds.prepare();
        self.stringData.prepare();
        self.header.prepare();
        let count: i32 = self.sections.len();
        let offset: i32 = 0;
        for(        let i: i32 = 0;;i<counti += 1)        {
            let one: Section = self.sections[i];
            if (one==self.callSiteIds||one==self.methodHandles)&&one.items().isEmpty()            {
                continue;
            }            
            let placedAt: i32 = one.setFileOffset(offset);
            if placedAt<offset            {
                throw RuntimeException::new("bogus placement for section "+i);
            }            
            try            {
                if one==self.map                {
                    MapItem::addMap(&self.sections, &self.map);
                    self.map.prepare();
                }                
                if //one instanceof MixedItemSection                {
                    ((MixedItemSection*)one).placeItems();
                }                
                offset=placedAt+one.writeSize();
            }            catch(            let ex: RuntimeException)            {
                throw ExceptionWithContext::withContext(&ex, "...while writing section "+i);
            }
        }
        self.fileSize=offset;
        let barr: Vec<i8> = if storage==None { new byte[fileSize] } else { storage.getStorage(self.fileSize) };
                let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new(&barr);
                if annotate                {
                    out.enableAnnotations(self.dumpWidth, verbose);
                }                
                for(                let i: i32 = 0;;i<counti += 1)                {
                    try                    {
                        let one: Section = self.sections[i];
                        if (one==self.callSiteIds||one==self.methodHandles)&&one.items().isEmpty()                        {
                            continue;
                        }                        
                        System::out.printf_String_Object[]("i==%d off=%d size=%d cur=%d\n", i, one.getFileOffset(), one.writeSize(), out.getCursor());
                        let zeroCount: i32 = one.getFileOffset()-out.getCursor();
                        if zeroCount<0                        {
                            throw ExceptionWithContext::new("excess write of "+(-zeroCount));
                        }                        
                        out.writeZeroes(zeroCount);
                        one.writeTo(&out);
                    }                    catch(                    let ex: RuntimeException)                    {
                        let ec: ExceptionWithContext;
                        if //ex instanceof ExceptionWithContext                        {
                            ec=(ExceptionWithContext*)ex;
                        }                        else                         {
                            ec=ExceptionWithContext::new(&ex);
                        }
                        ec.addContext("...while writing section "+i);
                        throw ec;
                    }
                }
                if out.getCursor()!=self.fileSize                {
                    throw RuntimeException::new("foreshortened write");
                }                
                DexFile::calcSignature(&barr, out.getCursor());
                DexFile::calcChecksum(&barr, out.getCursor());
                if annotate                {
                    self.wordData.writeIndexAnnotation(&out, &ItemType::TYPE_CODE_ITEM, "\nmethod code index:\n\n");
                    getStatistics().writeAnnotation(&out);
                    out.finishAnnotating();
                }                
                return out;
            }
            pub fn getStatistics(&self) -> Statistics            {
                let stats: Statistics = Statistics::new();
                for s in self.sections                {
                    stats.addAll(&s);
                }
                return stats;
            }
}
