use crate::helper;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::type::Type;

struct CstProtoRef{
    pub prototype: Prototype,
}
impl CstProtoRef{
    pub fn new(&self, prototype: &Prototype)    {
        self->prototype=prototype;
    }
    pub fn make(descriptor: &CstString) -> CstProtoRef    {
        let prototype: Prototype = Prototype::fromDescriptor(descriptor.getString());
        return CstProtoRef::new(&prototype);
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof CstProtoRef)        {
            return false;
        }        
        let otherCstProtoRef: CstProtoRef = (CstProtoRef*)other;
        return getPrototype().equals(otherCstProtoRef.getPrototype());
    }
    pub fn hashCode(&self) -> i32    {
        return self.prototype.hashCode();
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn typeName(&self) -> String    {
        return "proto";
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let otherCstProtoRef: CstProtoRef = (CstProtoRef*)other;
        return self.prototype.compareTo(otherCstProtoRef.getPrototype());
    }
    pub fn toHuman(&self) -> String    {
        return self.prototype.getDescriptor();
    }
    pub fn toString(&self) -> String    {
        return typeName()+"{"+toHuman()+'}';
    }
    pub fn getPrototype(&self) -> Prototype    {
        return self.prototype;
    }
    pub fn getType(&self) -> Type    {
        return Type::METHOD_TYPE;
    }
}
