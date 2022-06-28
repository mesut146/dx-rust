use crate::helper;
use crate::com::android::dx::rop::cst::CstChar;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

let static VALUE_0: CstChar = CstChar::make_char(0 as char);
struct CstChar{
}
impl CstChar{
    pub fn make(value: char) -> CstChar    {
        return CstChar::new(value);
    }
    pub fn make(value: i32) -> CstChar    {
        let cast: char = value as char;
        if cast!=value        {
            throw IllegalArgumentException::new("bogus char value: "+value);
        }        
        return CstChar::make_char(cast);
    }
    pub fn new(&self, value: char)    {
        super(value);

    }
    pub fn toString(&self) -> String    {
        let value: i32 = getIntBits();
        return "char{0x"+Hex::u2(value)+" / "+value+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::CHAR;
    }
    pub fn typeName(&self) -> String    {
        return "char";
    }
    pub fn toHuman(&self) -> String    {
        return Integer::toString_int(getIntBits());
    }
    pub fn getValue(&self) -> char    {
        return getIntBits() as char;
    }
}
