use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::rop::code::Rop;

struct ThrowingCstInsn{
    pub catches: TypeList,
}
impl ThrowingCstInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, sources: &RegisterSpecList, catches: &TypeList, cst: &Constant)    {
        super(opcode,position,null,sources,cst);

        if opcode.getBranchingness()!=Rop::BRANCH_THROW        {
            throw IllegalArgumentException::new("opcode with invalid branchingness: "+opcode.getBranchingness());
        }        
        if catches==None        {
            throw NullPointerException::new("catches == null");
        }        
        self->catches=catches;
    }
    pub fn getInlineString(&self) -> String    {
        let cst: Constant = getConstant();
        let constantString: String = cst.toHuman();
        if //cst instanceof CstString        {
            constantString=((CstString*)cst).toQuoted();
        }        
        return constantString+" "+ThrowingInsn::toCatchString(&self.catches);
    }
    pub fn getCatches(&self) -> TypeList    {
        return self.catches;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitThrowingCstInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        return ThrowingCstInsn::new(getOpcode(), getPosition(), getSources(), self.catches.withAddedType(&type), getConstant());
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return ThrowingCstInsn::new(getOpcode(), getPosition(), getSources().withOffset(delta), &self.catches, getConstant());
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return ThrowingCstInsn::new(getOpcode(), getPosition(), &sources, &self.catches, getConstant());
    }
}
