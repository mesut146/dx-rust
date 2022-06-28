use crate::helper;
use crate::com::android::dx::dex::code::CodeAddress;

struct CodeAddress{
    pub bindsClosely: boolean,
}
impl CodeAddress{
    pub fn new(&self, position: &SourcePosition)    {
        this(position,false);

    }
    pub fn new(&self, position: &SourcePosition, bindsClosely: boolean)    {
        super(position);

        self->bindsClosely=bindsClosely;
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return CodeAddress::new(getPosition());
    }
    pub fn argString(&self) -> String    {
        return None;
    }
    pub fn listingString0(&self, noteIndices: boolean) -> String    {
        return "code-address";
    }
    pub fn getBindsClosely(&self) -> boolean    {
        return self.bindsClosely;
    }
}
