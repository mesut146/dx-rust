use crate::helper;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstNat;

let static PRIMITIVE_TYPE_NAT: CstNat = CstNat::new(CstString::new("TYPE"), CstString::new("Ljava/lang/Class;"));
struct CstNat{
    pub name: CstString,
    pub descriptor: CstString,
}
impl CstNat{
    pub fn new(&self, name: &CstString, descriptor: &CstString)    {
        if name==None        {
            throw NullPointerException::new("name == null");
        }        
        if descriptor==None        {
            throw NullPointerException::new("descriptor == null");
        }        
        self->name=name;
        self->descriptor=descriptor;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof CstNat)        {
            return false;
        }        
        let otherNat: CstNat = (CstNat*)other;
        return self.name.equals(&)&&self.descriptor.equals(&);
    }
    pub fn hashCode(&self) -> i32    {
        return (self.name.hashCode()*31)^self.descriptor.hashCode();
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let otherNat: CstNat = (CstNat*)other;
        let cmp: i32 = self.name.compareTo(&);
        if cmp!=0        {
            return cmp;
        }        
        return self.descriptor.compareTo(&);
    }
    pub fn toString(&self) -> String    {
        return "nat{"+toHuman()+'}';
    }
    pub fn typeName(&self) -> String    {
        return "nat";
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn getName(&self) -> CstString    {
        return self.name;
    }
    pub fn getDescriptor(&self) -> CstString    {
        return self.descriptor;
    }
    pub fn toHuman(&self) -> String    {
        return self.name.toHuman()+':'+self.descriptor.toHuman();
    }
    pub fn getFieldType(&self) -> Type    {
        return Type::intern(self.descriptor.getString());
    }
    pub fn isInstanceInit(&self) -> boolean    {
        return self.name.getString().equals("<init>");
    }
    pub fn isClassInit(&self) -> boolean    {
        return self.name.getString().equals("<clinit>");
    }
}
