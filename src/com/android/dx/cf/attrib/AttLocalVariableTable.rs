use crate::helper;

let static ATTRIBUTE_NAME: String = "LocalVariableTable";
struct AttLocalVariableTable{
}
impl AttLocalVariableTable{
    pub fn new(&self, localVariables: &LocalVariableList)    {
        super(ATTRIBUTE_NAME,localVariables);

    }
}
