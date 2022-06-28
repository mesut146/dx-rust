use crate::helper;

struct SparseSwitchPayloadDecodedInstruction{
    pub keys: Vec<i32>,
    pub targets: Vec<i32>,
}
impl SparseSwitchPayloadDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, keys: &Vec<i32>, targets: &Vec<i32>)    {
        super(format,opcode,0,null,0,0L);

        if keys.len()!=targets.len()        {
            throw IllegalArgumentException::new("keys/targets length mismatch");
        }        
        self->keys=keys;
        self->targets=targets;
    }
    pub fn getRegisterCount(&self) -> i32    {
        return 0;
    }
    pub fn getKeys(&self) -> Vec<i32>    {
        return self.keys;
    }
    pub fn getTargets(&self) -> Vec<i32>    {
        return self.targets;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        throw UnsupportedOperationException::new("no index in instruction");
    }
}
