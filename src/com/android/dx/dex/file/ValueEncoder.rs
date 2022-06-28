use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::CstAnnotation;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::file::MethodHandlesSection;
use crate::com::android::dx::rop::cst::CstEnumRef;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::dex::file::ProtoIdsSection;
use crate::com::android::dx::rop::cst::CstArray::List;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::rop::cst::CstChar;
use crate::com::android::dx::rop::cst::CstShort;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dex::EncodedValueCodec;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstArray;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstBoolean;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::cst::CstDouble;
use crate::com::android::dx::rop::cst::CstByte;

struct ValueEncoder{
    pub file: DexFile,
    pub out: AnnotatedOutput,
}
impl ValueEncoder{
    pub const VALUE_BYTE: i32 = 0x00;
    pub const VALUE_SHORT: i32 = 0x02;
    pub const VALUE_CHAR: i32 = 0x03;
    pub const VALUE_INT: i32 = 0x04;
    pub const VALUE_LONG: i32 = 0x06;
    pub const VALUE_FLOAT: i32 = 0x10;
    pub const VALUE_DOUBLE: i32 = 0x11;
    pub const VALUE_METHOD_TYPE: i32 = 0x15;
    pub const VALUE_METHOD_HANDLE: i32 = 0x16;
    pub const VALUE_STRING: i32 = 0x17;
    pub const VALUE_TYPE: i32 = 0x18;
    pub const VALUE_FIELD: i32 = 0x19;
    pub const VALUE_METHOD: i32 = 0x1a;
    pub const VALUE_ENUM: i32 = 0x1b;
    pub const VALUE_ARRAY: i32 = 0x1c;
    pub const VALUE_ANNOTATION: i32 = 0x1d;
    pub const VALUE_NULL: i32 = 0x1e;
    pub const VALUE_BOOLEAN: i32 = 0x1f;
    pub fn new(&self, file: &DexFile, out: &AnnotatedOutput)    {
        if file==None        {
            throw NullPointerException::new("file == null");
        }        
        if out==None        {
            throw NullPointerException::new("out == null");
        }        
        self->file=file;
        self->out=out;
    }
    pub fn writeConstant(&self, cst: &Constant)    {
        let type: i32 = ValueEncoder::constantToValueType(&cst);
        let arg: i32;
        match type_renamed{ValueEncoder::VALUE_BYTE => ValueEncoder::VALUE_SHORT => ValueEncoder::VALUE_INT => ValueEncoder::VALUE_LONG =>             {
                let value: i64 = ((CstLiteralBits*)cst).getLongBits();
                EncodedValueCodec::writeSignedIntegralValue(&self.out, type_renamed, value);
                break;
            }ValueEncoder::VALUE_CHAR =>             {
                let value: i64 = ((CstLiteralBits*)cst).getLongBits();
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, value);
                break;
            }ValueEncoder::VALUE_FLOAT =>             {
                let value: i64 = ((CstFloat*)cst).getLongBits()<<32;
                EncodedValueCodec::writeRightZeroExtendedValue(&self.out, type_renamed, value);
                break;
            }ValueEncoder::VALUE_DOUBLE =>             {
                let value: i64 = ((CstDouble*)cst).getLongBits();
                EncodedValueCodec::writeRightZeroExtendedValue(&self.out, type_renamed, value);
                break;
            }ValueEncoder::VALUE_METHOD_TYPE =>             {
                let index: i32 = self.file.getProtoIds().indexOf(((CstProtoRef*)cst).getPrototype());
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_METHOD_HANDLE =>             {
                let index: i32 = self.file.getMethodHandles().indexOf((CstMethodHandle*)cst);
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_STRING =>             {
                let index: i32 = self.file.getStringIds().indexOf((CstString*)cst);
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_TYPE =>             {
                let index: i32 = self.file.getTypeIds().indexOf_CstType((CstType*)cst);
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_FIELD =>             {
                let index: i32 = self.file.getFieldIds().indexOf((CstFieldRef*)cst);
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_METHOD =>             {
                let index: i32 = self.file.getMethodIds().indexOf((CstMethodRef*)cst);
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_ENUM =>             {
                let fieldRef: CstFieldRef = ((CstEnumRef*)cst).getFieldRef();
                let index: i32 = self.file.getFieldIds().indexOf(&fieldRef);
                EncodedValueCodec::writeUnsignedIntegralValue(&self.out, type_renamed, index as i64);
                break;
            }ValueEncoder::VALUE_ARRAY =>             {
                self.out.writeByte(type_renamed);
                writeArray((CstArray*)cst, false);
                break;
            }ValueEncoder::VALUE_ANNOTATION =>             {
                self.out.writeByte(type_renamed);
                writeAnnotation(((CstAnnotation*)cst).getAnnotation(), false);
                break;
            }ValueEncoder::VALUE_NULL =>             {
                self.out.writeByte(type_renamed);
                break;
            }ValueEncoder::VALUE_BOOLEAN =>             {
                let value: i32 = ((CstBoolean*)cst).getIntBits();
                self.out.writeByte(type_renamed|(value<<5));
                break;
            }        _ => {}        {
            throw RuntimeException::new("Shouldn't happen");
        }    }
}
pub fn constantToValueType(cst: &Constant) -> i32{
    if //cst instanceof CstByte    {
        return ValueEncoder::VALUE_BYTE;
    }    else     if //cst instanceof CstShort    {
        return ValueEncoder::VALUE_SHORT;
    }    else     if //cst instanceof CstChar    {
        return ValueEncoder::VALUE_CHAR;
    }    else     if //cst instanceof CstInteger    {
        return ValueEncoder::VALUE_INT;
    }    else     if //cst instanceof CstLong    {
        return ValueEncoder::VALUE_LONG;
    }    else     if //cst instanceof CstFloat    {
        return ValueEncoder::VALUE_FLOAT;
    }    else     if //cst instanceof CstDouble    {
        return ValueEncoder::VALUE_DOUBLE;
    }    else     if //cst instanceof CstProtoRef    {
        return ValueEncoder::VALUE_METHOD_TYPE;
    }    else     if //cst instanceof CstMethodHandle    {
        return ValueEncoder::VALUE_METHOD_HANDLE;
    }    else     if //cst instanceof CstString    {
        return ValueEncoder::VALUE_STRING;
    }    else     if //cst instanceof CstType    {
        return ValueEncoder::VALUE_TYPE;
    }    else     if //cst instanceof CstFieldRef    {
        return ValueEncoder::VALUE_FIELD;
    }    else     if //cst instanceof CstMethodRef    {
        return ValueEncoder::VALUE_METHOD;
    }    else     if //cst instanceof CstEnumRef    {
        return ValueEncoder::VALUE_ENUM;
    }    else     if //cst instanceof CstArray    {
        return ValueEncoder::VALUE_ARRAY;
    }    else     if //cst instanceof CstAnnotation    {
        return ValueEncoder::VALUE_ANNOTATION;
    }    else     if //cst instanceof CstKnownNull    {
        return ValueEncoder::VALUE_NULL;
    }    else     if //cst instanceof CstBoolean    {
        return ValueEncoder::VALUE_BOOLEAN;
    }    else     {
        throw RuntimeException::new("Shouldn't happen");
    }
}
pub fn writeArray(&self, array: &CstArray, topLevel: boolean){
    let annotates: boolean = topLevel&&self.out.annotates();
    let list: List = ((CstArray*)array).getList();
    let size: i32 = list.size();
    if annotates    {
        self.out.annotate_String("  size: "+Hex::u4(size));
    }    
    self.out.writeUleb128(size);
    for(    let i: i32 = 0;;i<sizei += 1)    {
        let cst: Constant = list.get(i);
        if annotates        {
            self.out.annotate_String("  ["+Integer::toHexString(i)+"] "+ValueEncoder::constantToHuman(&cst));
        }        
        writeConstant(&cst);
    }
    if annotates    {
        self.out.endAnnotation();
    }    
}
pub fn writeAnnotation(&self, annotation: &Annotation, topLevel: boolean){
    let annotates: boolean = topLevel&&self.out.annotates();
    let stringIds: StringIdsSection = self.file.getStringIds();
    let typeIds: TypeIdsSection = self.file.getTypeIds();
    let type: CstType = annotation.getType();
    let typeIdx: i32 = typeIds.indexOf_CstType(&type_renamed);
    if annotates    {
        self.out.annotate_String("  type_idx: "+Hex::u4(typeIdx)+" // "+type_renamed.toHuman());
    }    
    self.out.writeUleb128(typeIds.indexOf_CstType(annotation.getType()));
    let pairs: Collection<NameValuePair> = annotation.getNameValuePairs();
    let size: i32 = pairs.size();
    if annotates    {
        self.out.annotate_String("  size: "+Hex::u4(size));
    }    
    self.out.writeUleb128(size);
    let at: i32 = 0;
    for pair in pairs    {
        let name: CstString = pair.getName();
        let nameIdx: i32 = stringIds.indexOf(&name);
        let value: Constant = pair.getValue();
        if annotates        {
            self.out.annotate_int_String(0, "  elements["+at+"]:");
            at += 1;
            self.out.annotate_String("    name_idx: "+Hex::u4(nameIdx)+" // "+name.toHuman());
        }        
        self.out.writeUleb128(nameIdx);
        if annotates        {
            self.out.annotate_String("    value: "+ValueEncoder::constantToHuman(&value));
        }        
        writeConstant(&value);
    }
    if annotates    {
        self.out.endAnnotation();
    }    
}
pub fn constantToHuman(cst: &Constant) -> String{
    let type: i32 = ValueEncoder::constantToValueType(&cst);
    if type_renamed==ValueEncoder::VALUE_NULL    {
        return "null";
    }    
    let sb: StringBuilder = StringBuilder::new();
    sb.append_String(cst.typeName());
    sb.append_char(' ');
    sb.append_String(cst.toHuman());
    return sb.toString();
}
pub fn addContents(file: &DexFile, annotation: &Annotation){
    let typeIds: TypeIdsSection = file.getTypeIds();
    let stringIds: StringIdsSection = file.getStringIds();
    typeIds.intern_CstType(annotation.getType());
    for pair in annotation.getNameValuePairs()    {
        stringIds.intern_CstString(pair.getName());
        ValueEncoder::addContents_DexFile_Constant(&file, pair.getValue());
    }
}
pub fn addContents(file: &DexFile, cst: &Constant){
    if //cst instanceof CstAnnotation    {
        ValueEncoder::addContents_DexFile_Annotation(&file, ((CstAnnotation*)cst).getAnnotation());
    }    else     if //cst instanceof CstArray    {
        let list: List = ((CstArray*)cst).getList();
        let size: i32 = list.size();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            ValueEncoder::addContents_DexFile_Constant(&file, list.get(i));
        }
    }    else     {
        file.internIfAppropriate(&cst);
    }
}
}
