use crate::helper;
use crate::com::android::dx::io::instructions::RegisterRangeDecodedInstruction;

struct RegisterRangeDecodedInstruction{
    pub a: i32,
    pub registerCount: i32,
}
impl RegisterRangeDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64, a: i32, registerCount: i32)    {
        super(format,opcode,index,indexType,target,literal);

        self->a=a;
        self->registerCount=registerCount;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return self.registerCount;
    }
    pub fn getA(&self) -> i32    {
        return self.a;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        return RegisterRangeDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), getTarget(), getLiteral(), self.a, self.registerCount);
    }
}
