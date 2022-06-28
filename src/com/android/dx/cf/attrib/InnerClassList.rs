use crate::helper;
use crate::com::android::dx::cf::attrib::InnerClassList::Item;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;

struct InnerClassList{
}
impl InnerClassList{
    pub fn new(&self, count: i32)    {
        super(count);

    }
    pub fn get(&self, n: i32) -> Item    {
        return (Item*)get0(n);
    }
    pub fn set(&self, n: i32, innerClass: &CstType, outerClass: &CstType, innerName: &CstString, accessFlags: i32)    {
        set0(n, Item::new(&innerClass, &outerClass, &innerName, accessFlags));
    }
}
