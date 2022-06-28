use crate::helper;
use crate::com::android::dx::io::instructions::FourRegisterDecodedInstruction;

struct FourRegisterDecodedInstruction{
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}
impl FourRegisterDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64, a: i32, b: i32, c: i32, d: i32)    {
        super(format,opcode,index,indexType,target,literal);

        self->a=a;
        self->b=b;
        self->c=c;
        self->d=d;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return 4;
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
    pub fn getD(&self) -> i32    {
        return self.d;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        return FourRegisterDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), getTarget(), getLiteral(), self.a, self.b, self.c, self.d);
    }
}
