use crate::helper;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::rop::cst::CstNat;

struct CstBaseMethodRef{
    pub prototype: Prototype,
    pub instancePrototype: Prototype,
}
impl CstBaseMethodRef{
    pub fn new(&self, definingClass: &CstType, nat: &CstNat)    {
        super(definingClass,nat);

        let descriptor: String = getNat().getDescriptor().getString();
        if isSignaturePolymorphic()        {
            self->prototype=Prototype::fromDescriptor(&descriptor);
        }        else         {
            self->prototype=Prototype::intern_String(&descriptor);
        }
        self->instancePrototype=None;
    }
    pub fn getPrototype(&self) -> Prototype    {
        return self.prototype;
    }
    pub fn getPrototype(&self, isStatic: boolean) -> Prototype    {
        if isStatic        {
            return self.prototype;
        }        else         {
            if self.instancePrototype==None            {
                let thisType: Type = getDefiningClass().getClassType();
                self.instancePrototype=self.prototype.withFirstParameter(&thisType);
            }            
            return self.instancePrototype;
        }
    }
    pub fn compareTo0(&self, other: &Constant) -> i32    {
        let cmp: i32 = super.compareTo0(other);
        if cmp!=0        {
            return cmp;
        }        
        let otherMethod: CstBaseMethodRef = (CstBaseMethodRef*)other;
        return self.prototype.compareTo(&);
    }
    pub fn getType(&self) -> Type    {
        return self.prototype.getReturnType();
    }
    pub fn getParameterWordCount(&self, isStatic: boolean) -> i32    {
        return getPrototype_boolean(isStatic).getParameterTypes().getWordCount();
    }
    pub fn isInstanceInit(&self) -> boolean    {
        return getNat().isInstanceInit();
    }
    pub fn isClassInit(&self) -> boolean    {
        return getNat().isClassInit();
    }
    pub fn isSignaturePolymorphic(&self) -> boolean    {
        let definingClass: CstType = getDefiningClass();
        if definingClass.equals(&CstType::METHOD_HANDLE)        {
            match getNat().getName().getString(){"invoke" => "invokeExact" =>                 return true;            }
        }        else         if definingClass.equals(&CstType::VAR_HANDLE)        {
            match getNat().getName().getString(){"compareAndExchange" => "compareAndExchangeAcquire" => "compareAndExchangeRelease" => "compareAndSet" => "get" => "getAcquire" => "getAndAdd" => "getAndAddAcquire" => "getAndAddRelease" => "getAndBitwiseAnd" => "getAndBitwiseAndAcquire" => "getAndBitwiseAndRelease" => "getAndBitwiseOr" => "getAndBitwiseOrAcquire" => "getAndBitwiseOrRelease" => "getAndBitwiseXor" => "getAndBitwiseXorAcquire" => "getAndBitwiseXorRelease" => "getAndSet" => "getAndSetAcquire" => "getAndSetRelease" => "getOpaque" => "getVolatile" => "set" => "setOpaque" => "setRelease" => "setVolatile" => "weakCompareAndSet" => "weakCompareAndSetAcquire" => "weakCompareAndSetPlain" => "weakCompareAndSetRelease" =>                 return true;            }
        }        
        return false;
    }
}
