use crate::helper;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::code::InvokePolymorphicInsn;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstNat;

let static DEFAULT_DESCRIPTOR: CstString = CstString::new("([Ljava/lang/Object;)Ljava/lang/Object;");
let static VARHANDLE_SET_DESCRIPTOR: CstString = CstString::new("([Ljava/lang/Object;)V");
let static VARHANDLE_COMPARE_AND_SET_DESCRIPTOR: CstString = CstString::new("([Ljava/lang/Object;)Z");
struct InvokePolymorphicInsn{
    pub catches: TypeList,
    pub callSiteMethod: CstMethodRef,
    pub polymorphicMethod: CstMethodRef,
    pub callSiteProto: CstProtoRef,
}
impl InvokePolymorphicInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, sources: &RegisterSpecList, catches: &TypeList, callSiteMethod: &CstMethodRef)    {
        super(opcode,position,null,sources);

        if opcode.getBranchingness()!=Rop::BRANCH_THROW        {
            throw IllegalArgumentException::new("opcode with invalid branchingness: "+opcode.getBranchingness());
        }        
        if catches==None        {
            throw NullPointerException::new("catches == null");
        }        
        self->catches=catches;
        if callSiteMethod==None        {
            throw NullPointerException::new("callSiteMethod == null");
        }        else         if !callSiteMethod.isSignaturePolymorphic()        {
            throw IllegalArgumentException::new("callSiteMethod is not signature polymorphic");
        }        
        self->callSiteMethod=callSiteMethod;
        self->polymorphicMethod=InvokePolymorphicInsn::makePolymorphicMethod(&callSiteMethod);
        self->callSiteProto=InvokePolymorphicInsn::makeCallSiteProto(&callSiteMethod);
    }
    pub fn getCatches(&self) -> TypeList    {
        return self->catches;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitInvokePolymorphicInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        return InvokePolymorphicInsn::new(getOpcode(), getPosition(), getSources(), self.catches.withAddedType(&type), getCallSiteMethod());
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return InvokePolymorphicInsn::new(getOpcode(), getPosition(), getSources().withOffset(delta), &self.catches, getCallSiteMethod());
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return InvokePolymorphicInsn::new(getOpcode(), getPosition(), &sources, &self.catches, getCallSiteMethod());
    }
    pub fn getCallSiteMethod(&self) -> CstMethodRef    {
        return self.callSiteMethod;
    }
    pub fn getPolymorphicMethod(&self) -> CstMethodRef    {
        return self.polymorphicMethod;
    }
    pub fn getCallSiteProto(&self) -> CstProtoRef    {
        return self.callSiteProto;
    }
    pub fn getInlineString(&self) -> String    {
        return getPolymorphicMethod().toString()+" "+getCallSiteProto().toString()+" "+ThrowingInsn::toCatchString(&self.catches);
    }
    pub fn makePolymorphicMethod(callSiteMethod: &CstMethodRef) -> CstMethodRef    {
        let definingClass: CstType = callSiteMethod.getDefiningClass();
        let cstMethodName: CstString = callSiteMethod.getNat().getName();
        let methodName: String = callSiteMethod.getNat().getName().getString();
        if definingClass.equals(&CstType::METHOD_HANDLE)        {
            if methodName.equals("invoke")||methodName.equals("invokeExact")            {
                let cstNat: CstNat = CstNat::new(&cstMethodName, &InvokePolymorphicInsn::DEFAULT_DESCRIPTOR);
                return CstMethodRef::new(&definingClass, &cstNat);
            }            
        }        
        if definingClass.equals(&CstType::VAR_HANDLE)        {
            match methodName{"compareAndExchange" => "compareAndExchangeAcquire" => "compareAndExchangeRelease" => "get" => "getAcquire" => "getAndAdd" => "getAndAddAcquire" => "getAndAddRelease" => "getAndBitwiseAnd" => "getAndBitwiseAndAcquire" => "getAndBitwiseAndRelease" => "getAndBitwiseOr" => "getAndBitwiseOrAcquire" => "getAndBitwiseOrRelease" => "getAndBitwiseXor" => "getAndBitwiseXorAcquire" => "getAndBitwiseXorRelease" => "getAndSet" => "getAndSetAcquire" => "getAndSetRelease" => "getOpaque" => "getVolatile" =>                 {
                    let cstNat: CstNat = CstNat::new(&cstMethodName, &InvokePolymorphicInsn::DEFAULT_DESCRIPTOR);
                    return CstMethodRef::new(&definingClass, &cstNat);
                }"set" => "setOpaque" => "setRelease" => "setVolatile" =>                 {
                    let cstNat: CstNat = CstNat::new(&cstMethodName, &InvokePolymorphicInsn::VARHANDLE_SET_DESCRIPTOR);
                    return CstMethodRef::new(&definingClass, &cstNat);
                }"compareAndSet" => "weakCompareAndSet" => "weakCompareAndSetAcquire" => "weakCompareAndSetPlain" => "weakCompareAndSetRelease" =>                 {
                    let cstNat: CstNat = CstNat::new(&cstMethodName, &InvokePolymorphicInsn::VARHANDLE_COMPARE_AND_SET_DESCRIPTOR);
                    return CstMethodRef::new(&definingClass, &cstNat);
                }            _ => {}            break;        }
    }    
    throw IllegalArgumentException::new("Unknown signature polymorphic method: "+callSiteMethod.toHuman());
}
pub fn makeCallSiteProto(callSiteMethod: &CstMethodRef) -> CstProtoRef{
    return CstProtoRef::new(callSiteMethod.getPrototype_boolean(true));
}
}
