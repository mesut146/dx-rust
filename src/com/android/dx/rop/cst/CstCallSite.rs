use crate::helper;
use crate::com::android::dx::rop::cst::CstArray::List;
use crate::com::android::dx::cf::code::BootstrapMethodArgumentsList;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::cst::CstCallSite;
use crate::com::android::dx::rop::cst::CstNat;

struct CstCallSite{
}
impl CstCallSite{
    pub fn make(bootstrapHandle: &CstMethodHandle, nat: &CstNat, optionalArguments: &BootstrapMethodArgumentsList) -> CstCallSite    {
        if bootstrapHandle==None        {
            throw NullPointerException::new("bootstrapMethodHandle == null");
        }        else         if nat==None        {
            throw NullPointerException::new("nat == null");
        }        
        let list: List = List::new(3+optionalArguments.size());
        list.set(0, &bootstrapHandle);
        list.set(1, nat.getName());
        list.set(2, CstProtoRef::new(Prototype::fromDescriptor(nat.getDescriptor().getString())));
        if optionalArguments!=None        {
            for(            let i: i32 = 0;;i<optionalArguments.size()++i)            {
                list.set(i+3, optionalArguments.get(i));
            }
        }        
        list.setImmutable();
        return CstCallSite::new(&list);
    }
    pub fn new(&self, list: &List)    {
        super(list);

    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if //other instanceof CstCallSite        {
            return getList().equals(((CstCallSite*)other).getList());
        }        else         {
            return false;
        }
    }
    pub fn hashCode(&self) -> i32    {
        return getList().hashCode();
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        return getList().compareTo(((CstCallSite*)other).getList());
    }
    pub fn toString(&self) -> String    {
        return getList().toString_String_String_String("call site{", ", ", "}");
    }
    pub fn typeName(&self) -> String    {
        return "call site";
    }
    pub fn isCategory2(&self) -> boolean    {
        return false;
    }
    pub fn toHuman(&self) -> String    {
        return getList().toHuman_String_String_String("{", ", ", "}");
    }
}
