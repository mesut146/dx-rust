use crate::helper;

trait Machine{
    pub fn getPrototype(&self) -> Prototype;
    pub fn clearArgs(&self);
    pub fn popArgs(&self, frame: &Frame, count: i32);
    pub fn popArgs(&self, frame: &Frame, prototype: &Prototype);
    pub fn popArgs(&self, frame: &Frame, type: &Type);
    pub fn popArgs(&self, frame: &Frame, type1: &Type, type2: &Type);
    pub fn popArgs(&self, frame: &Frame, type1: &Type, type2: &Type, type3: &Type);
    pub fn localArg(&self, frame: &Frame, idx: i32);
    pub fn localInfo(&self, local: boolean);
    pub fn auxType(&self, type: &Type);
    pub fn auxIntArg(&self, value: i32);
    pub fn auxCstArg(&self, cst: &Constant);
    pub fn auxTargetArg(&self, target: i32);
    pub fn auxSwitchArg(&self, cases: &SwitchList);
    pub fn auxInitValues(&self, initValues: &ArrayList<Constant>);
    pub fn localTarget(&self, idx: i32, type: &Type, local: &LocalItem);
    pub fn run(&self, frame: &Frame, offset: i32, opcode: i32);
}
