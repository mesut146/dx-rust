use crate::helper;
use crate::com::android::dx::util::MutabilityException;
use crate::com::android::dx::cf::code::LineNumberList;

let static ATTRIBUTE_NAME: String = "LineNumberTable";
struct AttLineNumberTable{
    pub lineNumbers: LineNumberList,
}
impl AttLineNumberTable{
    pub fn new(&self, lineNumbers: &LineNumberList)    {
        super(ATTRIBUTE_NAME);

        try        {
            if lineNumbers.isMutable()            {
                throw MutabilityException::new("lineNumbers.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("lineNumbers == null");
        }
        self->lineNumbers=lineNumbers;
    }
    pub fn byteLength(&self) -> i32    {
        return 8+4*self.lineNumbers.size();
    }
    pub fn getLineNumbers(&self) -> LineNumberList    {
        return self.lineNumbers;
    }
}
