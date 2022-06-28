use crate::helper;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstNat;

struct StdMember{
    pub definingClass: CstType,
    pub accessFlags: i32,
    pub nat: CstNat,
    pub attributes: AttributeList,
}
impl StdMember{
    pub fn new(&self, definingClass: &CstType, accessFlags: i32, nat: &CstNat, attributes: &AttributeList)    {
        if definingClass==None        {
            throw NullPointerException::new("definingClass == null");
        }        
        if nat==None        {
            throw NullPointerException::new("nat == null");
        }        
        if attributes==None        {
            throw NullPointerException::new("attributes == null");
        }        
        self->definingClass=definingClass;
        self->accessFlags=accessFlags;
        self->nat=nat;
        self->attributes=attributes;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String(getClass().getName());
        sb.append_char('{');
        sb.append_String(self.nat.toHuman());
        sb.append_char('}');
        return sb.toString();
    }
    pub fn getDefiningClass(&self) -> CstType    {
        return self.definingClass;
    }
    pub fn getAccessFlags(&self) -> i32    {
        return self.accessFlags;
    }
    pub fn getNat(&self) -> CstNat    {
        return self.nat;
    }
    pub fn getName(&self) -> CstString    {
        return self.nat.getName();
    }
    pub fn getDescriptor(&self) -> CstString    {
        return self.nat.getDescriptor();
    }
    pub fn getAttributes(&self) -> AttributeList    {
        return self.attributes;
    }
}
