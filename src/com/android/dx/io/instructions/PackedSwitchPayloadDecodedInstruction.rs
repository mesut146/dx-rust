use crate::helper;

struct PackedSwitchPayloadDecodedInstruction{
    pub firstKey: i32,
    pub targets: Vec<i32>,
}
impl PackedSwitchPayloadDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, firstKey: i32, targets: &Vec<i32>)    {
        super(format,opcode,0,null,0,0L);

        self->firstKey=firstKey;
        self->targets=targets;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return 0;
    }
    pub fn getFirstKey(&self) -> i32    {
        return self.firstKey;
    }
    pub fn getTargets(&self) -> Vec<i32>    {
        return self.targets;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        throw UnsupportedOperationException::new("no index in instruction");
    }
}
