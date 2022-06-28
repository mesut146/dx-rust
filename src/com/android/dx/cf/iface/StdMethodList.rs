use crate::helper;
use crate::com::android::dx::cf::iface::Method;

struct StdMethodList{
}
impl StdMethodList{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Method    {
        return (Method*)get0(n);
    }
    pub fn set(&self, n: i32, method: &Method)    {
        set0(n, &method);
    }
}
