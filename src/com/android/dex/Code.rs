use crate::helper;

struct Code{
    pub registersSize: i32,
    pub insSize: i32,
    pub outsSize: i32,
    pub debugInfoOffset: i32,
    pub instructions: Vec<i16>,
    pub tries: Vec<Try>,
    pub catchHandlers: Vec<CatchHandler>,
}
impl Code{
    pub fn new(&self, registersSize: i32, insSize: i32, outsSize: i32, debugInfoOffset: i32, instructions: &Vec<i16>, tries: &Vec<Try>, catchHandlers: &Vec<CatchHandler>)    {
        self->registersSize=registersSize;
        self->insSize=insSize;
        self->outsSize=outsSize;
        self->debugInfoOffset=debugInfoOffset;
        self->instructions=instructions;
        self->tries=tries;
        self->catchHandlers=catchHandlers;
    }
    pub fn getRegistersSize(&self) -> i32    {
        return self.registersSize;
    }
    pub fn getInsSize(&self) -> i32    {
        return self.insSize;
    }
    pub fn getOutsSize(&self) -> i32    {
        return self.outsSize;
    }
    pub fn getDebugInfoOffset(&self) -> i32    {
        return self.debugInfoOffset;
    }
    pub fn getInstructions(&self) -> Vec<i16>    {
        return self.instructions;
    }
    pub fn getTries(&self) -> Vec<Try>    {
        return self.tries;
    }
    pub fn getCatchHandlers(&self) -> Vec<CatchHandler>    {
        return self.catchHandlers;
    }
}
