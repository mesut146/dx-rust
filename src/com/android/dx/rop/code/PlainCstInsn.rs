use crate::helper;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::rop::code::Rop;

struct PlainCstInsn{
}
impl PlainCstInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, result: &RegisterSpec, sources: &RegisterSpecList, cst: &Constant)    {
        super(opcode,position,result,sources,cst);

        if opcode.getBranchingness()!=Rop::BRANCH_NONE        {
            throw IllegalArgumentException::new("opcode with invalid branchingness: "+opcode.getBranchingness());
        }        
    }
    pub fn getCatches(&self) -> TypeList    {
        return StdTypeList::EMPTY;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitPlainCstInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        throw UnsupportedOperationException::new("unsupported");
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return PlainCstInsn::new(getOpcode(), getPosition(), getResult().withOffset(delta), getSources().withOffset(delta), getConstant());
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return PlainCstInsn::new(getOpcode(), getPosition(), &result, &sources, getConstant());
    }
}
