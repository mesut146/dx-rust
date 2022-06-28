use crate::helper;
use crate::com::android::dx::cf::iface::Attribute;

struct StdAttributeList{
}
impl StdAttributeList{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Attribute    {
        return (Attribute*)get0(n);
    }
    pub fn byteLength(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 2;
        for(        let i: i32 = 0;;i<szi += 1)        {
            result+=get(i).byteLength();
        }
        return result;
    }
    pub fn findFirst(&self, name: &String) -> Attribute    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let att: Attribute = get(i);
            if att.getName().equals(&name)            {
                return att;
            }            
        }
        return None;
    }
    pub fn findNext(&self, attrib: &Attribute) -> Attribute    {
        let sz: i32 = size();
        let at: i32;
        outer:
        let name: String = attrib.getName();
        for(at += 1;at<szat += 1)        {
            let att: Attribute = get(at);
            if att.getName().equals(&name)            {
                return att;
            }            
        }
        return None;
    }
    pub fn set(&self, n: i32, attribute: &Attribute)    {
        set0(n, &attribute);
    }
}
