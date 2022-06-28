use crate::helper;
use crate::com::android::dx::rop::cst::CstMemberRef;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstNat;

struct MemberIdItem{
    pub cst: CstMemberRef,
}
impl MemberIdItem{
    pub fn new(&self, cst: &CstMemberRef)    {
        super(cst.getDefiningClass());

        self->cst=cst;
    }
    pub fn writeSize(&self) -> i32    {
        return SizeOf::MEMBER_ID_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        super.addContents(file);
        let stringIds: StringIdsSection = file.getStringIds();
        stringIds.intern_CstString(getRef().getNat().getName());
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let typeIds: TypeIdsSection = file.getTypeIds();
        let stringIds: StringIdsSection = file.getStringIds();
        let nat: CstNat = self.cst.getNat();
        let classIdx: i32 = typeIds.indexOf_CstType(getDefiningClass());
        let nameIdx: i32 = stringIds.indexOf(nat.getName());
        let typoidIdx: i32 = getTypoidIdx(&file);
        if out.annotates()        {
            out.annotate_int_String(0, indexString()+' '+self.cst.toHuman());
            out.annotate_int_String(2, "  class_idx: "+Hex::u2(classIdx));
            out.annotate_int_String(2, String::format_String_Object[]("  %-10s %s", getTypoidName()+':', Hex::u2(typoidIdx)));
            out.annotate_int_String(4, "  name_idx:  "+Hex::u4(nameIdx));
        }        
        out.writeShort(classIdx);
        out.writeShort(typoidIdx);
        out.writeInt(nameIdx);
    }
    pub fn getTypoidIdx(&self, file: &DexFile) -> i32;
    pub fn getTypoidName(&self) -> String;
    pub fn getRef(&self) -> CstMemberRef    {
        return self.cst;
    }
}
