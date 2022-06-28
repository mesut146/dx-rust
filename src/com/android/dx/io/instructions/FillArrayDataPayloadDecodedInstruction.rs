use crate::helper;

struct FillArrayDataPayloadDecodedInstruction{
    pub data: Object,
    pub size: i32,
    pub elementWidth: i32,
}
impl FillArrayDataPayloadDecodedInstruction{
    pub fn new(&self, format: &InstructionCodec, opcode: i32, data: &Object, size: i32, elementWidth: i32)    {
        super(format,opcode,0,null,0,0L);

        self->data=data;
        self->size=size;
        self->elementWidth=elementWidth;
    }
    pub fn new(&self, format: &InstructionCodec, opcode: i32, data: &Vec<i8>)    {
        this(format,opcode,data,data.length,1);

    }
    pub fn new(&self, format: &InstructionCodec, opcode: i32, data: &Vec<i16>)    {
        this(format,opcode,data,data.length,2);

    }
    pub fn new(&self, format: &InstructionCodec, opcode: i32, data: &Vec<i32>)    {
        this(format,opcode,data,data.length,4);

    }
    pub fn new(&self, format: &InstructionCodec, opcode: i32, data: &Vec<i64>)    {
        this(format,opcode,data,data.length,8);

    }
    pub fn getRegisterCount(&self) -> i32    {
        return 0;
    }
    pub fn getElementWidthUnit(&self) -> i16    {
        return self.elementWidth as i16;
    }
    pub fn getSize(&self) -> i32    {
        return self.size;
    }
    pub fn getData(&self) -> Object    {
        return self.data;
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction    {
        throw UnsupportedOperationException::new("no index in instruction");
    }
}
