use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::cst::CstDouble;

struct BootstrapMethodArgumentsList{
}
impl BootstrapMethodArgumentsList{
    pub fn new(&self, count: i32)    {
        super(count);

    }
    pub fn get(&self, n: i32) -> Constant    {
        return (Constant*)get0(n);
    }
    pub fn set(&self, n: i32, cst: &Constant)    {
        if //cst instanceof CstString||//cst instanceof CstType||//cst instanceof CstInteger||//cst instanceof CstLong||//cst instanceof CstFloat||//cst instanceof CstDouble||//cst instanceof CstMethodHandle||//cst instanceof CstProtoRef        {
            set0(n, &cst);
        }        else         {
            let klass: Class<?> = cst.getClass();
            throw IllegalArgumentException::new("bad type for bootstrap argument: "+klass);
        }
    }
}
