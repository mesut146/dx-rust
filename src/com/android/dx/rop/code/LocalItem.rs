use crate::helper;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::cst::CstString;

struct LocalItem{
    pub name: CstString,
    pub signature: CstString,
}
impl LocalItem{
    pub fn make(name: &CstString, signature: &CstString) -> LocalItem    {
        if name==None&&signature==None        {
            return None;
        }        
        return LocalItem::new(&name, &signature);
    }
    pub fn new(&self, name: &CstString, signature: &CstString)    {
        self->name=name;
        self->signature=signature;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof LocalItem)        {
            return false;
        }        
        let local: LocalItem = (LocalItem*)other;
        return 0==compareTo(&local);
    }
    pub fn compareHandlesNulls(a: &CstString, b: &CstString) -> i32    {
        if a==b        {
            return 0;
        }        else         if a==None        {
            return -1;
        }        else         if b==None        {
            return 1;
        }        else         {
            return a.compareTo(&b);
        }
    }
    pub fn compareTo(&self, local: &LocalItem) -> i32    {
        let ret: i32;
        ret=LocalItem::compareHandlesNulls(&self.name, &);
        if ret!=0        {
            return ret;
        }        
        ret=LocalItem::compareHandlesNulls(&self.signature, &);
        return ret;
    }
    pub fn hashCode(&self) -> i32    {
        return (if self.name==None { 0 } else { self.name.hashCode() })*31+(if self.signature==None { 0 } else { self.signature.hashCode() });
                    }
                    pub fn toString(&self) -> String                    {
                        if self.name!=None&&self.signature==None                        {
                            return self.name.toQuoted();
                        }                        else                         if self.name==None&&self.signature==None                        {
                            return "";
                        }                        
                        return "["+(if self.name==None { "" } else { self.name.toQuoted() })+"|"+(if self.signature==None { "" } else { self.signature.toQuoted() });
                                    }
                                    pub fn getName(&self) -> CstString                                    {
                                        return self.name;
                                    }
                                    pub fn getSignature(&self) -> CstString                                    {
                                        return self.signature;
                                    }
}
