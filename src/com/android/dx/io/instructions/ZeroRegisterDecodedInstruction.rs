use crate::helper;
use crate::com::android::dx::io::instructions::ZeroRegisterDecodedInstruction;

struct ZeroRegisterDecodedInstruction{
}
impl ZeroRegisterDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64)    {
        super(format,opcode,index,indexType,target,literal);

    }
    pub fn getRegisterCount(&self) -> i32    {
        return 0;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        return ZeroRegisterDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), getTarget(), getLiteral());
    }
}
