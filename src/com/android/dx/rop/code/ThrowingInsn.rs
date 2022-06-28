use crate::helper;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::Rop;

struct ThrowingInsn{
    pub catches: TypeList,
}
impl ThrowingInsn{
    pub fn toCatchString(catches: &TypeList) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String("catch");
        let sz: i32 = catches.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            sb.append_String(" ");
            sb.append_String(catches.getType(i).toHuman());
        }
        return sb.toString();
    }
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, sources: &RegisterSpecList, catches: &TypeList)    {
        super(opcode,position,null,sources);

        if opcode.getBranchingness()!=Rop::BRANCH_THROW        {
            throw IllegalArgumentException::new("opcode with invalid branchingness: "+opcode.getBranchingness());
        }        
        if catches==None        {
            throw NullPointerException::new("catches == null");
        }        
        self->catches=catches;
    }
    pub fn getInlineString(&self) -> String    {
        return ThrowingInsn::toCatchString(&self.catches);
    }
    pub fn getCatches(&self) -> TypeList    {
        return self.catches;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitThrowingInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        return ThrowingInsn::new(getOpcode(), getPosition(), getSources(), self.catches.withAddedType(&type));
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return ThrowingInsn::new(getOpcode(), getPosition(), getSources().withOffset(delta), &self.catches);
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return ThrowingInsn::new(getOpcode(), getPosition(), &sources, &self.catches);
    }
}
