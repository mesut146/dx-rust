use crate::helper;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::code::SwitchInsn;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::Rop;

struct SwitchInsn{
    pub cases: IntList,
}
impl SwitchInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, result: &RegisterSpec, sources: &RegisterSpecList, cases: &IntList)    {
        super(opcode,position,result,sources);

        if opcode.getBranchingness()!=Rop::BRANCH_SWITCH        {
            throw IllegalArgumentException::new("bogus branchingness");
        }        
        if cases==None        {
            throw NullPointerException::new("cases == null");
        }        
        self->cases=cases;
    }
    pub fn getInlineString(&self) -> String    {
        return self.cases.toString();
    }
    pub fn getCatches(&self) -> TypeList    {
        return StdTypeList::EMPTY;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitSwitchInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        throw UnsupportedOperationException::new("unsupported");
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return SwitchInsn::new(getOpcode(), getPosition(), getResult().withOffset(delta), getSources().withOffset(delta), &self.cases);
    }
    pub fn contentEquals(&self, b: &Insn) -> boolean    {
        return false;
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return SwitchInsn::new(getOpcode(), getPosition(), &result, &sources, &self.cases);
    }
    pub fn getCases(&self) -> IntList    {
        return self.cases;
    }
}
