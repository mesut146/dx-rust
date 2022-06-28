use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::rop::cst::CstInterfaceMethodRef;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::rop::cst::StdConstantPool;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::cf::cst::MethodHandleKind;
use crate::com::android::dx::rop::cst::CstDouble;
use crate::com::android::dx::rop::cst::CstNat;

struct ConstantPoolParser{
    pub bytes: ByteArray,
    pub pool: StdConstantPool,
    pub offsets: Vec<i32>,
    pub endOffset: i32,
    pub observer: ParseObserver,
}
impl ConstantPoolParser{
    pub fn new(&self, bytes: &ByteArray)    {
        let size: i32 = bytes.getUnsignedShort(8);
        self->bytes=bytes;
        self->pool=StdConstantPool::new(size);
        self->offsets=new int[size];
        self->endOffset=-1;
    }
    pub fn setObserver(&self, observer: &ParseObserver)    {
        self->observer=observer;
    }
    pub fn getEndOffset(&self) -> i32    {
        parseIfNecessary();
        return self.endOffset;
    }
    pub fn getPool(&self) -> StdConstantPool    {
        parseIfNecessary();
        return self.pool;
    }
    pub fn parseIfNecessary(&self)    {
        if self.endOffset<0        {
            parse();
        }        
    }
    pub fn parse(&self)    {
        determineOffsets();
        if self.observer!=None        {
            self.observer.parsed(&self.bytes, 8, 2, "constant_pool_count: "+Hex::u2(self.offsets.len()));
            self.observer.parsed(&self.bytes, 10, 0, "\nconstant_pool:");
            self.observer.changeIndent(1);
        }        
        let wasUtf8: BitSet = BitSet::new(self.offsets.len());
        for(        let i: i32 = 1;;i<self.offsets.len()i += 1)        {
            let offset: i32 = self.offsets[i];
            if (offset!=0)&&(self.pool.getOrNull(i)==None)            {
                parse0(i, &wasUtf8);
            }            
        }
        if self.observer!=None        {
            for(            let i: i32 = 1;;i<self.offsets.len()i += 1)            {
                let cst: Constant = self.pool.getOrNull(i);
                if cst==None                {
                    continue;
                }                
                let offset: i32 = self.offsets[i];
                let nextOffset: i32 = self.endOffset;
                for(                let j: i32 = i+1;;j<self.offsets.len()j += 1)                {
                    let off: i32 = self.offsets[j];
                    if off!=0                    {
                        nextOffset=off;
                        break;
                    }                    
                }
                let human: String = if wasUtf8.get_int(i) { Hex::u2(i)+": utf8{\""+cst.toHuman()+"\"}" } else { Hex::u2(i)+": "+cst.toString() };
                        self.observer.parsed(&self.bytes, offset, nextOffset-offset, &human);
                    }
                    self.observer.changeIndent(-1);
                    self.observer.parsed(&self.bytes, self.endOffset, 0, "end constant_pool");
                }                
            }
            pub fn determineOffsets(&self)            {
                let at: i32 = 10;
                let lastCategory: i32;
                for(                let i: i32 = 1;;i<self.offsets.len()i+=lastCategory)                {
                    self.offsets[i]=at;
                    let tag: i32 = self.bytes.getUnsignedByte(at);
                    try                    {
                        match tag{ConstantTags::CONSTANT_Integer => ConstantTags::CONSTANT_Float => ConstantTags::CONSTANT_Fieldref => ConstantTags::CONSTANT_Methodref => ConstantTags::CONSTANT_InterfaceMethodref => ConstantTags::CONSTANT_NameAndType =>                             {
                                lastCategory=1;
                                at+=5;
                                break;
                            }ConstantTags::CONSTANT_Long => ConstantTags::CONSTANT_Double =>                             {
                                lastCategory=2;
                                at+=9;
                                break;
                            }ConstantTags::CONSTANT_Class => ConstantTags::CONSTANT_String =>                             {
                                lastCategory=1;
                                at+=3;
                                break;
                            }ConstantTags::CONSTANT_Utf8 =>                             {
                                lastCategory=1;
                                at+=self.bytes.getUnsignedShort(at+1)+3;
                                break;
                            }ConstantTags::CONSTANT_MethodHandle =>                             {
                                lastCategory=1;
                                at+=4;
                                break;
                            }ConstantTags::CONSTANT_MethodType =>                             {
                                lastCategory=1;
                                at+=3;
                                break;
                            }ConstantTags::CONSTANT_InvokeDynamic =>                             {
                                lastCategory=1;
                                at+=5;
                                break;
                            }                        _ => {}                        {
                            throw ParseException::new("unknown tag byte: "+Hex::u1(tag));
                        }                    }
                }                catch(                let ex: ParseException)                {
                    ex.addContext("...while preparsing cst "+Hex::u2(i)+" at offset "+Hex::u4(at));
                    throw ex;
                }
            }
            self.endOffset=at;
        }
        pub fn parse0(&self, idx: i32, wasUtf8: &BitSet) -> Constant        {
            let cst: Constant = self.pool.getOrNull(idx);
            if cst!=None            {
                return cst;
            }            
            let at: i32 = self.offsets[idx];
            try            {
                let tag: i32 = self.bytes.getUnsignedByte(at);
                match tag{ConstantTags::CONSTANT_Utf8 =>                     {
                        cst=parseUtf8(at);
                        wasUtf8.set_int(idx);
                        break;
                    }ConstantTags::CONSTANT_Integer =>                     {
                        let value: i32 = self.bytes.getInt(at+1);
                        cst=CstInteger::make(value);
                        break;
                    }ConstantTags::CONSTANT_Float =>                     {
                        let bits: i32 = self.bytes.getInt(at+1);
                        cst=CstFloat::make(bits);
                        break;
                    }ConstantTags::CONSTANT_Long =>                     {
                        let value: i64 = self.bytes.getLong(at+1);
                        cst=CstLong::make(value);
                        break;
                    }ConstantTags::CONSTANT_Double =>                     {
                        let bits: i64 = self.bytes.getLong(at+1);
                        cst=CstDouble::make(bits);
                        break;
                    }ConstantTags::CONSTANT_Class =>                     {
                        let nameIndex: i32 = self.bytes.getUnsignedShort(at+1);
                        let name: CstString = (CstString*)parse0(nameIndex, &wasUtf8);
                        cst=CstType::new(Type::internClassName(name.getString()));
                        break;
                    }ConstantTags::CONSTANT_String =>                     {
                        let stringIndex: i32 = self.bytes.getUnsignedShort(at+1);
                        cst=parse0(stringIndex, &wasUtf8);
                        break;
                    }ConstantTags::CONSTANT_Fieldref =>                     {
                        let classIndex: i32 = self.bytes.getUnsignedShort(at+1);
                        let type: CstType = (CstType*)parse0(classIndex, &wasUtf8);
                        let natIndex: i32 = self.bytes.getUnsignedShort(at+3);
                        let nat: CstNat = (CstNat*)parse0(natIndex, &wasUtf8);
                        cst=CstFieldRef::new(&type_renamed, &nat);
                        break;
                    }ConstantTags::CONSTANT_Methodref =>                     {
                        let classIndex: i32 = self.bytes.getUnsignedShort(at+1);
                        let type: CstType = (CstType*)parse0(classIndex, &wasUtf8);
                        let natIndex: i32 = self.bytes.getUnsignedShort(at+3);
                        let nat: CstNat = (CstNat*)parse0(natIndex, &wasUtf8);
                        cst=CstMethodRef::new(&type_renamed, &nat);
                        break;
                    }ConstantTags::CONSTANT_InterfaceMethodref =>                     {
                        let classIndex: i32 = self.bytes.getUnsignedShort(at+1);
                        let type: CstType = (CstType*)parse0(classIndex, &wasUtf8);
                        let natIndex: i32 = self.bytes.getUnsignedShort(at+3);
                        let nat: CstNat = (CstNat*)parse0(natIndex, &wasUtf8);
                        cst=CstInterfaceMethodRef::new(&type_renamed, &nat);
                        break;
                    }ConstantTags::CONSTANT_NameAndType =>                     {
                        let nameIndex: i32 = self.bytes.getUnsignedShort(at+1);
                        let name: CstString = (CstString*)parse0(nameIndex, &wasUtf8);
                        let descriptorIndex: i32 = self.bytes.getUnsignedShort(at+3);
                        let descriptor: CstString = (CstString*)parse0(descriptorIndex, &wasUtf8);
                        cst=CstNat::new(&name, &descriptor);
                        break;
                    }ConstantTags::CONSTANT_MethodHandle =>                     {
                        let kind: i32 = self.bytes.getUnsignedByte(at+1);
                        let constantIndex: i32 = self.bytes.getUnsignedShort(at+2);
                        let ref: Constant;
                        match kind{MethodHandleKind::REF_getField => MethodHandleKind::REF_getStatic => MethodHandleKind::REF_putField => MethodHandleKind::REF_putStatic =>                             ref_renamed=(CstFieldRef*)parse0(constantIndex, &wasUtf8);                            break;MethodHandleKind::REF_invokeVirtual => MethodHandleKind::REF_newInvokeSpecial =>                             ref_renamed=(CstMethodRef*)parse0(constantIndex, &wasUtf8);                            break;MethodHandleKind::REF_invokeStatic => MethodHandleKind::REF_invokeSpecial =>                             ref_renamed=parse0(constantIndex, &wasUtf8);                            if !(//ref instanceof CstMethodRef||//ref instanceof CstInterfaceMethodRef)                            {
                                throw ParseException::new("Unsupported ref constant type for MethodHandle "+ref_renamed.getClass());
                            }                                                        break;MethodHandleKind::REF_invokeInterface =>                             ref_renamed=(CstInterfaceMethodRef*)parse0(constantIndex, &wasUtf8);                            break;                        _ => {}                        throw ParseException::new("Unsupported MethodHandle kind: "+kind);                    }
                    let methodHandleType: i32 = ConstantPoolParser::getMethodHandleTypeForKind(kind);
                    cst=CstMethodHandle::make(methodHandleType, &ref_renamed);
                    break;
                }ConstantTags::CONSTANT_MethodType =>                 {
                    let descriptorIndex: i32 = self.bytes.getUnsignedShort(at+1);
                    let descriptor: CstString = (CstString*)parse0(descriptorIndex, &wasUtf8);
                    cst=CstProtoRef::make(&descriptor);
                    break;
                }ConstantTags::CONSTANT_InvokeDynamic =>                 {
                    let bootstrapMethodIndex: i32 = self.bytes.getUnsignedShort(at+1);
                    let natIndex: i32 = self.bytes.getUnsignedShort(at+3);
                    let nat: CstNat = (CstNat*)parse0(natIndex, &wasUtf8);
                    cst=CstInvokeDynamic::make(bootstrapMethodIndex, &nat);
                    break;
                }            _ => {}            {
                throw ParseException::new("unknown tag byte: "+Hex::u1(tag));
            }        }
    }    catch(    let ex: ParseException)    {
        ex.addContext("...while parsing cst "+Hex::u2(idx)+" at offset "+Hex::u4(at));
        throw ex;
    }    catch(    let ex: RuntimeException)    {
        let pe: ParseException = ParseException::new(&ex);
        pe.addContext("...while parsing cst "+Hex::u2(idx)+" at offset "+Hex::u4(at));
        throw pe;
    }
    self.pool.set(idx, &cst);
    return cst;
}
pub fn parseUtf8(&self, at: i32) -> CstString{
    let length: i32 = self.bytes.getUnsignedShort(at+1);
    at+=3;
    let ubytes: ByteArray = self.bytes.slice(at, at+length);
    try    {
        return CstString::new(&ubytes);
    }    catch(    let ex: IllegalArgumentException)    {
        throw ParseException::new(&ex);
    }
}
pub fn getMethodHandleTypeForKind(kind: i32) -> i32{
    match kind{MethodHandleKind::REF_getField =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INSTANCE_GET;MethodHandleKind::REF_getStatic =>         return CstMethodHandle::METHOD_HANDLE_TYPE_STATIC_GET;MethodHandleKind::REF_putField =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INSTANCE_PUT;MethodHandleKind::REF_putStatic =>         return CstMethodHandle::METHOD_HANDLE_TYPE_STATIC_PUT;MethodHandleKind::REF_invokeVirtual =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_INSTANCE;MethodHandleKind::REF_invokeStatic =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_STATIC;MethodHandleKind::REF_invokeSpecial =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_DIRECT;MethodHandleKind::REF_newInvokeSpecial =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_CONSTRUCTOR;MethodHandleKind::REF_invokeInterface =>         return CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_INTERFACE;    }
    throw IllegalArgumentException::new("invalid kind: "+kind);
}
}
