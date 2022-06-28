use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::InsnList;

struct InsnList{
}
impl InsnList{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Insn    {
        return (Insn*)get0(n);
    }
    pub fn set(&self, n: i32, insn: &Insn)    {
        set0(n, &insn);
    }
    pub fn getLast(&self) -> Insn    {
        return get(size()-1);
    }
    pub fn forEach(&self, visitor: &Insn.Visitor)    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            get(i).accept(&visitor);
        }
    }
    pub fn contentEquals(&self, b: &InsnList) -> boolean    {
        if b==None{            return false;        }        
        let sz: i32 = size();
        if sz!=b.size(){            return false;        }        
        for(        let i: i32 = 0;;i<szi += 1)        {
            if !get(i).contentEquals(b.get(i))            {
                return false;
            }            
        }
        return true;
    }
    pub fn withRegisterOffset(&self, delta: i32) -> InsnList    {
        let sz: i32 = size();
        let result: InsnList = InsnList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Insn = (Insn*)get0(i);
            if one!=None            {
                result.set0(i, one.withRegisterOffset(delta));
            }            
        }
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
}
