use crate::helper;
use crate::com::android::dx::cf::code::Machine;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::cf::code::ExecutionStack;
use crate::com::android::dx::cf::code::ByteOps;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::cf::code::Merger;
use crate::com::android::dx::rop::cst::CstInterfaceMethodRef;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::cf::code::ByteBlock;
use crate::com::android::dx::cf::code::Simulator::SimVisitor;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::cf::code::BytecodeArray;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::cf::code::Frame;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dex::DexFormat;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::cf::code::LocalVariableList;
use crate::com::android::dx::cf::code::LocalVariableList::Item;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::cst::CstNat;

let static LOCAL_MISMATCH_ERROR: String = "This is symptomatic of .class transformation tools that ignore "+"local variable information.";
struct Simulator{
    pub machine: Machine,
    pub code: BytecodeArray,
    pub method: ConcreteMethod,
    pub localVariables: LocalVariableList,
    pub visitor: SimVisitor,
    pub dexOptions: DexOptions,
}
impl Simulator{
    pub fn new(&self, machine: &Machine, method: &ConcreteMethod, dexOptions: &DexOptions)    {
        if machine==None        {
            throw NullPointerException::new("machine == null");
        }        
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        if dexOptions==None        {
            throw NullPointerException::new("dexOptions == null");
        }        
        self->machine=machine;
        self->code=method.getCode();
        self->method=method;
        self->localVariables=method.getLocalVariables();
        self->visitor=SimVisitor::new(, self);
        self->dexOptions=dexOptions;
        if method.isDefaultOrStaticInterfaceMethod()        {
            checkInterfaceMethodDeclaration(&method);
        }        
    }
    pub fn simulate(&self, bb: &ByteBlock, frame: &Frame)    {
        let end: i32 = bb.getEnd();
        self.visitor.setFrame(&frame);
        try        {
            for(            let off: i32 = bb.getStart();;off<end)            {
                let length: i32 = self.code.parseInstruction(off, &self.visitor);
                self.visitor.setPreviousOffset(off);
                off+=length;
            }
        }        catch(        let ex: SimException)        {
            frame.annotate(&ex);
            throw ex;
        }
    }
    pub fn simulate(&self, offset: i32, frame: &Frame) -> i32    {
        self.visitor.setFrame(&frame);
        return self.code.parseInstruction(offset, &self.visitor);
    }
    pub fn illegalTos() -> SimException    {
        return SimException::new("stack mismatch: illegal "+"top-of-stack for opcode");
    }
    pub fn requiredArrayTypeFor(impliedType: &Type, foundArrayType: &Type) -> Type    {
        if foundArrayType==Type::KNOWN_NULL        {
            return if impliedType.isReference() { Type::KNOWN_NULL } else { impliedType.getArrayType() };
                }                
                if (impliedType==Type::OBJECT)&&foundArrayType.isArray()&&foundArrayType.getComponentType().isReference()                {
                    return foundArrayType;
                }                
                if (impliedType==Type::BYTE)&&(foundArrayType==Type::BOOLEAN_ARRAY)                {
                    return Type::BOOLEAN_ARRAY;
                }                
                return impliedType.getArrayType();
            }
            pub fn checkConstMethodHandleSupported(&self, cst: &Constant)            {
                if !self.dexOptions.apiIsSupported(DexFormat::API_CONST_METHOD_HANDLE)                {
                    fail(String::format_String_Object[]("invalid constant type %s requires --min-sdk-version >= %d "+"(currently %d)", cst.typeName(), DexFormat::API_CONST_METHOD_HANDLE, ));
                }                
            }
            pub fn checkInvokeDynamicSupported(&self, opcode: i32)            {
                if !self.dexOptions.apiIsSupported(DexFormat::API_METHOD_HANDLES)                {
                    fail(String::format_String_Object[]("invalid opcode %02x - invokedynamic requires "+"--min-sdk-version >= %d (currently %d)", opcode, DexFormat::API_METHOD_HANDLES, ));
                }                
            }
            pub fn checkInvokeInterfaceSupported(&self, opcode: i32, callee: &CstMethodRef)            {
                if opcode==ByteOps::INVOKEINTERFACE                {
                    return;
                }                
                if self.dexOptions.apiIsSupported(DexFormat::API_INVOKE_INTERFACE_METHODS)                {
                    return;
                }                
                let softFail: boolean = ;
                if opcode==ByteOps::INVOKESTATIC                {
                    softFail&=self.dexOptions.apiIsSupported(DexFormat::API_INVOKE_STATIC_INTERFACE_METHODS);
                }                else                 {
//assert (opcode == ByteOps.INVOKESPECIAL) || (opcode == ByteOps.INVOKEVIRTUAL);

                }
                let invokeKind: String = if (opcode==ByteOps::INVOKESTATIC) { "static" } else { "default" };
                        if softFail                        {
                            let reason: String = String::format_String_Object[]("invoking a %s interface method %s.%s strictly requires "+"--min-sdk-version >= %d (experimental at current API level %d)", &invokeKind, callee.getDefiningClass().toHuman(), callee.getNat().toHuman(), DexFormat::API_INVOKE_INTERFACE_METHODS, );
                            warn(&reason);
                        }                        else                         {
                            let reason: String = String::format_String_Object[]("invoking a %s interface method %s.%s strictly requires "+"--min-sdk-version >= %d (blocked at current API level %d)", &invokeKind, callee.getDefiningClass().toHuman(), callee.getNat().toHuman(), DexFormat::API_INVOKE_INTERFACE_METHODS, );
                            fail(&reason);
                        }
                    }
                    pub fn checkInterfaceMethodDeclaration(&self, declaredMethod: &ConcreteMethod)                    {
                        if !self.dexOptions.apiIsSupported(DexFormat::API_DEFINE_INTERFACE_METHODS)                        {
                            let reason: String = String::format_String_Object[]("defining a %s interface method requires --min-sdk-version >= %d (currently %d)"+" for interface methods: %s.%s", if declaredMethod.isStaticMethod() { "static" } else { "default" }, DexFormat::API_DEFINE_INTERFACE_METHODS, , declaredMethod.getDefiningClass().toHuman(), declaredMethod.getNat().toHuman());
                                    warn(&reason);
                                }                                
                            }
                            pub fn checkInvokeSignaturePolymorphic(&self, opcode: i32)                            {
                                if !self.dexOptions.apiIsSupported(DexFormat::API_METHOD_HANDLES)                                {
                                    fail(String::format_String_Object[]("invoking a signature-polymorphic requires --min-sdk-version >= %d (currently %d)", DexFormat::API_METHOD_HANDLES, ));
                                }                                else                                 if opcode!=ByteOps::INVOKEVIRTUAL                                {
                                    fail("Unsupported signature polymorphic invocation ("+ByteOps::opName(opcode)+")");
                                }                                
                            }
                            pub fn fail(&self, reason: &String)                            {
                                let message: String = String::format_String_Object[]("ERROR in %s.%s: %s", self.method.getDefiningClass().toHuman(), self.method.getNat().toHuman(), &reason);
                                throw SimException::new(&message);
                            }
                            pub fn warn(&self, reason: &String)                            {
                                let warning: String = String::format_String_Object[]("WARNING in %s.%s: %s", self.method.getDefiningClass().toHuman(), self.method.getNat().toHuman(), &reason);
                                .println_String(&warning);
                            }
}
