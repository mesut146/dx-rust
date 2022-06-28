use crate::helper;
use crate::com::android::dx::io::instructions::InvokePolymorphicDecodedInstruction;

struct InvokePolymorphicDecodedInstruction{
    pub protoIndex: i32,
    pub registers: Vec<i32>,
}
impl InvokePolymorphicDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, methodIndex: i32, indexType: &IndexType, protoIndex: i32, registers: &Vec<i32>)    {
        super(format,opcode,methodIndex,indexType,0,0);

        if protoIndex!=protoIndex as i16        {
            throw IllegalArgumentException::new("protoIndex doesn't fit in a short: "+protoIndex);
        }        
        self->protoIndex=protoIndex;
        self->registers=registers;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return self.registers.len();
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        throw UnsupportedOperationException::new("use withProtoIndex to update both the method and proto indices for"+" invoke-polymorphic");
    }
    pub fn withProtoIndex(&self, newIndex: i32, newProtoIndex: i32) -> DecodedInstruction    {
        return InvokePolymorphicDecodedInstruction::new(getFormat(), getOpcode(), newIndex, getIndexType(), newProtoIndex, &self.registers);
    }
    pub fn getC(&self) -> i32    {
        return if self.registers.len()>0 { self.registers[0] } else { 0 };
            }
            pub fn getD(&self) -> i32            {
                return if self.registers.len()>1 { self.registers[1] } else { 0 };
                    }
                    pub fn getE(&self) -> i32                    {
                        return if self.registers.len()>2 { self.registers[2] } else { 0 };
                            }
                            pub fn getF(&self) -> i32                            {
                                return if self.registers.len()>3 { self.registers[3] } else { 0 };
                                    }
                                    pub fn getG(&self) -> i32                                    {
                                        return if self.registers.len()>4 { self.registers[4] } else { 0 };
                                            }
                                            pub fn getProtoIndex(&self) -> i16                                            {
                                                return self.protoIndex as i16;
                                            }
}
