use crate::helper;
use crate::com::android::dx::rop::cst::CstLiteral64;

struct CstLiteral64{
    pub bits: i64,
}
impl CstLiteral64{
    pub fn new(&self, bits: i64)    {
        self->bits=bits;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        return (other!=None)&&(getClass()==other.getClass())&&self.bits==((CstLiteral64*)other)->bits;
    }
    pub fn hashCode(&self) -> i32    {
        return self.bits as i32^(self.bits>>32) as i32;
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let otherBits: i64 = ((CstLiteral64*)other)->bits;
        if self.bits<otherBits        {
            return -1;
        }        else         if self.bits>otherBits        {
            return 1;
        }        else         {
            return 0;
        }
    }
    pub fn isCategory2(&self) -> boolean    {
        return true;
    }
    pub fn fitsInInt(&self) -> boolean    {
        return self.bits as i32==self.bits;
    }
    pub fn getIntBits(&self) -> i32    {
        return self.bits as i32;
    }
    pub fn getLongBits(&self) -> i64    {
        return self.bits;
    }
}
