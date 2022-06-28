use crate::helper;
use crate::com::android::dx::cf::iface::Field;

struct StdFieldList{
}
impl StdFieldList{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Field    {
        return (Field*)get0(n);
    }
    pub fn set(&self, n: i32, field: &Field)    {
        set0(n, &field);
    }
}
