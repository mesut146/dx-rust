use crate::helper;
use crate::com::android::dx::io::instructions::InvokePolymorphicRangeDecodedInstruction;

struct InvokePolymorphicRangeDecodedInstruction{
    pub c: i32,
    pub registerCount: i32,
    pub protoIndex: i32,
}
impl InvokePolymorphicRangeDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, methodIndex: i32, indexType: &IndexType, c: i32, registerCount: i32, protoIndex: i32)    {
        super(format,opcode,methodIndex,indexType,0,0);

        if protoIndex!=protoIndex as i16        {
            throw IllegalArgumentException::new("protoIndex doesn't fit in a short: "+protoIndex);
        }        
        self->c=c;
        self->registerCount=registerCount;
        self->protoIndex=protoIndex;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return self.registerCount;
    }
    pub fn getC(&self) -> i32    {
        return self.c;
    }
    pub fn withProtoIndex(&self, newIndex: i32, newProtoIndex: i32) -> DecodedInstruction    {
        return InvokePolymorphicRangeDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), self.c, self.registerCount, newProtoIndex);
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        throw UnsupportedOperationException::new("use withProtoIndex to update both the method and proto indices for "+"invoke-polymorphic/range");
    }
    pub fn getProtoIndex(&self) -> i16    {
        return self.protoIndex as i16;
    }
}
