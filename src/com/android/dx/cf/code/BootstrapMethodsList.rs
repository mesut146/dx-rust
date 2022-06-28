use crate::helper;
use crate::com::android::dx::cf::code::BootstrapMethodsList;
use crate::com::android::dx::cf::code::BootstrapMethodsList::Item;
use crate::com::android::dx::cf::code::BootstrapMethodArgumentsList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstMethodHandle;

let static EMPTY: BootstrapMethodsList = BootstrapMethodsList::new(0);
struct BootstrapMethodsList{
}
impl BootstrapMethodsList{
    pub fn new(&self, count: i32)    {
        super(count);

    }
    pub fn get(&self, n: i32) -> Item    {
        return (Item*)get0(n);
    }
    pub fn set(&self, n: i32, item: &Item)    {
        if item==None        {
            throw NullPointerException::new("item == null");
        }        
        set0(n, &item);
    }
    pub fn set(&self, n: i32, declaringClass: &CstType, bootstrapMethodHandle: &CstMethodHandle, arguments: &BootstrapMethodArgumentsList)    {
        set_int_Item(n, Item::new(&declaringClass, &bootstrapMethodHandle, &arguments));
    }
    pub fn concat(list1: &BootstrapMethodsList, list2: &BootstrapMethodsList) -> BootstrapMethodsList    {
        if list1==BootstrapMethodsList::EMPTY        {
            return list2;
        }        else         if list2==BootstrapMethodsList::EMPTY        {
            return list1;
        }        
        let sz1: i32 = list1.size();
        let sz2: i32 = list2.size();
        let result: BootstrapMethodsList = BootstrapMethodsList::new(sz1+sz2);
        for(        let i: i32 = 0;;i<sz1i += 1)        {
            result.set_int_Item(i, list1.get(i));
        }
        for(        let i: i32 = 0;;i<sz2i += 1)        {
            result.set_int_Item(sz1+i, list2.get(i));
        }
        return result;
    }
}
