use crate::helper;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::dex::code::TargetInsn;

struct TargetInsn{
    pub target: CodeAddress,
}
impl TargetInsn{
    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList, target: &CodeAddress)    {
        super(opcode,position,registers);

        if target==None        {
            throw NullPointerException::new("target == null");
        }        
        self->target=target;
    }
    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn    {
        return TargetInsn::new(&opcode, getPosition(), getRegisters(), &self.target);
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return TargetInsn::new(getOpcode(), getPosition(), &registers, &self.target);
    }
    pub fn withNewTargetAndReversed(&self, target: &CodeAddress) -> TargetInsn    {
        let opcode: Dop = getOpcode().getOppositeTest();
        return TargetInsn::new(&opcode, getPosition(), getRegisters(), &target);
    }
    pub fn getTarget(&self) -> CodeAddress    {
        return self.target;
    }
    pub fn getTargetAddress(&self) -> i32    {
        return self.target.getAddress();
    }
    pub fn getTargetOffset(&self) -> i32    {
        return self.target.getAddress()-getAddress();
    }
    pub fn hasTargetOffset(&self) -> boolean    {
        return hasAddress()&&self.target.hasAddress();
    }
    pub fn argString(&self) -> String    {
        if self.target==None        {
            return "????";
        }        
        return self.target.identifierString();
    }
}
