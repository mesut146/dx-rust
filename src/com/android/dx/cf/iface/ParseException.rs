use crate::helper;

struct ParseException{
}
impl ParseException{
    pub fn new(&self, message: &String)    {
        super(message);

    }
    pub fn new(&self, cause: &Throwable)    {
        super(cause);

    }
    pub fn new(&self, message: &String, cause: &Throwable)    {
        super(message,cause);

    }
}
