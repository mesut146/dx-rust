use crate::helper;

let static ATTRIBUTE_NAME: String = "LocalVariableTypeTable";
struct AttLocalVariableTypeTable{
}
impl AttLocalVariableTypeTable{
    pub fn new(&self, localVariables: &LocalVariableList)    {
        super(ATTRIBUTE_NAME,localVariables);

    }
}
