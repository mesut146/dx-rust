use crate::helper;

struct CstLiteralBits{
}
impl CstLiteralBits{
    pub fn fitsInInt(&self) -> boolean;
    pub fn getIntBits(&self) -> i32;
    pub fn getLongBits(&self) -> i64;
    pub fn fitsIn16Bits(&self) -> boolean    {
        if !fitsInInt()        {
            return false;
        }        
        let bits: i32 = getIntBits();
        return bits as i16==bits;
    }
    pub fn fitsIn8Bits(&self) -> boolean    {
        if !fitsInInt()        {
            return false;
        }        
        let bits: i32 = getIntBits();
        return bits as i8==bits;
    }
}
