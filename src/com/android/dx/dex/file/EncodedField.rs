use crate::helper;
use crate::com::android::dx::dex::file::EncodedField;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dex::Leb128;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::rop::cst::CstNat;

struct EncodedField{
    pub field: CstFieldRef,
}
impl EncodedField{
    pub fn new(&self, field: &CstFieldRef, accessFlags: i32)    {
        super(accessFlags);

        if field==None        {
            throw NullPointerException::new("field == null");
        }        
        self->field=field;
    }
    pub fn hashCode(&self) -> i32    {
        return self.field.hashCode();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof EncodedField)        {
            return false;
        }        
        return compareTo((EncodedField*)other)==0;
    }
    pub fn compareTo(&self, other: &EncodedField) -> i32    {
        return self.field.compareTo(&);
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String(getClass().getName());
        sb.append_char('{');
        sb.append_String(Hex::u2(getAccessFlags()));
        sb.append_char(' ');
        sb.append_Object(&self.field);
        sb.append_char('}');
        return sb.toString();
    }
    pub fn addContents(&self, file: &DexFile)    {
        let fieldIds: FieldIdsSection = file.getFieldIds();
        fieldIds.intern(&self.field);
    }
    pub fn getName(&self) -> CstString    {
        return self.field.getNat().getName();
    }
    pub fn toHuman(&self) -> String    {
        return self.field.toHuman();
    }
    pub fn debugPrint(&self, out: &PrintWriter, verbose: boolean)    {
        out.println_String(toString());
    }
    pub fn getRef(&self) -> CstFieldRef    {
        return self.field;
    }
    pub fn encode(&self, file: &DexFile, out: &AnnotatedOutput, lastIndex: i32, dumpSeq: i32) -> i32    {
        let fieldIdx: i32 = file.getFieldIds().indexOf(&self.field);
        let diff: i32 = fieldIdx-lastIndex;
        let accessFlags: i32 = getAccessFlags();
        if out.annotates()        {
            out.annotate_int_String(0, String::format_String_Object[]("  [%x] %s", dumpSeq, self.field.toHuman()));
            out.annotate_int_String(Leb128::unsignedLeb128Size(diff), "    field_idx:    "+Hex::u4(fieldIdx));
            out.annotate_int_String(Leb128::unsignedLeb128Size(accessFlags), "    access_flags: "+AccessFlags::fieldString(accessFlags));
        }        
        out.writeUleb128(diff);
        out.writeUleb128(accessFlags);
        return fieldIdx;
    }
}
