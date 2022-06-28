use crate::helper;
use crate::com::android::dx::util::MutabilityException;
use crate::com::android::dx::cf::attrib::InnerClassList;

let static ATTRIBUTE_NAME: String = "InnerClasses";
struct AttInnerClasses{
    pub innerClasses: InnerClassList,
}
impl AttInnerClasses{
    pub fn new(&self, innerClasses: &InnerClassList)    {
        super(ATTRIBUTE_NAME);

        try        {
            if innerClasses.isMutable()            {
                throw MutabilityException::new("innerClasses.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("innerClasses == null");
        }
        self->innerClasses=innerClasses;
    }
    pub fn byteLength(&self) -> i32    {
        return 8+self.innerClasses.size()*8;
    }
    pub fn getInnerClasses(&self) -> InnerClassList    {
        return self.innerClasses;
    }
}
