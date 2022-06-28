use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::cf::iface::MethodList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::cf::code::ExecutionStack;
use crate::com::android::dx::cf::code::ByteOps;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::cf::code::ReturnAddress;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::TranslationAdvice;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::cf::code::SwitchList;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::cf::code::Frame;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::cf::code::Ropper;
use crate::com::android::dx::rop::code::FillArrayDataInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::InvokePolymorphicInsn;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::rop::code::SwitchInsn;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::rop::cst::CstNat;

let static ARRAY_REFLECT_TYPE: CstType = CstType::new(Type::internClassName("java/lang/reflect/Array"));
let static MULTIANEWARRAY_METHOD: CstMethodRef = CstMethodRef::new(&RopperMachine::ARRAY_REFLECT_TYPE, CstNat::new(CstString::new("newInstance"), CstString::new("(Ljava/lang/Class;[I)"+"Ljava/lang/Object;")));
struct RopperMachine{
    pub ropper: Ropper,
    pub method: ConcreteMethod,
    pub methods: MethodList,
    pub advice: TranslationAdvice,
    pub maxLocals: i32,
    pub insns: ArrayList<Insn>,
    pub catches: TypeList,
    pub catchesUsed: boolean,
    pub returns: boolean,
    pub primarySuccessorIndex: i32,
    pub extraBlockCount: i32,
    pub hasJsr: boolean,
    pub blockCanThrow: boolean,
    pub returnAddress: ReturnAddress,
    pub returnOp: Rop,
    pub returnPosition: SourcePosition,
}
impl RopperMachine{
    pub fn new(&self, ropper: &Ropper, method: &ConcreteMethod, advice: &TranslationAdvice, methods: &MethodList)    {
        super(method.getEffectiveDescriptor());

        if methods==None        {
            throw NullPointerException::new("methods == null");
        }        
        if ropper==None        {
            throw NullPointerException::new("ropper == null");
        }        
        if advice==None        {
            throw NullPointerException::new("advice == null");
        }        
        self->ropper=ropper;
        self->method=method;
        self->methods=methods;
        self->advice=advice;
        self->maxLocals=method.getMaxLocals();
        self->insns=ArrayList<Insn>::new(25);
        self->catches=None;
        self->catchesUsed=false;
        self->returns=false;
        self->primarySuccessorIndex=-1;
        self->extraBlockCount=0;
        self->blockCanThrow=false;
        self->returnOp=None;
        self->returnPosition=None;
    }
    pub fn getInsns(&self) -> ArrayList<Insn>    {
        return self.insns;
    }
    pub fn getReturnOp(&self) -> Rop    {
        return self.returnOp;
    }
    pub fn getReturnPosition(&self) -> SourcePosition    {
        return self.returnPosition;
    }
    pub fn startBlock(&self, catches: &TypeList)    {
        self->catches=catches;
        self.insns.clear();
        self.catchesUsed=false;
        self.returns=false;
        self.primarySuccessorIndex=0;
        self.extraBlockCount=0;
        self.blockCanThrow=false;
        self.hasJsr=false;
        self.returnAddress=None;
    }
    pub fn wereCatchesUsed(&self) -> boolean    {
        return self.catchesUsed;
    }
    pub fn returns(&self) -> boolean    {
        return self.returns;
    }
    pub fn getPrimarySuccessorIndex(&self) -> i32    {
        return self.primarySuccessorIndex;
    }
    pub fn getExtraBlockCount(&self) -> i32    {
        return self.extraBlockCount;
    }
    pub fn canThrow(&self) -> boolean    {
        return self.blockCanThrow;
    }
    pub fn hasJsr(&self) -> boolean    {
        return self.hasJsr;
    }
    pub fn hasRet(&self) -> boolean    {
        return self.returnAddress!=None;
    }
    pub fn getReturnAddress(&self) -> ReturnAddress    {
        return self.returnAddress;
    }
    pub fn run(&self, frame: &Frame, offset: i32, opcode: i32)    {
        let stackPointer: i32 = self.maxLocals+frame.getStack().size();
        let sources: RegisterSpecList = getSources(opcode, stackPointer);
        let sourceCount: i32 = sources.size();
        super.run(frame,offset,opcode);
        let pos: SourcePosition = self.method.makeSourcePosistion(offset);
        let localTarget: RegisterSpec = getLocalTarget(opcode==ByteOps::ISTORE);
        let destCount: i32 = resultCount();
        let dest: RegisterSpec;
        if destCount==0        {
            dest=None;
            match opcode{ByteOps::POP => ByteOps::POP2 =>                 {
                    return;
                }            }
        }        else         if localTarget!=None        {
            dest=localTarget;
        }        else         if destCount==1        {
            dest=RegisterSpec::make_int_TypeBearer(stackPointer, result(0));
        }        else         {
            let scratchAt: i32 = self.ropper.getFirstTempStackReg();
            let scratchRegs: Vec<RegisterSpec> = new RegisterSpec[sourceCount];
            for(            let i: i32 = 0;;i<sourceCounti += 1)            {
                let src: RegisterSpec = sources.get(i);
                let type: TypeBearer = src.getTypeBearer();
                let scratch: RegisterSpec = src.withReg(scratchAt);
                self.insns.add_Insn(PlainInsn::new(Rops::opMove(&type_renamed), &pos, &scratch, &src));
                scratchRegs[i]=scratch;
                scratchAt+=src.getCategory();
            }
            for(            let pattern: i32 = getAuxInt();;pattern!=0pattern>>=4)            {
                let which: i32 = (pattern&0x0f)-1;
                let scratch: RegisterSpec = scratchRegs[which];
                let type: TypeBearer = scratch.getTypeBearer();
                self.insns.add_Insn(PlainInsn::new(Rops::opMove(&type_renamed), &pos, scratch.withReg(stackPointer), &scratch));
                stackPointer+=type_renamed.getType().getCategory();
            }
            return;
        }
        let destType: TypeBearer = if (dest!=None) { dest } else { Type::VOID };
                let cst: Constant = getAuxCst();
                let ropOpcode: i32;
                let rop: Rop;
                let insn: Insn;
                if opcode==ByteOps::MULTIANEWARRAY                {
                    self.blockCanThrow=true;
                    self.extraBlockCount=6;
                    let dimsReg: RegisterSpec = RegisterSpec::make_int_TypeBearer(dest.getNextReg(), &Type::INT_ARRAY);
                    rop=Rops::opFilledNewArray(&Type::INT_ARRAY, sourceCount);
                    insn=ThrowingCstInsn::new(&rop, &pos, &sources, &self.catches, &CstType::INT_ARRAY);
                    self.insns.add_Insn(&insn);
                    rop=Rops::opMoveResult(&Type::INT_ARRAY);
                    insn=PlainInsn::new(&rop, &pos, &dimsReg, &RegisterSpecList::EMPTY);
                    self.insns.add_Insn(&insn);
                    let componentType: Type = ((CstType*)cst).getClassType();
                    for(                    let i: i32 = 0;;i<sourceCounti += 1)                    {
                        componentType=componentType.getComponentType();
                    }
                    let classReg: RegisterSpec = RegisterSpec::make_int_TypeBearer(dest.getReg(), &Type::CLASS);
                    if componentType.isPrimitive()                    {
                        let typeField: CstFieldRef = CstFieldRef::forPrimitiveType(&componentType);
                        insn=ThrowingCstInsn::new(&Rops::GET_STATIC_OBJECT, &pos, &RegisterSpecList::EMPTY, &self.catches, &typeField);
                    }                    else                     {
                        insn=ThrowingCstInsn::new(&Rops::CONST_OBJECT, &pos, &RegisterSpecList::EMPTY, &self.catches, CstType::new(&componentType));
                    }
                    self.insns.add_Insn(&insn);
                    rop=Rops::opMoveResultPseudo(classReg.getType());
                    insn=PlainInsn::new(&rop, &pos, &classReg, &RegisterSpecList::EMPTY);
                    self.insns.add_Insn(&insn);
                    let objectReg: RegisterSpec = RegisterSpec::make_int_TypeBearer(dest.getReg(), &Type::OBJECT);
                    insn=ThrowingCstInsn::new(Rops::opInvokeStatic(RopperMachine::MULTIANEWARRAY_METHOD.getPrototype()), &pos, RegisterSpecList::make_RegisterSpec_RegisterSpec(&classReg, &dimsReg), &self.catches, &RopperMachine::MULTIANEWARRAY_METHOD);
                    self.insns.add_Insn(&insn);
                    rop=Rops::opMoveResult(RopperMachine::MULTIANEWARRAY_METHOD.getPrototype().getReturnType());
                    insn=PlainInsn::new(&rop, &pos, &objectReg, &RegisterSpecList::EMPTY);
                    self.insns.add_Insn(&insn);
                    opcode=ByteOps::CHECKCAST;
                    sources=RegisterSpecList::make_RegisterSpec(&objectReg);
                }                else                 if opcode==ByteOps::JSR                {
                    self.hasJsr=true;
                    return;
                }                else                 if opcode==ByteOps::RET                {
                    try                    {
                        self.returnAddress=(ReturnAddress*)arg(0);
                    }                    catch(                    let ex: ClassCastException)                    {
                        throw RuntimeException::new("Argument to RET was not a ReturnAddress", &ex);
                    }
                    return;
                }                
                ropOpcode=jopToRopOpcode(opcode, &cst);
                rop=Rops::ropFor(ropOpcode, &destType, &sources, &cst);
                let moveResult: Insn = None;
                if dest!=None&&rop.isCallLike()                {
                    self.extraBlockCount += 1;
                    let returnType: Type;
                    if rop.getOpcode()==RegOps::INVOKE_CUSTOM                    {
                        returnType=((CstCallSiteRef*)cst).getReturnType();
                    }                    else                     {
                        returnType=((CstMethodRef*)cst).getPrototype().getReturnType();
                    }
                    moveResult=PlainInsn::new(Rops::opMoveResult(&returnType), &pos, &dest, &RegisterSpecList::EMPTY);
                    dest=None;
                }                else                 if dest!=None&&rop.canThrow()                {
                    self.extraBlockCount += 1;
                    moveResult=PlainInsn::new(Rops::opMoveResultPseudo(dest.getTypeBearer()), &pos, &dest, &RegisterSpecList::EMPTY);
                    dest=None;
                }                
                if ropOpcode==RegOps::NEW_ARRAY                {
                    cst=CstType::intern(rop.getResult());
                }                else                 if (cst==None)&&(sourceCount==2)                {
                    let firstType: TypeBearer = sources.get(0).getTypeBearer();
                    let lastType: TypeBearer = sources.get(1).getTypeBearer();
                    if (lastType.isConstant()||firstType.isConstant())&&self.advice.hasConstantOperation(&rop, sources.get(0), sources.get(1))                    {
                        if lastType.isConstant()                        {
                            cst=(Constant*)lastType;
                            sources=sources.withoutLast();
                            if rop.getOpcode()==RegOps::SUB                            {
                                ropOpcode=RegOps::ADD;
                                let cstInt: CstInteger = (CstInteger*)lastType;
                                cst=CstInteger::make(-cstInt.getValue());
                            }                            
                        }                        else                         {
                            cst=(Constant*)firstType;
                            sources=sources.withoutFirst();
                        }
                        rop=Rops::ropFor(ropOpcode, &destType, &sources, &cst);
                    }                    
                }                
                let cases: SwitchList = getAuxCases();
                let initValues: ArrayList<Constant> = getInitValues();
                let canThrow: boolean = rop.canThrow();
                self.blockCanThrow|=canThrow;
                if cases!=None                {
                    if cases.size()==0                    {
                        insn=PlainInsn::new(&Rops::GOTO, &pos, None, &RegisterSpecList::EMPTY);
                        self.primarySuccessorIndex=0;
                    }                    else                     {
                        let values: IntList = cases.getValues();
                        insn=SwitchInsn::new(&rop, &pos, &dest, &sources, &values);
                        self.primarySuccessorIndex=values.size();
                    }
                }                else                 if ropOpcode==RegOps::RETURN                {
                    if sources.size()!=0                    {
                        let source: RegisterSpec = sources.get(0);
                        let type: TypeBearer = source.getTypeBearer();
                        if source.getReg()!=0                        {
                            self.insns.add_Insn(PlainInsn::new(Rops::opMove(&type_renamed), &pos, RegisterSpec::make_int_TypeBearer(0, &type_renamed), &source));
                        }                        
                    }                    
                    insn=PlainInsn::new(&Rops::GOTO, &pos, None, &RegisterSpecList::EMPTY);
                    self.primarySuccessorIndex=0;
                    updateReturnOp(&rop, &pos);
                    self.returns=true;
                }                else                 if cst!=None                {
                    if canThrow                    {
                        if rop.getOpcode()==RegOps::INVOKE_POLYMORPHIC                        {
                            insn=makeInvokePolymorphicInsn(&rop, &pos, &sources, &self.catches, &cst);
                        }                        else                         {
                            insn=ThrowingCstInsn::new(&rop, &pos, &sources, &self.catches, &cst);
                        }
                        self.catchesUsed=true;
                        self.primarySuccessorIndex=self.catches.size();
                    }                    else                     {
                        insn=PlainCstInsn::new(&rop, &pos, &dest, &sources, &cst);
                    }
                }                else                 if canThrow                {
                    insn=ThrowingInsn::new(&rop, &pos, &sources, &self.catches);
                    self.catchesUsed=true;
                    if opcode==ByteOps::ATHROW                    {
                        self.primarySuccessorIndex=-1;
                    }                    else                     {
                        self.primarySuccessorIndex=self.catches.size();
                    }
                }                else                 {
                    insn=PlainInsn::new(&rop, &pos, &dest, &sources);
                }
                self.insns.add_Insn(&insn);
                if moveResult!=None                {
                    self.insns.add_Insn(&moveResult);
                }                
                if initValues!=None                {
                    self.extraBlockCount += 1;
                    insn=FillArrayDataInsn::new(&Rops::FILL_ARRAY_DATA, &pos, RegisterSpecList::make_RegisterSpec(moveResult.getResult()), &initValues, &cst);
                    self.insns.add_Insn(&insn);
                }                
            }
            pub fn getSources(&self, opcode: i32, stackPointer: i32) -> RegisterSpecList            {
                let count: i32 = argCount();
                if count==0                {
                    return RegisterSpecList::EMPTY;
                }                
                let localIndex: i32 = getLocalIndex();
                let sources: RegisterSpecList;
                if localIndex>=0                {
                    sources=RegisterSpecList::new(1);
                    sources.set_int_RegisterSpec(0, RegisterSpec::make_int_TypeBearer(localIndex, arg(0)));
                }                else                 {
                    sources=RegisterSpecList::new(count);
                    let regAt: i32 = stackPointer;
                    for(                    let i: i32 = 0;;i<counti += 1)                    {
                        let spec: RegisterSpec = RegisterSpec::make_int_TypeBearer(regAt, arg(i));
                        sources.set_int_RegisterSpec(i, &spec);
                        regAt+=spec.getCategory();
                    }
                    match opcode{ByteOps::IASTORE =>                         {
                            if count!=3                            {
                                throw RuntimeException::new("shouldn't happen");
                            }                            
                            let array: RegisterSpec = sources.get(0);
                            let index: RegisterSpec = sources.get(1);
                            let value: RegisterSpec = sources.get(2);
                            sources.set_int_RegisterSpec(0, &value);
                            sources.set_int_RegisterSpec(1, &array);
                            sources.set_int_RegisterSpec(2, &index);
                            break;
                        }ByteOps::PUTFIELD =>                         {
                            if count!=2                            {
                                throw RuntimeException::new("shouldn't happen");
                            }                            
                            let obj: RegisterSpec = sources.get(0);
                            let value: RegisterSpec = sources.get(1);
                            sources.set_int_RegisterSpec(0, &value);
                            sources.set_int_RegisterSpec(1, &obj);
                            break;
                        }                    }
                }
                sources.setImmutable();
                return sources;
            }
            pub fn updateReturnOp(&self, op: &Rop, pos: &SourcePosition)            {
                if op==None                {
                    throw NullPointerException::new("op == null");
                }                
                if pos==None                {
                    throw NullPointerException::new("pos == null");
                }                
                if self.returnOp==None                {
                    self.returnOp=op;
                    self.returnPosition=pos;
                }                else                 {
                    if self.returnOp!=op                    {
                        throw SimException::new("return op mismatch: "+op+", "+self.returnOp);
                    }                    
                    if pos.getLine()>self.returnPosition.getLine()                    {
                        self.returnPosition=pos;
                    }                    
                }
            }
            pub fn jopToRopOpcode(&self, jop: i32, cst: &Constant) -> i32            {
                match jop{ByteOps::POP => ByteOps::POP2 => ByteOps::DUP => ByteOps::DUP_X1 => ByteOps::DUP_X2 => ByteOps::DUP2 => ByteOps::DUP2_X1 => ByteOps::DUP2_X2 => ByteOps::SWAP => ByteOps::JSR => ByteOps::RET => ByteOps::MULTIANEWARRAY =>                     {
                        break;
                    }ByteOps::NOP =>                     {
                        return RegOps::NOP;
                    }ByteOps::LDC => ByteOps::LDC2_W =>                     {
                        return RegOps::CONST;
                    }ByteOps::ILOAD => ByteOps::ISTORE =>                     {
                        return RegOps::MOVE;
                    }ByteOps::IALOAD =>                     {
                        return RegOps::AGET;
                    }ByteOps::IASTORE =>                     {
                        return RegOps::APUT;
                    }ByteOps::IADD => ByteOps::IINC =>                     {
                        return RegOps::ADD;
                    }ByteOps::ISUB =>                     {
                        return RegOps::SUB;
                    }ByteOps::IMUL =>                     {
                        return RegOps::MUL;
                    }ByteOps::IDIV =>                     {
                        return RegOps::DIV;
                    }ByteOps::IREM =>                     {
                        return RegOps::REM;
                    }ByteOps::INEG =>                     {
                        return RegOps::NEG;
                    }ByteOps::ISHL =>                     {
                        return RegOps::SHL;
                    }ByteOps::ISHR =>                     {
                        return RegOps::SHR;
                    }ByteOps::IUSHR =>                     {
                        return RegOps::USHR;
                    }ByteOps::IAND =>                     {
                        return RegOps::AND;
                    }ByteOps::IOR =>                     {
                        return RegOps::OR;
                    }ByteOps::IXOR =>                     {
                        return RegOps::XOR;
                    }ByteOps::I2L => ByteOps::I2F => ByteOps::I2D => ByteOps::L2I => ByteOps::L2F => ByteOps::L2D => ByteOps::F2I => ByteOps::F2L => ByteOps::F2D => ByteOps::D2I => ByteOps::D2L => ByteOps::D2F =>                     {
                        return RegOps::CONV;
                    }ByteOps::I2B =>                     {
                        return RegOps::TO_BYTE;
                    }ByteOps::I2C =>                     {
                        return RegOps::TO_CHAR;
                    }ByteOps::I2S =>                     {
                        return RegOps::TO_SHORT;
                    }ByteOps::LCMP => ByteOps::FCMPL => ByteOps::DCMPL =>                     {
                        return RegOps::CMPL;
                    }ByteOps::FCMPG => ByteOps::DCMPG =>                     {
                        return RegOps::CMPG;
                    }ByteOps::IFEQ => ByteOps::IF_ICMPEQ => ByteOps::IF_ACMPEQ => ByteOps::IFNULL =>                     {
                        return RegOps::IF_EQ;
                    }ByteOps::IFNE => ByteOps::IF_ICMPNE => ByteOps::IF_ACMPNE => ByteOps::IFNONNULL =>                     {
                        return RegOps::IF_NE;
                    }ByteOps::IFLT => ByteOps::IF_ICMPLT =>                     {
                        return RegOps::IF_LT;
                    }ByteOps::IFGE => ByteOps::IF_ICMPGE =>                     {
                        return RegOps::IF_GE;
                    }ByteOps::IFGT => ByteOps::IF_ICMPGT =>                     {
                        return RegOps::IF_GT;
                    }ByteOps::IFLE => ByteOps::IF_ICMPLE =>                     {
                        return RegOps::IF_LE;
                    }ByteOps::GOTO =>                     {
                        return RegOps::GOTO;
                    }ByteOps::LOOKUPSWITCH =>                     {
                        return RegOps::SWITCH;
                    }ByteOps::IRETURN => ByteOps::RETURN =>                     {
                        return RegOps::RETURN;
                    }ByteOps::GETSTATIC =>                     {
                        return RegOps::GET_STATIC;
                    }ByteOps::PUTSTATIC =>                     {
                        return RegOps::PUT_STATIC;
                    }ByteOps::GETFIELD =>                     {
                        return RegOps::GET_FIELD;
                    }ByteOps::PUTFIELD =>                     {
                        return RegOps::PUT_FIELD;
                    }ByteOps::INVOKEVIRTUAL =>                     {
                        let ref: CstMethodRef = (CstMethodRef*)cst;
                        if ref_renamed.getDefiningClass().equals(self.method.getDefiningClass())                        {
                            for(                            let i: i32 = 0;;i<self.methods.size()++i)                            {
                                let m: Method = self.methods.get(i);
                                if AccessFlags::isPrivate(m.getAccessFlags())&&ref_renamed.getNat().equals(m.getNat())                                {
                                    return RegOps::INVOKE_DIRECT;
                                }                                
                            }
                        }                        
                        if ref_renamed.isSignaturePolymorphic()                        {
                            return RegOps::INVOKE_POLYMORPHIC;
                        }                        
                        return RegOps::INVOKE_VIRTUAL;
                    }ByteOps::INVOKESPECIAL =>                     {
                        let ref: CstMethodRef = (CstMethodRef*)cst;
                        if ref_renamed.isInstanceInit()||(ref_renamed.getDefiningClass().equals(self.method.getDefiningClass()))                        {
                            return RegOps::INVOKE_DIRECT;
                        }                        
                        return RegOps::INVOKE_SUPER;
                    }ByteOps::INVOKESTATIC =>                     {
                        return RegOps::INVOKE_STATIC;
                    }ByteOps::INVOKEINTERFACE =>                     {
                        return RegOps::INVOKE_INTERFACE;
                    }ByteOps::INVOKEDYNAMIC =>                     {
                        return RegOps::INVOKE_CUSTOM;
                    }ByteOps::NEW =>                     {
                        return RegOps::NEW_INSTANCE;
                    }ByteOps::NEWARRAY => ByteOps::ANEWARRAY =>                     {
                        return RegOps::NEW_ARRAY;
                    }ByteOps::ARRAYLENGTH =>                     {
                        return RegOps::ARRAY_LENGTH;
                    }ByteOps::ATHROW =>                     {
                        return RegOps::THROW;
                    }ByteOps::CHECKCAST =>                     {
                        return RegOps::CHECK_CAST;
                    }ByteOps::INSTANCEOF =>                     {
                        return RegOps::INSTANCE_OF;
                    }ByteOps::MONITORENTER =>                     {
                        return RegOps::MONITOR_ENTER;
                    }ByteOps::MONITOREXIT =>                     {
                        return RegOps::MONITOR_EXIT;
                    }                }
                throw RuntimeException::new("shouldn't happen");
            }
            pub fn makeInvokePolymorphicInsn(&self, rop: &Rop, pos: &SourcePosition, sources: &RegisterSpecList, catches: &TypeList, cst: &Constant) -> Insn            {
                let cstMethodRef: CstMethodRef = (CstMethodRef*)cst;
                return InvokePolymorphicInsn::new(&rop, &pos, &sources, &catches, &cstMethodRef);
            }
}
