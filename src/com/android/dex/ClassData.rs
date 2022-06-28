use crate::helper;

struct ClassData{
    pub staticFields: Vec<Field>,
    pub instanceFields: Vec<Field>,
    pub directMethods: Vec<Method>,
    pub virtualMethods: Vec<Method>,
}
impl ClassData{
    pub fn new(&self, staticFields: &Vec<Field>, instanceFields: &Vec<Field>, directMethods: &Vec<Method>, virtualMethods: &Vec<Method>)    {
        self->staticFields=staticFields;
        self->instanceFields=instanceFields;
        self->directMethods=directMethods;
        self->virtualMethods=virtualMethods;
    }
    pub fn getStaticFields(&self) -> Vec<Field>    {
        return self.staticFields;
    }
    pub fn getInstanceFields(&self) -> Vec<Field>    {
        return self.instanceFields;
    }
    pub fn getDirectMethods(&self) -> Vec<Method>    {
        return self.directMethods;
    }
    pub fn getVirtualMethods(&self) -> Vec<Method>    {
        return self.virtualMethods;
    }
    pub fn allFields(&self) -> Vec<Field>    {
        let result: Vec<Field> = new Field[staticFields.length + instanceFields.length];
        System::arraycopy(&self.staticFields, 0, &result, 0, self.staticFields.len());
        System::arraycopy(&self.instanceFields, 0, &result, self.staticFields.len(), self.instanceFields.len());
        return result;
    }
    pub fn allMethods(&self) -> Vec<Method>    {
        let result: Vec<Method> = new Method[directMethods.length + virtualMethods.length];
        System::arraycopy(&self.directMethods, 0, &result, 0, self.directMethods.len());
        System::arraycopy(&self.virtualMethods, 0, &result, self.directMethods.len(), self.virtualMethods.len());
        return result;
    }
}
