use crate::helper;
use crate::com::android::dex::Dex::TypeIndexToDescriptorTable;
use crate::com::android::dex::Mutf8;
use crate::com::android::dex::FieldId;
use crate::com::android::dex::MethodHandle;
use crate::com::android::dex::TypeList;
use crate::com::android::dex::TableOfContents;
use crate::com::android::dex::Code::CatchHandler;
use crate::com::android::dex::Code;
use crate::com::android::dex::ClassData::Method;
use crate::com::android::dex::Dex::ClassDefIterable;
use crate::com::android::dex::Dex::ClassDefIterator;
use crate::com::android::dex::MethodId;
use crate::com::android::dex::ClassData::Field;
use crate::com::android::dex::Leb128;
use crate::com::android::dex::EncodedValue;
use crate::com::android::dex::DexFormat;
use crate::com::android::dex::Dex::TypeIndexToDescriptorIndexTable;
use crate::com::android::dex::Dex::ProtoIdTable;
use crate::com::android::dex::TableOfContents::Section;
use crate::com::android::dex::ClassData;
use crate::com::android::dex::Dex::StringTable;
use crate::com::android::dex::Dex::Section;
use crate::com::android::dex::CallSiteId;
use crate::com::android::dex::Dex::MethodIdTable;
use crate::com::android::dex::SizeOf;
use crate::com::android::dex::Annotation;
use crate::com::android::dex::EncodedValueReader;
use crate::com::android::dex::Code::Try;
use crate::com::android::dex::util::FileUtils;
use crate::com::android::dex::ProtoId;
use crate::com::android::dex::MethodHandle::MethodHandleType;
use crate::com::android::dex::Dex::FieldIdTable;
use crate::com::android::dex::ClassDef;
use crate::com::android::dex::DexException;

let static EMPTY_SHORT_ARRAY: Vec<i16> = new short[0];
struct Dex{
    pub data: ByteBuffer,
    pub tableOfContents: TableOfContents,
    pub nextSectionStart: i32,
    pub strings: StringTable,
    pub typeIds: TypeIndexToDescriptorIndexTable,
    pub typeNames: TypeIndexToDescriptorTable,
    pub protoIds: ProtoIdTable,
    pub fieldIds: FieldIdTable,
    pub methodIds: MethodIdTable,
}
impl Dex{
    pub const CHECKSUM_OFFSET: i32 = 8;
    pub const CHECKSUM_SIZE: i32 = 4;
    pub const SIGNATURE_OFFSET: i32 = Dex::CHECKSUM_OFFSET+Dex::CHECKSUM_SIZE;
    pub const SIGNATURE_SIZE: i32 = 20;
    pub fn new(&self, data: &Vec<i8>)    {
        this(ByteBuffer.wrap(data));

    }
    pub fn new(&self, data: &ByteBuffer)    {
        self->data=data;
        self->data.order_ByteOrder(&ByteOrder::LITTLE_ENDIAN);
        self->tableOfContents.readFrom(self);
    }
    pub fn new(&self, byteCount: i32)    {
        self->data=ByteBuffer::wrap_byte[](new byte[byteCount]);
        self->data.order_ByteOrder(&ByteOrder::LITTLE_ENDIAN);
    }
    pub fn new(&self, in: &InputStream)    {
        try        {
            loadFrom(&in);
        }        finally        {
            in.close();
        }
    }
    pub fn new(&self, file: &File)    {
        if FileUtils::hasArchiveSuffix(file.getName())        {
            let zipFile: ZipFile = ZipFile::new(&file);
            let entry: ZipEntry = zipFile.getEntry_String(&DexFormat::DEX_IN_JAR_NAME);
            if entry!=None            {
                try                {
                    loadFrom(&inputStream);
                }
                zipFile.close();
            }            else             {
                throw DexException::new("Expected "+DexFormat::DEX_IN_JAR_NAME+" in "+file);
            }
        }        else         if file.getName().endsWith(".dex")        {
            try            {
                loadFrom(&inputStream);
            }
        }        else         {
            throw DexException::new("unknown output extension: "+file);
        }
    }
    pub fn loadFrom(&self, in: &InputStream)    {
        let bytesOut: ByteArrayOutputStream = ByteArrayOutputStream::new();
        let buffer: Vec<i8> = new byte[8192];
        let count: i32;
        while (count=in.read_byte[](&buffer))!=-1        {
            bytesOut.write_byte[]_int_int(&buffer, 0, count);
        }
        self->data=ByteBuffer::wrap_byte[](bytesOut.toByteArray());
        self->data.order_ByteOrder(&ByteOrder::LITTLE_ENDIAN);
        self->tableOfContents.readFrom(self);
    }
    pub fn checkBounds(index: i32, length: i32)    {
        if index<0||index>=length        {
            throw IndexOutOfBoundsException::new("index:"+index+", length="+length);
        }        
    }
    pub fn writeTo(&self, out: &OutputStream)    {
        let buffer: Vec<i8> = new byte[8192];
        let data: ByteBuffer = self->data.duplicate();
        data.clear();
        while data.hasRemaining()        {
            let count: i32 = Math::min_int_int(buffer.len(), data.remaining());
            data.get_byte[]_int_int(&buffer, 0, count);
            out.write_byte[]_int_int(&buffer, 0, count);
        }
    }
    pub fn writeTo(&self, dexOut: &File)    {
        try        {
            writeTo_OutputStream(&out);
        }
    }
    pub fn getTableOfContents(&self) -> TableOfContents    {
        return self.tableOfContents;
    }
    pub fn open(&self, position: i32) -> Section    {
        if position<0||position>=self.data.capacity()        {
            throw IllegalArgumentException::new("position="+position+" length="+self.data.capacity());
        }        
        let sectionData: ByteBuffer = self.data.duplicate();
        sectionData.order_ByteOrder(&ByteOrder::LITTLE_ENDIAN);
        sectionData.position(position);
        sectionData.limit(self.data.capacity());
        return Section::new("section", &sectionData, self);
    }
    pub fn appendSection(&self, maxByteCount: i32, name: &String) -> Section    {
        if (maxByteCount&3)!=0        {
            throw IllegalStateException::new("Not four byte aligned!");
        }        
        let limit: i32 = self.nextSectionStart+maxByteCount;
        let sectionData: ByteBuffer = self.data.duplicate();
        sectionData.order_ByteOrder(&ByteOrder::LITTLE_ENDIAN);
        sectionData.position(self.nextSectionStart);
        sectionData.limit(limit);
        let result: Section = Section::new(&name, &sectionData, self);
        self.nextSectionStart=limit;
        return result;
    }
    pub fn getLength(&self) -> i32    {
        return self.data.capacity();
    }
    pub fn getNextSectionStart(&self) -> i32    {
        return self.nextSectionStart;
    }
    pub fn getBytes(&self) -> Vec<i8>    {
        let data: ByteBuffer = self->data.duplicate();
        let result: Vec<i8> = new byte[data.capacity()];
        data.position(0);
        data.get_byte[](&result);
        return result;
    }
    pub fn strings(&self) -> List<String>    {
        return self.strings;
    }
    pub fn typeIds(&self) -> List<Integer>    {
        return self.typeIds;
    }
    pub fn typeNames(&self) -> List<String>    {
        return self.typeNames;
    }
    pub fn protoIds(&self) -> List<ProtoId>    {
        return self.protoIds;
    }
    pub fn fieldIds(&self) -> List<FieldId>    {
        return self.fieldIds;
    }
    pub fn methodIds(&self) -> List<MethodId>    {
        return self.methodIds;
    }
    pub fn classDefs(&self) -> Iterable<ClassDef>    {
        return ClassDefIterable::new(, self);
    }
    pub fn readTypeList(&self, offset: i32) -> TypeList    {
        if offset==0        {
            return TypeList::EMPTY;
        }        
        return open(offset).readTypeList();
    }
    pub fn readClassData(&self, classDef: &ClassDef) -> ClassData    {
        let offset: i32 = classDef.getClassDataOffset();
        if offset==0        {
            throw IllegalArgumentException::new("offset == 0");
        }        
        return open(offset).readClassData();
    }
    pub fn readCode(&self, method: &ClassData.Method) -> Code    {
        let offset: i32 = method.getCodeOffset();
        if offset==0        {
            throw IllegalArgumentException::new("offset == 0");
        }        
        return open(offset).readCode();
    }
    pub fn computeSignature(&self) -> Vec<i8>    {
        let digest: MessageDigest;
        try        {
            digest=MessageDigest::getInstance_String("SHA-1");
        }        catch(        let e: NoSuchAlgorithmException)        {
            throw AssertionError::new();
        }
        let buffer: Vec<i8> = new byte[8192];
        let data: ByteBuffer = self->data.duplicate();
        data.limit(data.capacity());
        data.position(Dex::SIGNATURE_OFFSET+Dex::SIGNATURE_SIZE);
        while data.hasRemaining()        {
            let count: i32 = Math::min_int_int(buffer.len(), data.remaining());
            data.get_byte[]_int_int(&buffer, 0, count);
            digest.update_byte[]_int_int(&buffer, 0, count);
        }
        return digest.digest();
    }
    pub fn computeChecksum(&self) -> i32    {
        let adler32: Adler32 = Adler32::new();
        let buffer: Vec<i8> = new byte[8192];
        let data: ByteBuffer = self->data.duplicate();
        data.limit(data.capacity());
        data.position(Dex::CHECKSUM_OFFSET+Dex::CHECKSUM_SIZE);
        while data.hasRemaining()        {
            let count: i32 = Math::min_int_int(buffer.len(), data.remaining());
            data.get_byte[]_int_int(&buffer, 0, count);
            adler32.update_byte[]_int_int(&buffer, 0, count);
        }
        return adler32.getValue() as i32;
    }
    pub fn writeHashes(&self)    {
        open(Dex::SIGNATURE_OFFSET).write_byte[](computeSignature());
        open(Dex::CHECKSUM_OFFSET).writeInt(computeChecksum());
    }
    pub fn descriptorIndexFromTypeIndex(&self, typeIndex: i32) -> i32    {
        Dex::checkBounds(typeIndex, );
        let position: i32 = +(SizeOf::TYPE_ID_ITEM*typeIndex);
        return self.data.getInt_int(position);
    }
}
