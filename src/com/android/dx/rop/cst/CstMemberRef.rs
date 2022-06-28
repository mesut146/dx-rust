use crate::helper;
use crate::com::android::dx::rop::cst::CstMemberRef;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstNat;

struct CstMemberRef{
    pub definingClass: CstType,
    pub nat: CstNat,
}
impl CstMemberRef{
    pub fn new(&self, definingClass: &CstType, nat: &CstNat)    {
        if definingClass==None        {
            throw NullPointerException::new("definingClass == null");
        }        
        if nat==None        {
            throw NullPointerException::new("nat == null");
        }        
        self->definingClass=definingClass;
        self->nat=nat;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if (other==None)||(getClass()!=other.getClass())        {
            return false;
        }        
        let otherRef: CstMemberRef = (CstMemberRef*)other;
        return self.definingClass.equals(&)&&self.nat.equals(&);
    }
    pub fn hashCode(&self) -> i32    {
        return (self.definingClass.hashCode()*31)^self.nat.hashCode();
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let otherMember: CstMemberRef = (CstMemberRef*)other;
        let cmp: i32 = self.definingClass.compareTo(&);
        if cmp!=0        {
            return cmp;
        }        
        let thisName: CstString = self.nat.getName();
        let otherName: CstString = .getName();
        return thisName.compareTo(&otherName);
    }
    pub fn toString(&self) -> String    {
        return typeName()+'{'+toHuman()+'}';
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn toHuman(&self) -> String    {
        return self.definingClass.toHuman()+'.'+self.nat.toHuman();
    }
    pub fn getDefiningClass(&self) -> CstType    {
        return self.definingClass;
    }
    pub fn getNat(&self) -> CstNat    {
        return self.nat;
    }
}
