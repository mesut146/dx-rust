use crate::helper;
use crate::com::android::dx::io::instructions::TwoRegisterDecodedInstruction;

struct TwoRegisterDecodedInstruction{
    pub a: i32,
    pub b: i32,
}
impl TwoRegisterDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64, a: i32, b: i32)    {
        super(format,opcode,index,indexType,target,literal);

        self->a=a;
        self->b=b;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return 2;
    }
    pub fn getA(&self) -> i32    {
        return self.a;
    }
    pub fn getB(&self) -> i32    {
        return self.b;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        return TwoRegisterDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), getTarget(), getLiteral(), self.a, self.b);
    }
}
