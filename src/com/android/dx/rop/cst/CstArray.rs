use crate::helper;
use crate::com::android::dx::rop::cst::CstArray;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::cst::CstArray::List;

struct CstArray{
    pub list: List,
}
impl CstArray{
    pub fn new(&self, list: &List)    {
        if list==None        {
            throw NullPointerException::new("list == null");
        }        
        list.throwIfMutable();
        self->list=list;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof CstArray)        {
            return false;
        }        
        return self.list.equals(((CstArray*)other)->list);
    }
    pub fn hashCode(&self) -> i32    {
        return self.list.hashCode();
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        return self.list.compareTo(((CstArray*)other)->list);
    }
    pub fn toString(&self) -> String    {
        return self.list.toString_String_String_String("array{", ", ", "}");
    }
    pub fn typeName(&self) -> String    {
        return "array";
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn toHuman(&self) -> String    {
        return self.list.toHuman_String_String_String("{", ", ", "}");
    }
    pub fn getList(&self) -> List    {
        return self.list;
    }
}
