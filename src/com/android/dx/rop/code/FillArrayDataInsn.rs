use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::code::FillArrayDataInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::Rop;

struct FillArrayDataInsn{
    pub initValues: ArrayList<Constant>,
    pub arrayType: Constant,
}
impl FillArrayDataInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, sources: &RegisterSpecList, initValues: &ArrayList<Constant>, cst: &Constant)    {
        super(opcode,position,null,sources);

        if opcode.getBranchingness()!=Rop::BRANCH_NONE        {
            throw IllegalArgumentException::new("opcode with invalid branchingness: "+opcode.getBranchingness());
        }        
        self->initValues=initValues;
        self->arrayType=cst;
    }
    pub fn getCatches(&self) -> TypeList    {
        return StdTypeList::EMPTY;
    }
    pub fn getInitValues(&self) -> ArrayList<Constant>    {
        return self.initValues;
    }
    pub fn getConstant(&self) -> Constant    {
        return self.arrayType;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitFillArrayDataInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        throw UnsupportedOperationException::new("unsupported");
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return FillArrayDataInsn::new(getOpcode(), getPosition(), getSources().withOffset(delta), &self.initValues, &self.arrayType);
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return FillArrayDataInsn::new(getOpcode(), getPosition(), &sources, &self.initValues, &self.arrayType);
    }
}
