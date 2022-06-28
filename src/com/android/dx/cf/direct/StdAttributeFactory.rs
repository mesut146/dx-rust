use crate::helper;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::cf::attrib::AttDeprecated;
use crate::com::android::dx::cf::attrib::AttSignature;
use crate::com::android::dx::cf::direct::CodeObserver;
use crate::com::android::dx::cf::attrib::AttExceptions;
use crate::com::android::dx::rop::annotation::AnnotationVisibility;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::cf::code::LineNumberList;
use crate::com::android::dx::cf::attrib::AttLineNumberTable;
use crate::com::android::dx::cf::direct::AnnotationParser;
use crate::com::android::dx::cf::code::ByteCatchList;
use crate::com::android::dx::cf::attrib::InnerClassList;
use crate::com::android::dx::cf::attrib::AttRuntimeVisibleParameterAnnotations;
use crate::com::android::dx::cf::attrib::AttSynthetic;
use crate::com::android::dx::cf::code::BootstrapMethodsList;
use crate::com::android::dx::cf::attrib::AttInnerClasses;
use crate::com::android::dx::cf::attrib::AttSourceFile;
use crate::com::android::dx::cf::code::BytecodeArray;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::cf::attrib::AttSourceDebugExtension;
use crate::com::android::dx::cf::attrib::AttLocalVariableTable;
use crate::com::android::dx::cf::attrib::AttLocalVariableTypeTable;
use crate::com::android::dx::cf::code::BootstrapMethodArgumentsList;
use crate::com::android::dx::cf::attrib::AttRuntimeVisibleAnnotations;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::cf::attrib::AttRuntimeInvisibleParameterAnnotations;
use crate::com::android::dx::cf::attrib::AttEnclosingMethod;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::cf::attrib::AttBootstrapMethods;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::attrib::AttRuntimeInvisibleAnnotations;
use crate::com::android::dx::cf::attrib::AttAnnotationDefault;
use crate::com::android::dx::cf::direct::AttributeListParser;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::cf::attrib::AttConstantValue;
use crate::com::android::dx::rop::cst::TypedConstant;
use crate::com::android::dx::cf::iface::StdAttributeList;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::cf::code::LocalVariableList;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::util::ByteArray::MyDataInputStream;
use crate::com::android::dx::cf::attrib::AttCode;
use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::rop::cst::CstNat;

let static THE_ONE: StdAttributeFactory = StdAttributeFactory::new();
struct StdAttributeFactory{
}
impl StdAttributeFactory{
    pub fn new(&self)    {
    }
    pub fn parse0(&self, cf: &DirectClassFile, context: i32, name: &String, offset: i32, length: i32, observer: &ParseObserver) -> Attribute    {
        match context{AttributeFactory::CTX_CLASS =>             {
                if name==AttBootstrapMethods::ATTRIBUTE_NAME                {
                    return bootstrapMethods(&cf, offset, length, &observer);
                }                
                if name==AttDeprecated::ATTRIBUTE_NAME                {
                    return deprecated(&cf, offset, length, &observer);
                }                
                if name==AttEnclosingMethod::ATTRIBUTE_NAME                {
                    return enclosingMethod(&cf, offset, length, &observer);
                }                
                if name==AttInnerClasses::ATTRIBUTE_NAME                {
                    return innerClasses(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeInvisibleAnnotations::ATTRIBUTE_NAME                {
                    return runtimeInvisibleAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeVisibleAnnotations::ATTRIBUTE_NAME                {
                    return runtimeVisibleAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttSynthetic::ATTRIBUTE_NAME                {
                    return synthetic(&cf, offset, length, &observer);
                }                
                if name==AttSignature::ATTRIBUTE_NAME                {
                    return signature(&cf, offset, length, &observer);
                }                
                if name==AttSourceDebugExtension::ATTRIBUTE_NAME                {
                    return sourceDebugExtension(&cf, offset, length, &observer);
                }                
                if name==AttSourceFile::ATTRIBUTE_NAME                {
                    return sourceFile(&cf, offset, length, &observer);
                }                
                break;
            }AttributeFactory::CTX_FIELD =>             {
                if name==AttConstantValue::ATTRIBUTE_NAME                {
                    return constantValue(&cf, offset, length, &observer);
                }                
                if name==AttDeprecated::ATTRIBUTE_NAME                {
                    return deprecated(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeInvisibleAnnotations::ATTRIBUTE_NAME                {
                    return runtimeInvisibleAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeVisibleAnnotations::ATTRIBUTE_NAME                {
                    return runtimeVisibleAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttSignature::ATTRIBUTE_NAME                {
                    return signature(&cf, offset, length, &observer);
                }                
                if name==AttSynthetic::ATTRIBUTE_NAME                {
                    return synthetic(&cf, offset, length, &observer);
                }                
                break;
            }AttributeFactory::CTX_METHOD =>             {
                if name==AttAnnotationDefault::ATTRIBUTE_NAME                {
                    return annotationDefault(&cf, offset, length, &observer);
                }                
                if name==AttCode::ATTRIBUTE_NAME                {
                    return code(&cf, offset, length, &observer);
                }                
                if name==AttDeprecated::ATTRIBUTE_NAME                {
                    return deprecated(&cf, offset, length, &observer);
                }                
                if name==AttExceptions::ATTRIBUTE_NAME                {
                    return exceptions(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeInvisibleAnnotations::ATTRIBUTE_NAME                {
                    return runtimeInvisibleAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeVisibleAnnotations::ATTRIBUTE_NAME                {
                    return runtimeVisibleAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeInvisibleParameterAnnotations::ATTRIBUTE_NAME                {
                    return runtimeInvisibleParameterAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttRuntimeVisibleParameterAnnotations::ATTRIBUTE_NAME                {
                    return runtimeVisibleParameterAnnotations(&cf, offset, length, &observer);
                }                
                if name==AttSignature::ATTRIBUTE_NAME                {
                    return signature(&cf, offset, length, &observer);
                }                
                if name==AttSynthetic::ATTRIBUTE_NAME                {
                    return synthetic(&cf, offset, length, &observer);
                }                
                break;
            }AttributeFactory::CTX_CODE =>             {
                if name==AttLineNumberTable::ATTRIBUTE_NAME                {
                    return lineNumberTable(&cf, offset, length, &observer);
                }                
                if name==AttLocalVariableTable::ATTRIBUTE_NAME                {
                    return localVariableTable(&cf, offset, length, &observer);
                }                
                if name==AttLocalVariableTypeTable::ATTRIBUTE_NAME                {
                    return localVariableTypeTable(&cf, offset, length, &observer);
                }                
                break;
            }        }
        return super.parse0(cf,context,name,offset,length,observer);
    }
    pub fn annotationDefault(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute    {
        if length<2        {
            StdAttributeFactory::throwSeverelyTruncated();
        }        
        let ap: AnnotationParser = AnnotationParser::new(&cf, offset, length, &observer);
        let cst: Constant = ap.parseValueAttribute();
        return AttAnnotationDefault::new(&cst, length);
    }
    pub fn bootstrapMethods(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute    {
        if length<2        {
            return StdAttributeFactory::throwSeverelyTruncated();
        }        
        let bytes: ByteArray = cf.getBytes();
        let numMethods: i32 = bytes.getUnsignedShort(offset);
        if observer!=None        {
            observer.parsed(&bytes, offset, 2, "num_boostrap_methods: "+Hex::u2(numMethods));
        }        
        offset+=2;
        length-=2;
        let methods: BootstrapMethodsList = parseBootstrapMethods(&bytes, cf.getConstantPool(), cf.getThisClass(), numMethods, offset, length, &observer);
        return AttBootstrapMethods::new(&methods);
    }
    pub fn code(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute    {
        if length<12        {
            return StdAttributeFactory::throwSeverelyTruncated();
        }        
        let bytes: ByteArray = cf.getBytes();
        let pool: ConstantPool = cf.getConstantPool();
        let maxStack: i32 = bytes.getUnsignedShort(offset);
        let maxLocals: i32 = bytes.getUnsignedShort(offset+2);
        let codeLength: i32 = bytes.getInt(offset+4);
        let origOffset: i32 = offset;
        if observer!=None        {
            observer.parsed(&bytes, offset, 2, "max_stack: "+Hex::u2(maxStack));
            observer.parsed(&bytes, offset+2, 2, "max_locals: "+Hex::u2(maxLocals));
            observer.parsed(&bytes, offset+4, 4, "code_length: "+Hex::u4(codeLength));
        }        
        offset+=8;
        length-=8;
        if length<(codeLength+4)        {
            return StdAttributeFactory::throwTruncated();
        }        
        let codeOffset: i32 = offset;
        offset+=codeLength;
        length-=codeLength;
        let code: BytecodeArray = BytecodeArray::new(bytes.slice(codeOffset, codeOffset+codeLength), &pool);
        if observer!=None        {
            code.forEach(CodeObserver::new(code.getBytes(), &observer));
        }        
        let exceptionTableLength: i32 = bytes.getUnsignedShort(offset);
        let catches: ByteCatchList = if (exceptionTableLength==0) { ByteCatchList::EMPTY } else { ByteCatchList::new(exceptionTableLength) };
                if observer!=None                {
                    observer.parsed(&bytes, offset, 2, "exception_table_length: "+Hex::u2(exceptionTableLength));
                }                
                offset+=2;
                length-=2;
                if length<(exceptionTableLength*8+2)                {
                    return StdAttributeFactory::throwTruncated();
                }                
                for(                let i: i32 = 0;;i<exceptionTableLengthi += 1)                {
                    if observer!=None                    {
                        observer.changeIndent(1);
                    }                    
                    let startPc: i32 = bytes.getUnsignedShort(offset);
                    let endPc: i32 = bytes.getUnsignedShort(offset+2);
                    let handlerPc: i32 = bytes.getUnsignedShort(offset+4);
                    let catchTypeIdx: i32 = bytes.getUnsignedShort(offset+6);
                    let catchType: CstType = (CstType*)pool.get0Ok(catchTypeIdx);
                    catches.set_int_int_int_int_CstType(i, startPc, endPc, handlerPc, &catchType);
                    if observer!=None                    {
                        observer.parsed(&bytes, offset, 8, Hex::u2(startPc)+".."+Hex::u2(endPc)+" -> "+Hex::u2(handlerPc)+" "+(if (catchType==None) { "<any>" } else { catchType.toHuman() }));
                            }                            
                            offset+=8;
                            length-=8;
                            if observer!=None                            {
                                observer.changeIndent(-1);
                            }                            
                        }
                        catches.setImmutable();
                        let parser: AttributeListParser = AttributeListParser::new(&cf, AttributeFactory::CTX_CODE, offset, self);
                        parser.setObserver(&observer);
                        let attributes: StdAttributeList = parser.getList();
                        attributes.setImmutable();
                        let attributeByteCount: i32 = parser.getEndOffset()-offset;
                        if attributeByteCount!=length                        {
                            return StdAttributeFactory::throwBadLength(attributeByteCount+(offset-origOffset));
                        }                        
                        return AttCode::new(maxStack, maxLocals, &code, &catches, &attributes);
                    }
                    pub fn constantValue(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length!=2                        {
                            return StdAttributeFactory::throwBadLength(2);
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let pool: ConstantPool = cf.getConstantPool();
                        let idx: i32 = bytes.getUnsignedShort(offset);
                        let cst: TypedConstant = (TypedConstant*)pool.get(idx);
                        let result: Attribute = AttConstantValue::new(&cst);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "value: "+cst);
                        }                        
                        return result;
                    }
                    pub fn deprecated(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length!=0                        {
                            return StdAttributeFactory::throwBadLength(0);
                        }                        
                        return AttDeprecated::new();
                    }
                    pub fn enclosingMethod(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length!=4                        {
                            StdAttributeFactory::throwBadLength(4);
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let pool: ConstantPool = cf.getConstantPool();
                        let idx: i32 = bytes.getUnsignedShort(offset);
                        let type: CstType = (CstType*)pool.get(idx);
                        idx=bytes.getUnsignedShort(offset+2);
                        let method: CstNat = (CstNat*)pool.get0Ok(idx);
                        let result: Attribute = AttEnclosingMethod::new(&type_renamed, &method);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "class: "+type_renamed);
                            observer.parsed(&bytes, offset+2, 2, "method: "+DirectClassFile::stringOrNone(&method));
                        }                        
                        return result;
                    }
                    pub fn exceptions(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            return StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let count: i32 = bytes.getUnsignedShort(offset);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "number_of_exceptions: "+Hex::u2(count));
                        }                        
                        offset+=2;
                        length-=2;
                        if length!=(count*2)                        {
                            StdAttributeFactory::throwBadLength((count*2)+2);
                        }                        
                        let list: TypeList = cf.makeTypeList(offset, count);
                        return AttExceptions::new(&list);
                    }
                    pub fn innerClasses(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            return StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let pool: ConstantPool = cf.getConstantPool();
                        let count: i32 = bytes.getUnsignedShort(offset);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "number_of_classes: "+Hex::u2(count));
                        }                        
                        offset+=2;
                        length-=2;
                        if length!=(count*8)                        {
                            StdAttributeFactory::throwBadLength((count*8)+2);
                        }                        
                        let list: InnerClassList = InnerClassList::new(count);
                        for(                        let i: i32 = 0;;i<counti += 1)                        {
                            let innerClassIdx: i32 = bytes.getUnsignedShort(offset);
                            let outerClassIdx: i32 = bytes.getUnsignedShort(offset+2);
                            let nameIdx: i32 = bytes.getUnsignedShort(offset+4);
                            let accessFlags: i32 = bytes.getUnsignedShort(offset+6);
                            let innerClass: CstType = (CstType*)pool.get(innerClassIdx);
                            let outerClass: CstType = (CstType*)pool.get0Ok(outerClassIdx);
                            let name: CstString = (CstString*)pool.get0Ok(nameIdx);
                            list.set(i, &innerClass, &outerClass, &name, accessFlags);
                            if observer!=None                            {
                                observer.parsed(&bytes, offset, 2, "inner_class: "+DirectClassFile::stringOrNone(&innerClass));
                                observer.parsed(&bytes, offset+2, 2, "  outer_class: "+DirectClassFile::stringOrNone(&outerClass));
                                observer.parsed(&bytes, offset+4, 2, "  name: "+DirectClassFile::stringOrNone(&name));
                                observer.parsed(&bytes, offset+6, 2, "  access_flags: "+AccessFlags::innerClassString(accessFlags));
                            }                            
                            offset+=8;
                        }
                        list.setImmutable();
                        return AttInnerClasses::new(&list);
                    }
                    pub fn lineNumberTable(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            return StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let count: i32 = bytes.getUnsignedShort(offset);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "line_number_table_length: "+Hex::u2(count));
                        }                        
                        offset+=2;
                        length-=2;
                        if length!=(count*4)                        {
                            StdAttributeFactory::throwBadLength((count*4)+2);
                        }                        
                        let list: LineNumberList = LineNumberList::new(count);
                        for(                        let i: i32 = 0;;i<counti += 1)                        {
                            let startPc: i32 = bytes.getUnsignedShort(offset);
                            let lineNumber: i32 = bytes.getUnsignedShort(offset+2);
                            list.set_int_int_int(i, startPc, lineNumber);
                            if observer!=None                            {
                                observer.parsed(&bytes, offset, 4, Hex::u2(startPc)+" "+lineNumber);
                            }                            
                            offset+=4;
                        }
                        list.setImmutable();
                        return AttLineNumberTable::new(&list);
                    }
                    pub fn localVariableTable(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            return StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let count: i32 = bytes.getUnsignedShort(offset);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "local_variable_table_length: "+Hex::u2(count));
                        }                        
                        let list: LocalVariableList = parseLocalVariables(bytes.slice(offset+2, offset+length), cf.getConstantPool(), &observer, count, false);
                        return AttLocalVariableTable::new(&list);
                    }
                    pub fn localVariableTypeTable(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            return StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let count: i32 = bytes.getUnsignedShort(offset);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "local_variable_type_table_length: "+Hex::u2(count));
                        }                        
                        let list: LocalVariableList = parseLocalVariables(bytes.slice(offset+2, offset+length), cf.getConstantPool(), &observer, count, true);
                        return AttLocalVariableTypeTable::new(&list);
                    }
                    pub fn parseLocalVariables(&self, bytes: &ByteArray, pool: &ConstantPool, observer: &ParseObserver, count: i32, typeTable: boolean) -> LocalVariableList                    {
                        if bytes.size()!=(count*10)                        {
                            StdAttributeFactory::throwBadLength((count*10)+2);
                        }                        
                        let in: MyDataInputStream = bytes.makeDataInputStream();
                        let list: LocalVariableList = LocalVariableList::new(count);
                        try                        {
                            for(                            let i: i32 = 0;;i<counti += 1)                            {
                                let startPc: i32 = in_renamed.readUnsignedShort();
                                let length: i32 = in_renamed.readUnsignedShort();
                                let nameIdx: i32 = in_renamed.readUnsignedShort();
                                let typeIdx: i32 = in_renamed.readUnsignedShort();
                                let index: i32 = in_renamed.readUnsignedShort();
                                let name: CstString = (CstString*)pool.get(nameIdx);
                                let type: CstString = (CstString*)pool.get(typeIdx);
                                let descriptor: CstString = None;
                                let signature: CstString = None;
                                if typeTable                                {
                                    signature=type_renamed;
                                }                                else                                 {
                                    descriptor=type_renamed;
                                }
                                list.set_int_int_int_CstString_CstString_CstString_int(i, startPc, length, &name, &descriptor, &signature, index);
                                if observer!=None                                {
                                    observer.parsed(&bytes, i*10, 10, Hex::u2(startPc)+".."+Hex::u2(startPc+length)+" "+Hex::u2(index)+" "+name.toHuman()+" "+type_renamed.toHuman());
                                }                                
                            }
                        }                        catch(                        let ex: IOException)                        {
                            throw RuntimeException::new("shouldn't happen", &ex);
                        }
                        list.setImmutable();
                        return list;
                    }
                    pub fn runtimeInvisibleAnnotations(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let ap: AnnotationParser = AnnotationParser::new(&cf, offset, length, &observer);
                        let annotations: Annotations = ap.parseAnnotationAttribute(&AnnotationVisibility::BUILD);
                        return AttRuntimeInvisibleAnnotations::new(&annotations, length);
                    }
                    pub fn runtimeVisibleAnnotations(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let ap: AnnotationParser = AnnotationParser::new(&cf, offset, length, &observer);
                        let annotations: Annotations = ap.parseAnnotationAttribute(&AnnotationVisibility::RUNTIME);
                        return AttRuntimeVisibleAnnotations::new(&annotations, length);
                    }
                    pub fn runtimeInvisibleParameterAnnotations(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let ap: AnnotationParser = AnnotationParser::new(&cf, offset, length, &observer);
                        let list: AnnotationsList = ap.parseParameterAttribute(&AnnotationVisibility::BUILD);
                        return AttRuntimeInvisibleParameterAnnotations::new(&list, length);
                    }
                    pub fn runtimeVisibleParameterAnnotations(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length<2                        {
                            StdAttributeFactory::throwSeverelyTruncated();
                        }                        
                        let ap: AnnotationParser = AnnotationParser::new(&cf, offset, length, &observer);
                        let list: AnnotationsList = ap.parseParameterAttribute(&AnnotationVisibility::RUNTIME);
                        return AttRuntimeVisibleParameterAnnotations::new(&list, length);
                    }
                    pub fn signature(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length!=2                        {
                            StdAttributeFactory::throwBadLength(2);
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let pool: ConstantPool = cf.getConstantPool();
                        let idx: i32 = bytes.getUnsignedShort(offset);
                        let cst: CstString = (CstString*)pool.get(idx);
                        let result: Attribute = AttSignature::new(&cst);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "signature: "+cst);
                        }                        
                        return result;
                    }
                    pub fn sourceDebugExtension(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        let bytes: ByteArray = cf.getBytes().slice(offset, offset+length);
                        let smapString: CstString = CstString::new(&bytes);
                        let result: Attribute = AttSourceDebugExtension::new(&smapString);
                        if observer!=None                        {
                            let decoded: String = smapString.getString();
                            observer.parsed(&bytes, offset, length, "sourceDebugExtension: "+decoded);
                        }                        
                        return result;
                    }
                    pub fn sourceFile(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length!=2                        {
                            StdAttributeFactory::throwBadLength(2);
                        }                        
                        let bytes: ByteArray = cf.getBytes();
                        let pool: ConstantPool = cf.getConstantPool();
                        let idx: i32 = bytes.getUnsignedShort(offset);
                        let cst: CstString = (CstString*)pool.get(idx);
                        let result: Attribute = AttSourceFile::new(&cst);
                        if observer!=None                        {
                            observer.parsed(&bytes, offset, 2, "source: "+cst);
                        }                        
                        return result;
                    }
                    pub fn synthetic(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver) -> Attribute                    {
                        if length!=0                        {
                            return StdAttributeFactory::throwBadLength(0);
                        }                        
                        return AttSynthetic::new();
                    }
                    pub fn throwSeverelyTruncated() -> Attribute                    {
                        throw ParseException::new("severely truncated attribute");
                    }
                    pub fn throwTruncated() -> Attribute                    {
                        throw ParseException::new("truncated attribute");
                    }
                    pub fn throwBadLength(expected: i32) -> Attribute                    {
                        throw ParseException::new("bad attribute length; expected length "+Hex::u4(expected));
                    }
                    pub fn parseBootstrapMethods(&self, bytes: &ByteArray, constantPool: &ConstantPool, declaringClass: &CstType, numMethods: i32, offset: i32, length: i32, observer: &ParseObserver) -> BootstrapMethodsList                    {
                        let methods: BootstrapMethodsList = BootstrapMethodsList::new(numMethods);
                        for(                        let methodIndex: i32 = 0;;methodIndex<numMethods++methodIndex)                        {
                            if length<4                            {
                                StdAttributeFactory::throwTruncated();
                            }                            
                            let methodRef: i32 = bytes.getUnsignedShort(offset);
                            let numArguments: i32 = bytes.getUnsignedShort(offset+2);
                            if observer!=None                            {
                                observer.parsed(&bytes, offset, 2, "bootstrap_method_ref: "+Hex::u2(methodRef));
                                observer.parsed(&bytes, offset+2, 2, "num_bootstrap_arguments: "+Hex::u2(numArguments));
                            }                            
                            offset+=4;
                            length-=4;
                            if length<numArguments*2                            {
                                StdAttributeFactory::throwTruncated();
                            }                            
                            let arguments: BootstrapMethodArgumentsList = BootstrapMethodArgumentsList::new(numArguments);
                            for(                            let argIndex: i32 = 0;;argIndex<numArguments++argIndex, offset+=2, length-=2)                            {
                                let argumentRef: i32 = bytes.getUnsignedShort(offset);
                                if observer!=None                                {
                                    observer.parsed(&bytes, offset, 2, "bootstrap_arguments["+argIndex+"]"+Hex::u2(argumentRef));
                                }                                
                                arguments.set(argIndex, constantPool.get(argumentRef));
                            }
                            arguments.setImmutable();
                            let cstMethodRef: Constant = constantPool.get(methodRef);
                            methods.set_int_CstType_CstMethodHandle_BootstrapMethodArgumentsList(methodIndex, &declaringClass, (CstMethodHandle*)cstMethodRef, &arguments);
                        }
                        methods.setImmutable();
                        if length!=0                        {
                            StdAttributeFactory::throwBadLength(length);
                        }                        
                        return methods;
                    }
}
