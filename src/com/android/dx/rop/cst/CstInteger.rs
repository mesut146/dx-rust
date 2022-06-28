use crate::helper;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

let static cache: Vec<CstInteger> = new CstInteger[511];
let static VALUE_M1: CstInteger = CstInteger::make(-1);
let static VALUE_0: CstInteger = CstInteger::make(0);
let static VALUE_1: CstInteger = CstInteger::make(1);
let static VALUE_2: CstInteger = CstInteger::make(2);
let static VALUE_3: CstInteger = CstInteger::make(3);
let static VALUE_4: CstInteger = CstInteger::make(4);
let static VALUE_5: CstInteger = CstInteger::make(5);
struct CstInteger{
}
impl CstInteger{
    pub fn make(value: i32) -> CstInteger    {
        let idx: i32 = (value&0x7fffffff)%CstInteger::cache.len();
        let obj: CstInteger = CstInteger::cache[idx];
        if (obj!=None)&&(obj.getValue()==value)        {
            return obj;
        }        
        obj=CstInteger::new(value);
        CstInteger::cache[idx]=obj;
        return obj;
    }
    pub fn new(&self, value: i32)    {
        super(value);

    }
    pub fn toString(&self) -> String    {
        let value: i32 = getIntBits();
        return "int{0x"+Hex::u4(value)+" / "+value+'}';
    }
    pub fn getType(&self) -> Type    {
        return Type::INT;
    }
    pub fn typeName(&self) -> String    {
        return "int";
    }
    pub fn toHuman(&self) -> String    {
        return Integer::toString_int(getIntBits());
    }
    pub fn getValue(&self) -> i32    {
        return getIntBits();
    }
}
