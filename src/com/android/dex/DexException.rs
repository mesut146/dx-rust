use crate::helper;

struct DexException{
}
impl DexException{
    pub fn new(&self, message: &String)    {
        super(message);

    }
    pub fn new(&self, cause: &Throwable)    {
        super(cause);

    }
}
