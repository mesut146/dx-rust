use crate::helper;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::util::MutabilityException;

let static ATTRIBUTE_NAME: String = "Exceptions";
struct AttExceptions{
    pub exceptions: TypeList,
}
impl AttExceptions{
    pub fn new(&self, exceptions: &TypeList)    {
        super(ATTRIBUTE_NAME);

        try        {
            if exceptions.isMutable()            {
                throw MutabilityException::new("exceptions.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("exceptions == null");
        }
        self->exceptions=exceptions;
    }
    pub fn byteLength(&self) -> i32    {
        return 8+self.exceptions.size()*2;
    }
    pub fn getExceptions(&self) -> TypeList    {
        return self.exceptions;
    }
}
