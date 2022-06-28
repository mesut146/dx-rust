use crate::helper;

struct BaseAttribute{
    pub name: String,
}
impl BaseAttribute{
    pub fn new(&self, name: &String)    {
        if name==None        {
            throw NullPointerException::new("name == null");
        }        
        self->name=name;
    }
    pub fn getName(&self) -> String    {
        return self.name;
    }
}
