use crate::helper;
use crate::com::android::dx::rop::cst::CstLiteral32;

struct CstLiteral32{
    pub bits: i32,
}
impl CstLiteral32{
    pub fn new(&self, bits: i32)    {
        self->bits=bits;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        return (other!=None)&&(getClass()==other.getClass())&&self.bits==((CstLiteral32*)other)->bits;
    }
    pub fn hashCode(&self) -> i32    {
        return self.bits;
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let otherBits: i32 = ((CstLiteral32*)other)->bits;
        if self.bits<otherBits        {
            return -1;
        }        else         if self.bits>otherBits        {
            return 1;
        }        else         {
            return 0;
        }
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn fitsInInt(&self) -> boolean    {
        return true;
    }
    pub fn getIntBits(&self) -> i32    {
        return self.bits;
    }
    pub fn getLongBits(&self) -> i64    {
        return self.bits as i64;
    }
}
