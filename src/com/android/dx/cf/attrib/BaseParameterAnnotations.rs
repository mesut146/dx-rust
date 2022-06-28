use crate::helper;
use crate::com::android::dx::util::MutabilityException;
use crate::com::android::dx::rop::annotation::AnnotationsList;

struct BaseParameterAnnotations{
    pub parameterAnnotations: AnnotationsList,
    pub byteLength: i32,
}
impl BaseParameterAnnotations{
    pub fn new(&self, attributeName: &String, parameterAnnotations: &AnnotationsList, byteLength: i32)    {
        super(attributeName);

        try        {
            if parameterAnnotations.isMutable()            {
                throw MutabilityException::new("parameterAnnotations.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("parameterAnnotations == null");
        }
        self->parameterAnnotations=parameterAnnotations;
        self->byteLength=byteLength;
    }
    pub fn byteLength(&self) -> i32    {
        return self.byteLength+6;
    }
    pub fn getParameterAnnotations(&self) -> AnnotationsList    {
        return self.parameterAnnotations;
    }
}
