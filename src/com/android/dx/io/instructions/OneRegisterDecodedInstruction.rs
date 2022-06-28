use crate::helper;
use crate::com::android::dx::io::instructions::OneRegisterDecodedInstruction;

struct OneRegisterDecodedInstruction{
    pub a: i32,
}
impl OneRegisterDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64, a: i32)    {
        super(format,opcode,index,indexType,target,literal);

        self->a=a;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return 1;
    }
    pub fn getA(&self) -> i32    {
        return self.a;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        return OneRegisterDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), getTarget(), getLiteral(), self.a);
    }
}
