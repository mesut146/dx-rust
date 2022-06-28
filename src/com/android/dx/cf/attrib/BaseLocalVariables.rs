use crate::helper;
use crate::com::android::dx::util::MutabilityException;
use crate::com::android::dx::cf::code::LocalVariableList;

struct BaseLocalVariables{
    pub localVariables: LocalVariableList,
}
impl BaseLocalVariables{
    pub fn new(&self, name: &String, localVariables: &LocalVariableList)    {
        super(name);

        try        {
            if localVariables.isMutable()            {
                throw MutabilityException::new("localVariables.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("localVariables == null");
        }
        self->localVariables=localVariables;
    }
    pub fn byteLength(&self) -> i32    {
        return 8+self.localVariables.size()*10;
    }
    pub fn getLocalVariables(&self) -> LocalVariableList    {
        return self.localVariables;
    }
}
