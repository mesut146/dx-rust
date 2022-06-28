use crate::helper;

struct IndexedItem{
    pub index: i32,
}
impl IndexedItem{
    pub fn new(&self)    {
        self.index=-1;
    }
    pub fn hasIndex(&self) -> boolean    {
        return (self.index>=0);
    }
    pub fn getIndex(&self) -> i32    {
        if self.index<0        {
            throw RuntimeException::new("index not yet set");
        }        
        return self.index;
    }
    pub fn setIndex(&self, index: i32)    {
        if self->index!=-1        {
            throw RuntimeException::new("index already set");
        }        
        self->index=index;
    }
    pub fn indexString(&self) -> String    {
        return '['+Integer::toHexString(self.index)+']';
    }
}
