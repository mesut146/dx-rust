use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::rop::cst::CstBoolean;
use crate::com::android::dx::rop::cst::CstChar;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::cst::CstShort;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstDouble;
use crate::com::android::dx::rop::cst::CstByte;

struct Zeroes{
}
impl Zeroes{
    pub fn new(&self)    {
    }
    pub fn zeroFor(type: &Type) -> Constant    {
        match type.getBasicType(){Type::BT_BOOLEAN =>             return CstBoolean::VALUE_FALSE;Type::BT_BYTE =>             return CstByte::VALUE_0;Type::BT_CHAR =>             return CstChar::VALUE_0;Type::BT_DOUBLE =>             return CstDouble::VALUE_0;Type::BT_FLOAT =>             return CstFloat::VALUE_0;Type::BT_INT =>             return CstInteger::VALUE_0;Type::BT_LONG =>             return CstLong::VALUE_0;Type::BT_SHORT =>             return CstShort::VALUE_0;Type::BT_OBJECT =>             return CstKnownNull::THE_ONE;        _ => {}        {
            throw UnsupportedOperationException::new("no zero for type: "+type.toHuman());
        }    }
}
}
