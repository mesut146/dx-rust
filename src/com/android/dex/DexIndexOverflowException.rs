use crate::helper;

struct DexIndexOverflowException{
}
impl DexIndexOverflowException{
    pub fn new(&self, message: &String)    {
        super(message);

    }
    pub fn new(&self, cause: &Throwable)    {
        super(cause);

    }
}
