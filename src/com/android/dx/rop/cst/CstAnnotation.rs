use crate::helper;
use crate::com::android::dx::rop::cst::CstAnnotation;
use crate::com::android::dx::rop::annotation::Annotation;

struct CstAnnotation{
    pub annotation: Annotation,
}
impl CstAnnotation{
    pub fn new(&self, annotation: &Annotation)    {
        if annotation==None        {
            throw NullPointerException::new("annotation == null");
        }        
        annotation.throwIfMutable();
        self->annotation=annotation;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof CstAnnotation)        {
            return false;
        }        
        return self.annotation.equals(((CstAnnotation*)other)->annotation);
    }
    pub fn hashCode(&self) -> i32    {
        return self.annotation.hashCode();
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        return self.annotation.compareTo(((CstAnnotation*)other)->annotation);
    }
    pub fn toString(&self) -> String    {
        return self.annotation.toString();
    }
    pub fn typeName(&self) -> String    {
        return "annotation";
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn toHuman(&self) -> String    {
        return self.annotation.toString();
    }
    pub fn getAnnotation(&self) -> Annotation    {
        return self.annotation;
    }
}
