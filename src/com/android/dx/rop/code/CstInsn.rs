use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::CstInsn;

struct CstInsn{
    pub cst: Constant,
}
impl CstInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, result: &RegisterSpec, sources: &RegisterSpecList, cst: &Constant)    {
        super(opcode,position,result,sources);

        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        self->cst=cst;
    }
    pub fn getInlineString(&self) -> String    {
        return self.cst.toHuman();
    }
    pub fn getConstant(&self) -> Constant    {
        return self.cst;
    }
    pub fn contentEquals(&self, b: &Insn) -> boolean    {
        return super.contentEquals(b)&&self.cst.equals(((CstInsn*)b).getConstant());
    }
}
