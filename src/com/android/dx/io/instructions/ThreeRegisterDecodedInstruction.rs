use crate::helper;
use crate::com::android::dx::io::instructions::ThreeRegisterDecodedInstruction;

struct ThreeRegisterDecodedInstruction{
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
impl ThreeRegisterDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64, a: i32, b: i32, c: i32)    {
        super(format,opcode,index,indexType,target,literal);

        self->a=a;
        self->b=b;
        self->c=c;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return 3;
    }
    pub fn getA(&self) -> i32    {
        return self.a;
    }
    pub fn getB(&self) -> i32    {
        return self.b;
    }
    pub fn getC(&self) -> i32    {
        return self.c;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        return ThreeRegisterDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), getTarget(), getLiteral(), self.a, self.b, self.c);
    }
}
