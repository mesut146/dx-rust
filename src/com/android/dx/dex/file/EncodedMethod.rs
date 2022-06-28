use crate::helper;
use crate::com::android::dx::dex::file::EncodedMethod;
use crate::com::android::dx::dex::file::CodeItem;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dex::Leb128;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::file::OffsettedItem;
use crate::com::android::dx::rop::cst::CstNat;

struct EncodedMethod{
    pub method: CstMethodRef,
    pub code: CodeItem,
}
impl EncodedMethod{
    pub fn new(&self, method: &CstMethodRef, accessFlags: i32, code: &DalvCode, throwsList: &TypeList)    {
        super(accessFlags);

        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        self->method=method;
        if code==None        {
            self->code=None;
        }        else         {
            let isStatic: boolean = (accessFlags&AccessFlags::ACC_STATIC)!=0;
            self->code=CodeItem::new(&method, &code, isStatic, &throwsList);
        }
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof EncodedMethod)        {
            return false;
        }        
        return compareTo((EncodedMethod*)other)==0;
    }
    pub fn compareTo(&self, other: &EncodedMethod) -> i32    {
        return self.method.compareTo(&);
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String(getClass().getName());
        sb.append_char('{');
        sb.append_String(Hex::u2(getAccessFlags()));
        sb.append_char(' ');
        sb.append_Object(&self.method);
        if self.code!=None        {
            sb.append_char(' ');
            sb.append_Object(&self.code);
        }        
        sb.append_char('}');
        return sb.toString();
    }
    pub fn addContents(&self, file: &DexFile)    {
        let methodIds: MethodIdsSection = file.getMethodIds();
        let wordData: MixedItemSection = file.getWordData();
        methodIds.intern(&self.method);
        if self.code!=None        {
            wordData.add(&self.code);
        }        
    }
    pub fn toHuman(&self) -> String    {
        return self.method.toHuman();
    }
    pub fn getName(&self) -> CstString    {
        return self.method.getNat().getName();
    }
    pub fn debugPrint(&self, out: &PrintWriter, verbose: boolean)    {
        if self.code==None        {
            out.println_String(getRef().toHuman()+": abstract or native");
        }        else         {
            self.code.debugPrint(&out, "  ", verbose);
        }
    }
    pub fn getRef(&self) -> CstMethodRef    {
        return self.method;
    }
    pub fn encode(&self, file: &DexFile, out: &AnnotatedOutput, lastIndex: i32, dumpSeq: i32) -> i32    {
        let methodIdx: i32 = file.getMethodIds().indexOf(&self.method);
        let diff: i32 = methodIdx-lastIndex;
        let accessFlags: i32 = getAccessFlags();
        let codeOff: i32 = OffsettedItem::getAbsoluteOffsetOr0(&self.code);
        let hasCode: boolean = (codeOff!=0);
        let shouldHaveCode: boolean = (accessFlags&(AccessFlags::ACC_ABSTRACT|AccessFlags::ACC_NATIVE))==0;
        if hasCode!=shouldHaveCode        {
            throw UnsupportedOperationException::new("code vs. access_flags mismatch");
        }        
        if out.annotates()        {
            out.annotate_int_String(0, String::format_String_Object[]("  [%x] %s", dumpSeq, self.method.toHuman()));
            out.annotate_int_String(Leb128::unsignedLeb128Size(diff), "    method_idx:   "+Hex::u4(methodIdx));
            out.annotate_int_String(Leb128::unsignedLeb128Size(accessFlags), "    access_flags: "+AccessFlags::methodString(accessFlags));
            out.annotate_int_String(Leb128::unsignedLeb128Size(codeOff), "    code_off:     "+Hex::u4(codeOff));
        }        
        out.writeUleb128(diff);
        out.writeUleb128(accessFlags);
        out.writeUleb128(codeOff);
        return methodIdx;
    }
}
