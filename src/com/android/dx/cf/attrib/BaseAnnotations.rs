use crate::helper;
use crate::com::android::dx::util::MutabilityException;
use crate::com::android::dx::rop::annotation::Annotations;

struct BaseAnnotations{
    pub annotations: Annotations,
    pub byteLength: i32,
}
impl BaseAnnotations{
    pub fn new(&self, attributeName: &String, annotations: &Annotations, byteLength: i32)    {
        super(attributeName);

        try        {
            if annotations.isMutable()            {
                throw MutabilityException::new("annotations.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("annotations == null");
        }
        self->annotations=annotations;
        self->byteLength=byteLength;
    }
    pub fn byteLength(&self) -> i32    {
        return self.byteLength+6;
    }
    pub fn getAnnotations(&self) -> Annotations    {
        return self.annotations;
    }
}
