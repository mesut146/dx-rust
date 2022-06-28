use crate::helper;
use crate::com::android::dx::rop::cst::CstBoolean;
use crate::com::android::dx::rop::type::Type;

let static VALUE_FALSE: CstBoolean = CstBoolean::new(false);
let static VALUE_TRUE: CstBoolean = CstBoolean::new(true);
struct CstBoolean{
}
impl CstBoolean{
    pub fn make(value: boolean) -> CstBoolean    {
        return if value { CstBoolean::VALUE_TRUE } else { CstBoolean::VALUE_FALSE };
            }
            pub fn make(value: i32) -> CstBoolean            {
                if value==0                {
                    return CstBoolean::VALUE_FALSE;
                }                else                 if value==1                {
                    return CstBoolean::VALUE_TRUE;
                }                else                 {
                    throw IllegalArgumentException::new("bogus value: "+value);
                }
            }
            pub fn new(&self, value: boolean)            {
                super(value ? 1 : 0);

            }
            pub fn toString(&self) -> String            {
                return if getValue() { "boolean{true}" } else { "boolean{false}" };
                    }
                    pub fn getType(&self) -> Type                    {
                        return Type::BOOLEAN;
                    }
                    pub fn typeName(&self) -> String                    {
                        return "boolean";
                    }
                    pub fn toHuman(&self) -> String                    {
                        return if getValue() { "true" } else { "false" };
                            }
                            pub fn getValue(&self) -> boolean                            {
                                return if (getIntBits()==0) { false } else { true };
                                    }
}
