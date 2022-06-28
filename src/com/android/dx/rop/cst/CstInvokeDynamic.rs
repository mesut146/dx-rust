use crate::helper;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstCallSite;
use crate::com::android::dx::rop::cst::CstNat;

struct CstInvokeDynamic{
    pub bootstrapMethodIndex: i32,
    pub nat: CstNat,
    pub prototype: Prototype,
    pub declaringClass: CstType,
    pub callSite: CstCallSite,
    pub references: List<CstCallSiteRef>,
}
impl CstInvokeDynamic{
    pub fn make(bootstrapMethodIndex: i32, nat: &CstNat) -> CstInvokeDynamic    {
        return CstInvokeDynamic::new(bootstrapMethodIndex, &nat);
    }
    pub fn new(&self, bootstrapMethodIndex: i32, nat: &CstNat)    {
        self->bootstrapMethodIndex=bootstrapMethodIndex;
        self->nat=nat;
        self->prototype=Prototype::fromDescriptor(nat.getDescriptor().toHuman());
        self->references=ArrayList<>::new();
    }
    pub fn addReference(&self) -> CstCallSiteRef    {
        let ref: CstCallSiteRef = CstCallSiteRef::new(self, self.references.size());
        self.references.add_CstCallSiteRef(&ref_renamed);
        return ref_renamed;
    }
    pub fn getReferences(&self) -> List<CstCallSiteRef>    {
        return self.references;
    }
    pub fn toString(&self) -> String    {
        return toHuman();
    }
    pub fn typeName(&self) -> String    {
        return "InvokeDynamic";
    }
    pub fn toHuman(&self) -> String    {
        let klass: String = if (self.declaringClass!=None) { self.declaringClass.toHuman() } else { "Unknown" };
                return "InvokeDynamic("+klass+":"+self.bootstrapMethodIndex+", "+self.nat.toHuman()+")";
            }
            pub fn isCategory2(&self) -> boolean            {
                return false;
            }
            pub fn compareTo0(&self, other: &Constant) -> i32            {
                let otherInvoke: CstInvokeDynamic = (CstInvokeDynamic*)other;
                let result: i32 = Integer::compare(self.bootstrapMethodIndex, otherInvoke.getBootstrapMethodIndex());
                if result!=0                {
                    return result;
                }                
                result=self.nat.compareTo(otherInvoke.getNat());
                if result!=0                {
                    return result;
                }                
                result=self.declaringClass.compareTo(otherInvoke.getDeclaringClass());
                if result!=0                {
                    return result;
                }                
                return self.callSite.compareTo(otherInvoke.getCallSite());
            }
            pub fn getBootstrapMethodIndex(&self) -> i32            {
                return self.bootstrapMethodIndex;
            }
            pub fn getNat(&self) -> CstNat            {
                return self.nat;
            }
            pub fn getPrototype(&self) -> Prototype            {
                return self.prototype;
            }
            pub fn getReturnType(&self) -> Type            {
                return self.prototype.getReturnType();
            }
            pub fn setDeclaringClass(&self, declaringClass: &CstType)            {
                if self->declaringClass!=None                {
                    throw IllegalArgumentException::new("already added declaring class");
                }                else                 if declaringClass==None                {
                    throw NullPointerException::new("declaringClass == null");
                }                
                self->declaringClass=declaringClass;
            }
            pub fn getDeclaringClass(&self) -> CstType            {
                return self.declaringClass;
            }
            pub fn setCallSite(&self, callSite: &CstCallSite)            {
                if self->callSite!=None                {
                    throw IllegalArgumentException::new("already added call site");
                }                else                 if callSite==None                {
                    throw NullPointerException::new("callSite == null");
                }                
                self->callSite=callSite;
            }
            pub fn getCallSite(&self) -> CstCallSite            {
                return self.callSite;
            }
}
