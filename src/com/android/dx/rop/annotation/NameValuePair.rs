use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::rop::cst::CstString;

struct NameValuePair{
    pub name: CstString,
    pub value: Constant,
}
impl NameValuePair{
    pub fn new(&self, name: &CstString, value: &Constant)    {
        if name==None        {
            throw NullPointerException::new("name == null");
        }        
        if value==None        {
            throw NullPointerException::new("value == null");
        }        
        self->name=name;
        self->value=value;
    }
    pub fn toString(&self) -> String    {
        return self.name.toHuman()+":"+self.value;
    }
    pub fn hashCode(&self) -> i32    {
        return self.name.hashCode()*31+self.value.hashCode();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof NameValuePair)        {
            return false;
        }        
        let otherPair: NameValuePair = (NameValuePair*)other;
        return self.name.equals(&)&&self.value.equals(&);
    }
    pub fn compareTo(&self, other: &NameValuePair) -> i32    {
        let result: i32 = self.name.compareTo(&);
        if result!=0        {
            return result;
        }        
        return self.value.compareTo(&);
    }
    pub fn getName(&self) -> CstString    {
        return self.name;
    }
    pub fn getValue(&self) -> Constant    {
        return self.value;
    }
}
