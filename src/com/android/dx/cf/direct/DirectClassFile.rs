use crate::helper;
use crate::com::android::dx::cf::direct::MethodListParser;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::cf::code::BootstrapMethodsList;
use crate::com::android::dx::cf::attrib::AttSourceFile;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::cf::attrib::AttBootstrapMethods;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::StdConstantPool;
use crate::com::android::dx::cf::direct::DirectClassFile::DcfTypeList;
use crate::com::android::dx::cf::direct::FieldListParser;
use crate::com::android::dx::cf::direct::AttributeListParser;
use crate::com::android::dx::cf::direct::AttributeFactory;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::cf::cst::ConstantPoolParser;
use crate::com::android::dx::cf::iface::StdAttributeList;

struct DirectClassFile{
    pub filePath: String,
    pub bytes: ByteArray,
    pub strictParse: boolean,
    pub pool: StdConstantPool,
    pub accessFlags: i32,
    pub thisClass: CstType,
    pub superClass: CstType,
    pub interfaces: TypeList,
    pub fields: FieldList,
    pub methods: MethodList,
    pub attributes: StdAttributeList,
    pub attributeFactory: AttributeFactory,
    pub observer: ParseObserver,
}
impl DirectClassFile{
    pub const CLASS_FILE_MAGIC: i32 = 0xcafebabe;
    pub const CLASS_FILE_MIN_MAJOR_VERSION: i32 = 45;
    pub const CLASS_FILE_MAX_MAJOR_VERSION: i32 = 53;
    pub const CLASS_FILE_MAX_MINOR_VERSION: i32 = 0;
    pub fn stringOrNone(obj: &Object) -> String    {
        if obj==None        {
            return "(none)";
        }        
        return obj.toString();
    }
    pub fn new(&self, bytes: &ByteArray, filePath: &String, strictParse: boolean)    {
        if bytes==None        {
            throw NullPointerException::new("bytes == null");
        }        
        if filePath==None        {
            throw NullPointerException::new("filePath == null");
        }        
        self->filePath=filePath;
        self->bytes=bytes;
        self->strictParse=strictParse;
        self->accessFlags=-1;
    }
    pub fn new(&self, bytes: &Vec<i8>, filePath: &String, strictParse: boolean)    {
        this(new ByteArray(bytes),filePath,strictParse);

    }
    pub fn setObserver(&self, observer: &ParseObserver)    {
        self->observer=observer;
    }
    pub fn setAttributeFactory(&self, attributeFactory: &AttributeFactory)    {
        if attributeFactory==None        {
            throw NullPointerException::new("attributeFactory == null");
        }        
        self->attributeFactory=attributeFactory;
    }
    pub fn getFilePath(&self) -> String    {
        return self.filePath;
    }
    pub fn getBytes(&self) -> ByteArray    {
        return self.bytes;
    }
    pub fn getMagic(&self) -> i32    {
        parseToInterfacesIfNecessary();
        return getMagic0();
    }
    pub fn getMinorVersion(&self) -> i32    {
        parseToInterfacesIfNecessary();
        return getMinorVersion0();
    }
    pub fn getMajorVersion(&self) -> i32    {
        parseToInterfacesIfNecessary();
        return getMajorVersion0();
    }
    pub fn getAccessFlags(&self) -> i32    {
        parseToInterfacesIfNecessary();
        return self.accessFlags;
    }
    pub fn getThisClass(&self) -> CstType    {
        parseToInterfacesIfNecessary();
        return self.thisClass;
    }
    pub fn getSuperclass(&self) -> CstType    {
        parseToInterfacesIfNecessary();
        return self.superClass;
    }
    pub fn getConstantPool(&self) -> ConstantPool    {
        parseToInterfacesIfNecessary();
        return self.pool;
    }
    pub fn getInterfaces(&self) -> TypeList    {
        parseToInterfacesIfNecessary();
        return self.interfaces;
    }
    pub fn getFields(&self) -> FieldList    {
        parseToEndIfNecessary();
        return self.fields;
    }
    pub fn getMethods(&self) -> MethodList    {
        parseToEndIfNecessary();
        return self.methods;
    }
    pub fn getAttributes(&self) -> AttributeList    {
        parseToEndIfNecessary();
        return self.attributes;
    }
    pub fn getBootstrapMethods(&self) -> BootstrapMethodsList    {
        let bootstrapMethodsAttribute: AttBootstrapMethods = (AttBootstrapMethods*)getAttributes().findFirst(&AttBootstrapMethods::ATTRIBUTE_NAME);
        if bootstrapMethodsAttribute!=None        {
            return bootstrapMethodsAttribute.getBootstrapMethods();
        }        else         {
            return BootstrapMethodsList::EMPTY;
        }
    }
    pub fn getSourceFile(&self) -> CstString    {
        let attribs: AttributeList = getAttributes();
        let attSf: Attribute = attribs.findFirst(&AttSourceFile::ATTRIBUTE_NAME);
        if //attSf instanceof AttSourceFile        {
            return ((AttSourceFile*)attSf).getSourceFile();
        }        
        return None;
    }
    pub fn makeTypeList(&self, offset: i32, size: i32) -> TypeList    {
        if size==0        {
            return StdTypeList::EMPTY;
        }        
        if self.pool==None        {
            throw IllegalStateException::new("pool not yet initialized");
        }        
        return DcfTypeList::new(&self.bytes, offset, size, &self.pool, &self.observer);
    }
    pub fn getMagic0(&self) -> i32    {
        return self.bytes.getInt(0);
    }
    pub fn getMinorVersion0(&self) -> i32    {
        return self.bytes.getUnsignedShort(4);
    }
    pub fn getMajorVersion0(&self) -> i32    {
        return self.bytes.getUnsignedShort(6);
    }
    pub fn parseToInterfacesIfNecessary(&self)    {
        if self.accessFlags==-1        {
            parse();
        }        
    }
    pub fn parseToEndIfNecessary(&self)    {
        if self.attributes==None        {
            parse();
        }        
    }
    pub fn parse(&self)    {
        try        {
            parse0();
        }        catch(        let ex: ParseException)        {
            ex.addContext("...while parsing "+self.filePath);
            throw ex;
        }        catch(        let ex: RuntimeException)        {
            let pe: ParseException = ParseException::new(&ex);
            pe.addContext("...while parsing "+self.filePath);
            throw pe;
        }
    }
    pub fn isGoodMagic(&self, magic: i32) -> boolean    {
        return magic==DirectClassFile::CLASS_FILE_MAGIC;
    }
    pub fn isGoodVersion(&self, minorVersion: i32, majorVersion: i32) -> boolean    {
        if minorVersion>=0        {
            if majorVersion==DirectClassFile::CLASS_FILE_MAX_MAJOR_VERSION            {
                if minorVersion<=DirectClassFile::CLASS_FILE_MAX_MINOR_VERSION                {
                    return true;
                }                
            }            else             if majorVersion<DirectClassFile::CLASS_FILE_MAX_MAJOR_VERSION&&majorVersion>=DirectClassFile::CLASS_FILE_MIN_MAJOR_VERSION            {
                return true;
            }            
        }        
        return false;
    }
    pub fn parse0(&self)    {
        if self.bytes.size()<10        {
            throw ParseException::new("severely truncated class file");
        }        
        if self.observer!=None        {
            self.observer.parsed(&self.bytes, 0, 0, "begin classfile");
            self.observer.parsed(&self.bytes, 0, 4, "magic: "+Hex::u4(getMagic0()));
            self.observer.parsed(&self.bytes, 4, 2, "minor_version: "+Hex::u2(getMinorVersion0()));
            self.observer.parsed(&self.bytes, 6, 2, "major_version: "+Hex::u2(getMajorVersion0()));
        }        
        if self.strictParse        {
            if !isGoodMagic(getMagic0())            {
                throw ParseException::new("bad class file magic ("+Hex::u4(getMagic0())+")");
            }            
            if !isGoodVersion(getMinorVersion0(), getMajorVersion0())            {
                throw ParseException::new("unsupported class file version "+getMajorVersion0()+"."+getMinorVersion0());
            }            
        }        
        let cpParser: ConstantPoolParser = ConstantPoolParser::new(&self.bytes);
        cpParser.setObserver(&self.observer);
        self.pool=cpParser.getPool();
        self.pool.setImmutable();
        let at: i32 = cpParser.getEndOffset();
        let accessFlags: i32 = self.bytes.getUnsignedShort(at);
        let cpi: i32 = self.bytes.getUnsignedShort(at+2);
        self.thisClass=(CstType*)self.pool.get(cpi);
        cpi=self.bytes.getUnsignedShort(at+4);
        self.superClass=(CstType*)self.pool.get0Ok(cpi);
        let count: i32 = self.bytes.getUnsignedShort(at+6);
        if self.observer!=None        {
            self.observer.parsed(&self.bytes, at, 2, "access_flags: "+AccessFlags::classString(accessFlags));
            self.observer.parsed(&self.bytes, at+2, 2, "this_class: "+self.thisClass);
            self.observer.parsed(&self.bytes, at+4, 2, "super_class: "+DirectClassFile::stringOrNone(&self.superClass));
            self.observer.parsed(&self.bytes, at+6, 2, "interfaces_count: "+Hex::u2(count));
            if count!=0            {
                self.observer.parsed(&self.bytes, at+8, 0, "interfaces:");
            }            
        }        
        at+=8;
        self.interfaces=makeTypeList(at, count);
        at+=count*2;
        if self.strictParse        {
            let thisClassName: String = self.thisClass.getClassType().getClassName();
            if !(self.filePath.endsWith(".class")&&self.filePath.startsWith_String(&thisClassName)&&(self.filePath.length()==(thisClassName.length()+6)))            {
                throw ParseException::new("class name ("+thisClassName+") does not match path ("+self.filePath+")");
            }            
        }        
        self->accessFlags=accessFlags;
        let flParser: FieldListParser = FieldListParser::new(self, &self.thisClass, at, &self.attributeFactory);
        flParser.setObserver(&self.observer);
        self.fields=flParser.getList();
        at=flParser.getEndOffset();
        let mlParser: MethodListParser = MethodListParser::new(self, &self.thisClass, at, &self.attributeFactory);
        mlParser.setObserver(&self.observer);
        self.methods=mlParser.getList();
        at=mlParser.getEndOffset();
        let alParser: AttributeListParser = AttributeListParser::new(self, AttributeFactory::CTX_CLASS, at, &self.attributeFactory);
        alParser.setObserver(&self.observer);
        self.attributes=alParser.getList();
        self.attributes.setImmutable();
        at=alParser.getEndOffset();
        if at!=self.bytes.size()        {
            throw ParseException::new("extra bytes at end of class file, "+"at offset "+Hex::u4(at));
        }        
        if self.observer!=None        {
            self.observer.parsed(&self.bytes, at, 0, "end classfile");
        }        
    }
}
