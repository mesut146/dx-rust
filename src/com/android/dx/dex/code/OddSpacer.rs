use crate::helper;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::InsnFormat;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::code::OddSpacer;

struct OddSpacer{
}
impl OddSpacer{
    pub fn new(&self, position: &SourcePosition)    {
        super(position,RegisterSpecList.EMPTY);

    }
    pub fn codeSize(&self) -> i32    {
        return (getAddress()&1);
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
        if codeSize()!=0        {
            out.writeShort(InsnFormat::codeUnit_int_int(Opcodes::NOP, 0));
        }        
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return OddSpacer::new(getPosition());
    }
    pub fn argString(&self) -> String    {
        return None;
    }
    pub fn listingString0(&self, noteIndices: boolean) -> String    {
        if codeSize()==0        {
            return None;
        }        
        return "nop // spacer";
    }
}
