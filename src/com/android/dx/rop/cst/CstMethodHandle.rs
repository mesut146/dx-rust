use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstFieldRef;

let static TYPE_NAMES: Vec<String> = vec!["static-put", "static-get", "instance-put", "instance-get", "invoke-static", "invoke-instance", "invoke-constructor", "invoke-direct", "invoke-interface"];
struct CstMethodHandle{
    pub type: i32,
    pub ref: Constant,
}
impl CstMethodHandle{
    pub const METHOD_HANDLE_TYPE_STATIC_PUT: i32 = 0;
    pub const METHOD_HANDLE_TYPE_STATIC_GET: i32 = 1;
    pub const METHOD_HANDLE_TYPE_INSTANCE_PUT: i32 = 2;
    pub const METHOD_HANDLE_TYPE_INSTANCE_GET: i32 = 3;
    pub const METHOD_HANDLE_TYPE_INVOKE_STATIC: i32 = 4;
    pub const METHOD_HANDLE_TYPE_INVOKE_INSTANCE: i32 = 5;
    pub const METHOD_HANDLE_TYPE_INVOKE_CONSTRUCTOR: i32 = 6;
    pub const METHOD_HANDLE_TYPE_INVOKE_DIRECT: i32 = 7;
    pub const METHOD_HANDLE_TYPE_INVOKE_INTERFACE: i32 = 8;
    pub fn make(type: i32, ref: &Constant) -> CstMethodHandle    {
        if CstMethodHandle::isAccessor_int(type)        {
            if !(//ref instanceof CstFieldRef)            {
                throw IllegalArgumentException::new("ref has wrong type: "+ref.getClass());
            }            
        }        else         if CstMethodHandle::isInvocation_int(type)        {
            if !(//ref instanceof CstBaseMethodRef)            {
                throw IllegalArgumentException::new("ref has wrong type: "+ref.getClass());
            }            
        }        else         {
            throw IllegalArgumentException::new("type is out of range: "+type);
        }
        return CstMethodHandle::new(type, &ref);
    }
    pub fn new(&self, type: i32, ref: &Constant)    {
        self->type=type;
        self->ref=ref;
    }
    pub fn getRef(&self) -> Constant    {
        return self.ref_renamed;
    }
    pub fn getMethodHandleType(&self) -> i32    {
        return self.type_renamed;
    }
    pub fn isAccessor(type: i32) -> boolean    {
        match type{CstMethodHandle::METHOD_HANDLE_TYPE_STATIC_PUT => CstMethodHandle::METHOD_HANDLE_TYPE_STATIC_GET => CstMethodHandle::METHOD_HANDLE_TYPE_INSTANCE_PUT => CstMethodHandle::METHOD_HANDLE_TYPE_INSTANCE_GET =>             return true;        _ => {}        return false;    }
}
pub fn isAccessor(&self) -> boolean{
    return CstMethodHandle::isAccessor_int(self.type_renamed);
}
pub fn isInvocation(type: i32) -> boolean{
    match type{CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_STATIC => CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_INSTANCE => CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_CONSTRUCTOR => CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_DIRECT => CstMethodHandle::METHOD_HANDLE_TYPE_INVOKE_INTERFACE =>         return true;    _ => {}    return false;}
}
pub fn isInvocation(&self) -> boolean{
return CstMethodHandle::isInvocation_int(self.type_renamed);
}
pub fn getMethodHandleTypeName(type: i32) -> String{
return CstMethodHandle::TYPE_NAMES[type];
}
pub fn isCategory2(&self) -> boolean{
return false;
}
pub fn compareTo0(&self, other: &Constant) -> i32{
let otherHandle: CstMethodHandle = (CstMethodHandle*)other;
if getMethodHandleType()==otherHandle.getMethodHandleType(){
    return getRef().compareTo(otherHandle.getRef());
}else {
    return Integer::compare(getMethodHandleType(), otherHandle.getMethodHandleType());
}
}
pub fn toString(&self) -> String{
return "method-handle{"+toHuman()+"}";
}
pub fn typeName(&self) -> String{
return "method handle";
}
pub fn toHuman(&self) -> String{
return CstMethodHandle::getMethodHandleTypeName(self.type_renamed)+","+self.ref_renamed.toString();
}
pub fn getType(&self) -> Type{
return Type::METHOD_HANDLE;
}
}
