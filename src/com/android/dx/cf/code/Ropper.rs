use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::cf::code::ByteCatchList;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::rop::code::BasicBlock::Visitor;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::cf::code::Ropper::SubroutineInliner;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::cf::code::Ropper;
use crate::com::android::dx::cf::code::ByteBlockList;
use crate::com::android::dx::cf::code::RopperMachine;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::cf::code::ByteCatchList::Item;
use crate::com::android::dx::cf::code::Simulator;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::util::Bits;
use crate::com::android::dx::cf::code::LocalVariableList;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::cf::code::BasicBlocker;
use crate::com::android::dx::rop::cst::CstNat;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::cf::code::ReturnAddress;
use crate::com::android::dx::cf::code::Ropper::Subroutine;
use crate::com::android::dx::cf::code::ByteBlock;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::cf::code::Frame;
use crate::com::android::dx::cf::code::Ropper::ExceptionSetupLabelAllocator;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::cf::code::Ropper::CatchInfo;
use crate::com::android::dx::cf::code::Ropper::ExceptionHandlerSetup;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::code::Ropper::LabelAllocator;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::cf::code::LocalVariableList::Item;

struct Ropper{
    pub method: ConcreteMethod,
    pub blocks: ByteBlockList,
    pub maxLocals: i32,
    pub maxLabel: i32,
    pub machine: RopperMachine,
    pub sim: Simulator,
    pub startFrames: Vec<Frame>,
    pub result: ArrayList<BasicBlock>,
    pub resultSubroutines: ArrayList<IntList>,
    pub catchInfos: Vec<CatchInfo>,
    pub subroutines: Vec<Subroutine>,
    pub exceptionSetupLabelAllocator: ExceptionSetupLabelAllocator,
    pub synchNeedsExceptionHandler: boolean,
    pub hasSubroutines: boolean,
}
impl Ropper{
    pub const PARAM_ASSIGNMENT: i32 = -1;
    pub const RETURN: i32 = -2;
    pub const SYNCH_RETURN: i32 = -3;
    pub const SYNCH_SETUP_1: i32 = -4;
    pub const SYNCH_SETUP_2: i32 = -5;
    pub const SYNCH_CATCH_1: i32 = -6;
    pub const SYNCH_CATCH_2: i32 = -7;
    pub const SPECIAL_LABEL_COUNT: i32 = 7;
    pub fn new(&self, method: &ConcreteMethod, advice: &TranslationAdvice, methods: &MethodList, dexOptions: &DexOptions)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        if advice==None        {
            throw NullPointerException::new("advice == null");
        }        
        self->method=method;
        self->blocks=BasicBlocker::identifyBlocks(&method);
        self->maxLabel=self.blocks.getMaxLabel();
        self->maxLocals=method.getMaxLocals();
        self->machine=RopperMachine::new(self, &method, &advice, &methods);
        self->sim=Simulator::new(&self.machine, &method, &dexOptions);
        self->startFrames=new Frame[maxLabel];
        self->subroutines=new Subroutine[maxLabel];
        self->result=ArrayList<BasicBlock>::new(self.blocks.size()*2+10);
        self->resultSubroutines=ArrayList<IntList>::new(self.blocks.size()*2+10);
        self->catchInfos=new CatchInfo[maxLabel];
        self->synchNeedsExceptionHandler=false;
        self.startFrames[0]=Frame::new(self.maxLocals, method.getMaxStack());
        self.exceptionSetupLabelAllocator=ExceptionSetupLabelAllocator::new(, self);
    }
    pub fn convert(method: &ConcreteMethod, advice: &TranslationAdvice, methods: &MethodList, dexOptions: &DexOptions) -> RopMethod    {
        try        {
            let r: Ropper = Ropper::new(&method, &advice, &methods, &dexOptions);
            r.doit();
            return r.getRopMethod();
        }        catch(        let ex: SimException)        {
            ex.addContext("...while working on method "+method.getNat().toHuman());
            throw ex;
        }
    }
    pub fn getFirstTempStackReg(&self) -> i32    {
        let regCount: i32 = getNormalRegCount();
        return if isSynchronized() { regCount+1 } else { regCount };
            }
            pub fn getSpecialLabel(&self, label: i32) -> i32            {
                return self.maxLabel+self.method.getCatches().size()+~label;
            }
            pub fn getMinimumUnreservedLabel(&self) -> i32            {
                return self.maxLabel+self.method.getCatches().size()+Ropper::SPECIAL_LABEL_COUNT;
            }
            pub fn getAvailableLabel(&self) -> i32            {
                let candidate: i32 = getMinimumUnreservedLabel();
                for bb in self.result                {
                    let label: i32 = bb.getLabel();
                    if label>=candidate                    {
                        candidate=label+1;
                    }                    
                }
                return candidate;
            }
            pub fn isSynchronized(&self) -> boolean            {
                let accessFlags: i32 = self.method.getAccessFlags();
                return (accessFlags&AccessFlags::ACC_SYNCHRONIZED)!=0;
            }
            pub fn isStatic(&self) -> boolean            {
                let accessFlags: i32 = self.method.getAccessFlags();
                return (accessFlags&AccessFlags::ACC_STATIC)!=0;
            }
            pub fn getNormalRegCount(&self) -> i32            {
                return self.maxLocals+self.method.getMaxStack();
            }
            pub fn getSynchReg(&self) -> RegisterSpec            {
                let reg: i32 = getNormalRegCount();
                return RegisterSpec::make_int_TypeBearer(if (reg<1) { 1 } else { reg }, &Type::OBJECT);
                    }
                    pub fn labelToResultIndex(&self, label: i32) -> i32                    {
                        let sz: i32 = self.result.size();
                        for(                        let i: i32 = 0;;i<szi += 1)                        {
                            let one: BasicBlock = self.result.get(i);
                            if one.getLabel()==label                            {
                                return i;
                            }                            
                        }
                        return -1;
                    }
                    pub fn labelToBlock(&self, label: i32) -> BasicBlock                    {
                        let idx: i32 = labelToResultIndex(label);
                        if idx<0                        {
                            throw IllegalArgumentException::new("no such label "+Hex::u2(label));
                        }                        
                        return self.result.get(idx);
                    }
                    pub fn addBlock(&self, block: &BasicBlock, subroutines: &IntList)                    {
                        if block==None                        {
                            throw NullPointerException::new("block == null");
                        }                        
                        self.result.add_BasicBlock(&block);
                        subroutines.throwIfMutable();
                        self.resultSubroutines.add_IntList(&subroutines);
                    }
                    pub fn addOrReplaceBlock(&self, block: &BasicBlock, subroutines: &IntList) -> boolean                    {
                        if block==None                        {
                            throw NullPointerException::new("block == null");
                        }                        
                        let idx: i32 = labelToResultIndex(block.getLabel());
                        let ret: boolean;
                        if idx<0                        {
                            ret=false;
                        }                        else                         {
                            removeBlockAndSpecialSuccessors(idx);
                            ret=true;
                        }
                        self.result.add_BasicBlock(&block);
                        subroutines.throwIfMutable();
                        self.resultSubroutines.add_IntList(&subroutines);
                        return ret;
                    }
                    pub fn addOrReplaceBlockNoDelete(&self, block: &BasicBlock, subroutines: &IntList) -> boolean                    {
                        if block==None                        {
                            throw NullPointerException::new("block == null");
                        }                        
                        let idx: i32 = labelToResultIndex(block.getLabel());
                        let ret: boolean;
                        if idx<0                        {
                            ret=false;
                        }                        else                         {
                            self.result.remove_int(idx);
                            self.resultSubroutines.remove_int(idx);
                            ret=true;
                        }
                        self.result.add_BasicBlock(&block);
                        subroutines.throwIfMutable();
                        self.resultSubroutines.add_IntList(&subroutines);
                        return ret;
                    }
                    pub fn removeBlockAndSpecialSuccessors(&self, idx: i32)                    {
                        let minLabel: i32 = getMinimumUnreservedLabel();
                        let block: BasicBlock = self.result.get(idx);
                        let successors: IntList = block.getSuccessors();
                        let sz: i32 = successors.size();
                        self.result.remove_int(idx);
                        self.resultSubroutines.remove_int(idx);
                        for(                        let i: i32 = 0;;i<szi += 1)                        {
                            let label: i32 = successors.get(i);
                            if label>=minLabel                            {
                                idx=labelToResultIndex(label);
                                if idx<0                                {
                                    throw RuntimeException::new("Invalid label "+Hex::u2(label));
                                }                                
                                removeBlockAndSpecialSuccessors(idx);
                            }                            
                        }
                    }
                    pub fn getRopMethod(&self) -> RopMethod                    {
                        let sz: i32 = self.result.size();
                        let bbl: BasicBlockList = BasicBlockList::new(sz);
                        for(                        let i: i32 = 0;;i<szi += 1)                        {
                            bbl.set_int_BasicBlock(i, self.result.get(i));
                        }
                        bbl.setImmutable();
                        return RopMethod::new(&bbl, getSpecialLabel(Ropper::PARAM_ASSIGNMENT));
                    }
                    pub fn doit(&self)                    {
                        let workSet: Vec<i32> = Bits::makeBitSet(self.maxLabel);
                        Bits::set_int[]_int(&workSet, 0);
                        addSetupBlocks();
                        setFirstFrame();
                        for(;)                        {
                            let offset: i32 = Bits::findFirst_int[]_int(&workSet, 0);
                            if offset<0                            {
                                break;
                            }                            
                            Bits::clear(&workSet, offset);
                            let block: ByteBlock = self.blocks.labelToBlock(offset);
                            let frame: Frame = self.startFrames[offset];
                            try                            {
                                processBlock(&block, &frame, &workSet);
                            }                            catch(                            let ex: SimException)                            {
                                ex.addContext("...while working on block "+Hex::u2(offset));
                                throw ex;
                            }
                        }
                        addReturnBlock();
                        addSynchExceptionHandlerBlock();
                        addExceptionSetupBlocks();
                        if self.hasSubroutines                        {
                            inlineSubroutines();
                        }                        
                    }
                    pub fn setFirstFrame(&self)                    {
                        let desc: Prototype = self.method.getEffectiveDescriptor();
                        self.startFrames[0].initializeWithParameters(desc.getParameterTypes());
                        self.startFrames[0].setImmutable();
                    }
                    pub fn processBlock(&self, block: &ByteBlock, frame: &Frame, workSet: &Vec<i32>)                    {
                        let catches: ByteCatchList = block.getCatches();
                        self.machine.startBlock(catches.toRopCatchList());
                        frame=frame.copy();
                        self.sim.simulate_ByteBlock_Frame(&block, &frame);
                        frame.setImmutable();
                        let extraBlockCount: i32 = self.machine.getExtraBlockCount();
                        let insns: ArrayList<Insn> = self.machine.getInsns();
                        let insnSz: i32 = insns.size();
                        let catchSz: i32 = catches.size();
                        let successors: IntList = block.getSuccessors();
                        let startSuccessorIndex: i32;
                        let calledSubroutine: Subroutine = None;
                        if self.machine.hasJsr()                        {
                            startSuccessorIndex=1;
                            let subroutineLabel: i32 = successors.get(1);
                            if self.subroutines[subroutineLabel]==None                            {
                                self.subroutines[subroutineLabel]=Subroutine::new(subroutineLabel, self);
                            }                            
                            self.subroutines[subroutineLabel].addCallerBlock(block.getLabel());
                            calledSubroutine=self.subroutines[subroutineLabel];
                        }                        else                         if self.machine.hasRet()                        {
                            let ra: ReturnAddress = self.machine.getReturnAddress();
                            let subroutineLabel: i32 = ra.getSubroutineAddress();
                            if self.subroutines[subroutineLabel]==None                            {
                                self.subroutines[subroutineLabel]=Subroutine::new(subroutineLabel, block.getLabel(), self);
                            }                            else                             {
                                self.subroutines[subroutineLabel].addRetBlock(block.getLabel());
                            }
                            successors=self.subroutines[subroutineLabel].getSuccessors();
                            self.subroutines[subroutineLabel].mergeToSuccessors(&frame, &workSet);
                            startSuccessorIndex=successors.size();
                        }                        else                         if self.machine.wereCatchesUsed()                        {
                            startSuccessorIndex=catchSz;
                        }                        else                         {
                            startSuccessorIndex=0;
                        }
                        let succSz: i32 = successors.size();
                        for(                        let i: i32 = startSuccessorIndex;;i<succSzi += 1)                        {
                            let succ: i32 = successors.get(i);
                            try                            {
                                mergeAndWorkAsNecessary(succ, block.getLabel(), &calledSubroutine, &frame, &workSet);
                            }                            catch(                            let ex: SimException)                            {
                                ex.addContext("...while merging to block "+Hex::u2(succ));
                                throw ex;
                            }
                        }
                        if (succSz==0)&&self.machine.returns()                        {
                            successors=IntList::makeImmutable_int(getSpecialLabel(Ropper::RETURN));
                            succSz=1;
                        }                        
                        let primarySucc: i32;
                        if succSz==0                        {
                            primarySucc=-1;
                        }                        else                         {
                            primarySucc=self.machine.getPrimarySuccessorIndex();
                            if primarySucc>=0                            {
                                primarySucc=successors.get(primarySucc);
                            }                            
                        }
                        let synch: boolean = isSynchronized()&&self.machine.canThrow();
                        if synch||(catchSz!=0)                        {
                            let catchesAny: boolean = false;
                            let newSucc: IntList = IntList::new(succSz);
                            for(                            let i: i32 = 0;;i<catchSzi += 1)                            {
                                let one: Item = catches.get(i);
                                let exceptionClass: CstType = one.getExceptionClass();
                                let targ: i32 = one.getHandlerPc();
                                catchesAny|=(exceptionClass==CstType::OBJECT);
                                let f: Frame = frame.makeExceptionHandlerStartFrame(&exceptionClass);
                                try                                {
                                    mergeAndWorkAsNecessary(targ, block.getLabel(), None, &f, &workSet);
                                }                                catch(                                let ex: SimException)                                {
                                    ex.addContext("...while merging exception to block "+Hex::u2(targ));
                                    throw ex;
                                }
                                let handlers: CatchInfo = self.catchInfos[targ];
                                if handlers==None                                {
                                    handlers=CatchInfo::new(, self);
                                    self.catchInfos[targ]=handlers;
                                }                                
                                let handler: ExceptionHandlerSetup = handlers.getSetup(exceptionClass.getClassType());
                                newSucc.add(handler.getLabel());
                            }
                            if synch&&!catchesAny                            {
                                newSucc.add(getSpecialLabel(Ropper::SYNCH_CATCH_1));
                                self.synchNeedsExceptionHandler=true;
                                for(                                let i: i32 = insnSz-extraBlockCount-1;;i<insnSzi += 1)                                {
                                    let insn: Insn = insns.get(i);
                                    if insn.canThrow()                                    {
                                        insn=insn.withAddedCatch(&Type::OBJECT);
                                        insns.set(i, &insn);
                                    }                                    
                                }
                            }                            
                            if primarySucc>=0                            {
                                newSucc.add(primarySucc);
                            }                            
                            newSucc.setImmutable();
                            successors=newSucc;
                        }                        
                        let primarySuccListIndex: i32 = successors.indexOf(primarySucc);
                        for(;extraBlockCount>0extraBlockCount -= 1)                        {
                            let extraInsn: Insn = insns.get(--insnSz);
                            let needsGoto: boolean = extraInsn.getOpcode().getBranchingness()==Rop::BRANCH_NONE;
                            let il: InsnList = InsnList::new(if needsGoto { 2 } else { 1 });
                                    let extraBlockSuccessors: IntList = successors;
                                    il.set_int_Insn(0, &extraInsn);
                                    if needsGoto                                    {
                                        il.set_int_Insn(1, PlainInsn::new(&Rops::GOTO, extraInsn.getPosition(), None, &RegisterSpecList::EMPTY));
                                        extraBlockSuccessors=IntList::makeImmutable_int(primarySucc);
                                    }                                    
                                    il.setImmutable();
                                    let label: i32 = getAvailableLabel();
                                    let bb: BasicBlock = BasicBlock::new(label, &il, &extraBlockSuccessors, primarySucc);
                                    addBlock(&bb, frame.getSubroutines());
                                    successors=successors.mutableCopy();
                                    successors.set(primarySuccListIndex, label);
                                    successors.setImmutable();
                                    primarySucc=label;
                                }
                                let lastInsn: Insn = if (insnSz==0) { None } else { insns.get(insnSz-1) };
                                        if (lastInsn==None)||(lastInsn.getOpcode().getBranchingness()==Rop::BRANCH_NONE)                                        {
                                            let pos: SourcePosition = if (lastInsn==None) { SourcePosition::NO_INFO } else { lastInsn.getPosition() };
                                                    insns.add_Insn(PlainInsn::new(&Rops::GOTO, &pos, None, &RegisterSpecList::EMPTY));
                                                    insnSz += 1;
                                                }                                                
                                                let il: InsnList = InsnList::new(insnSz);
                                                for(                                                let i: i32 = 0;;i<insnSzi += 1)                                                {
                                                    il.set_int_Insn(i, insns.get(i));
                                                }
                                                il.setImmutable();
                                                let bb: BasicBlock = BasicBlock::new(block.getLabel(), &il, &successors, primarySucc);
                                                addOrReplaceBlock(&bb, frame.getSubroutines());
                                            }
                                            pub fn mergeAndWorkAsNecessary(&self, label: i32, pred: i32, calledSubroutine: &Subroutine, frame: &Frame, workSet: &Vec<i32>)                                            {
                                                let existing: Frame = self.startFrames[label];
                                                let merged: Frame;
                                                if existing!=None                                                {
                                                    if calledSubroutine!=None                                                    {
                                                        merged=existing.mergeWithSubroutineCaller(&frame, calledSubroutine.getStartBlock(), pred);
                                                    }                                                    else                                                     {
                                                        merged=existing.mergeWith(&frame);
                                                    }
                                                    if merged!=existing                                                    {
                                                        self.startFrames[label]=merged;
                                                        Bits::set_int[]_int(&workSet, label);
                                                    }                                                    
                                                }                                                else                                                 {
                                                    if calledSubroutine!=None                                                    {
                                                        self.startFrames[label]=frame.makeNewSubroutineStartFrame(label, pred);
                                                    }                                                    else                                                     {
                                                        self.startFrames[label]=frame;
                                                    }
                                                    Bits::set_int[]_int(&workSet, label);
                                                }
                                            }
                                            pub fn addSetupBlocks(&self)                                            {
                                                let localVariables: LocalVariableList = self.method.getLocalVariables();
                                                let pos: SourcePosition = self.method.makeSourcePosistion(0);
                                                let desc: Prototype = self.method.getEffectiveDescriptor();
                                                let params: StdTypeList = desc.getParameterTypes();
                                                let sz: i32 = params.size();
                                                let insns: InsnList = InsnList::new(sz+1);
                                                let at: i32 = 0;
                                                for(                                                let i: i32 = 0;;i<szi += 1)                                                {
                                                    let one: Type = params.get(i);
                                                    let local: Item = localVariables.pcAndIndexToLocal(0, at);
                                                    let result: RegisterSpec = if (local==None) { RegisterSpec::make_int_TypeBearer(at, &one) } else { RegisterSpec::makeLocalOptional(at, &one, local.getLocalItem()) };
                                                            let insn: Insn = PlainCstInsn::new(Rops::opMoveParam(&one), &pos, &result, &RegisterSpecList::EMPTY, CstInteger::make(at));
                                                            insns.set_int_Insn(i, &insn);
                                                            at+=one.getCategory();
                                                        }
                                                        insns.set_int_Insn(sz, PlainInsn::new(&Rops::GOTO, &pos, None, &RegisterSpecList::EMPTY));
                                                        insns.setImmutable();
                                                        let synch: boolean = isSynchronized();
                                                        let label: i32 = if synch { getSpecialLabel(Ropper::SYNCH_SETUP_1) } else { 0 };
                                                                let bb: BasicBlock = BasicBlock::new(getSpecialLabel(Ropper::PARAM_ASSIGNMENT), &insns, IntList::makeImmutable_int(label), label);
                                                                addBlock(&bb, &IntList::EMPTY);
                                                                if synch                                                                {
                                                                    let synchReg: RegisterSpec = getSynchReg();
                                                                    let insn: Insn;
                                                                    if isStatic()                                                                    {
                                                                        insn=ThrowingCstInsn::new(&Rops::CONST_OBJECT, &pos, &RegisterSpecList::EMPTY, &StdTypeList::EMPTY, self.method.getDefiningClass());
                                                                        insns=InsnList::new(1);
                                                                        insns.set_int_Insn(0, &insn);
                                                                    }                                                                    else                                                                     {
                                                                        insns=InsnList::new(2);
                                                                        insn=PlainCstInsn::new(&Rops::MOVE_PARAM_OBJECT, &pos, &synchReg, &RegisterSpecList::EMPTY, &CstInteger::VALUE_0);
                                                                        insns.set_int_Insn(0, &insn);
                                                                        insns.set_int_Insn(1, PlainInsn::new(&Rops::GOTO, &pos, None, &RegisterSpecList::EMPTY));
                                                                    }
                                                                    let label2: i32 = getSpecialLabel(Ropper::SYNCH_SETUP_2);
                                                                    insns.setImmutable();
                                                                    bb=BasicBlock::new(label, &insns, IntList::makeImmutable_int(label2), label2);
                                                                    addBlock(&bb, &IntList::EMPTY);
                                                                    insns=InsnList::new(if isStatic() { 2 } else { 1 });
                                                                            if isStatic()                                                                            {
                                                                                insns.set_int_Insn(0, PlainInsn::new(Rops::opMoveResultPseudo(&synchReg), &pos, &synchReg, &RegisterSpecList::EMPTY));
                                                                            }                                                                            
                                                                            insn=ThrowingInsn::new(&Rops::MONITOR_ENTER, &pos, RegisterSpecList::make_RegisterSpec(&synchReg), &StdTypeList::EMPTY);
                                                                            insns.set_int_Insn(if isStatic() { 1 } else { 0 }, &insn);
                                                                                    insns.setImmutable();
                                                                                    bb=BasicBlock::new(label2, &insns, IntList::makeImmutable_int(0), 0);
                                                                                    addBlock(&bb, &IntList::EMPTY);
                                                                                }                                                                                
                                                                            }
                                                                            pub fn addReturnBlock(&self)                                                                            {
                                                                                let returnOp: Rop = self.machine.getReturnOp();
                                                                                if returnOp==None                                                                                {
                                                                                    return;
                                                                                }                                                                                
                                                                                let returnPos: SourcePosition = self.machine.getReturnPosition();
                                                                                let label: i32 = getSpecialLabel(Ropper::RETURN);
                                                                                if isSynchronized()                                                                                {
                                                                                    let insns: InsnList = InsnList::new(1);
                                                                                    let insn: Insn = ThrowingInsn::new(&Rops::MONITOR_EXIT, &returnPos, RegisterSpecList::make_RegisterSpec(getSynchReg()), &StdTypeList::EMPTY);
                                                                                    insns.set_int_Insn(0, &insn);
                                                                                    insns.setImmutable();
                                                                                    let nextLabel: i32 = getSpecialLabel(Ropper::SYNCH_RETURN);
                                                                                    let bb: BasicBlock = BasicBlock::new(label, &insns, IntList::makeImmutable_int(nextLabel), nextLabel);
                                                                                    addBlock(&bb, &IntList::EMPTY);
                                                                                    label=nextLabel;
                                                                                }                                                                                
                                                                                let insns: InsnList = InsnList::new(1);
                                                                                let sourceTypes: TypeList = returnOp.getSources();
                                                                                let sources: RegisterSpecList;
                                                                                if sourceTypes.size()==0                                                                                {
                                                                                    sources=RegisterSpecList::EMPTY;
                                                                                }                                                                                else                                                                                 {
                                                                                    let source: RegisterSpec = RegisterSpec::make_int_TypeBearer(0, sourceTypes.getType(0));
                                                                                    sources=RegisterSpecList::make_RegisterSpec(&source);
                                                                                }
                                                                                let insn: Insn = PlainInsn::new(&returnOp, &returnPos, None, &sources);
                                                                                insns.set_int_Insn(0, &insn);
                                                                                insns.setImmutable();
                                                                                let bb: BasicBlock = BasicBlock::new(label, &insns, &IntList::EMPTY, -1);
                                                                                addBlock(&bb, &IntList::EMPTY);
                                                                            }
                                                                            pub fn addSynchExceptionHandlerBlock(&self)                                                                            {
                                                                                if !self.synchNeedsExceptionHandler                                                                                {
                                                                                    return;
                                                                                }                                                                                
                                                                                let pos: SourcePosition = self.method.makeSourcePosistion(0);
                                                                                let exReg: RegisterSpec = RegisterSpec::make_int_TypeBearer(0, &Type::THROWABLE);
                                                                                let bb: BasicBlock;
                                                                                let insn: Insn;
                                                                                let insns: InsnList = InsnList::new(2);
                                                                                insn=PlainInsn::new(Rops::opMoveException(&Type::THROWABLE), &pos, &exReg, &RegisterSpecList::EMPTY);
                                                                                insns.set_int_Insn(0, &insn);
                                                                                insn=ThrowingInsn::new(&Rops::MONITOR_EXIT, &pos, RegisterSpecList::make_RegisterSpec(getSynchReg()), &StdTypeList::EMPTY);
                                                                                insns.set_int_Insn(1, &insn);
                                                                                insns.setImmutable();
                                                                                let label2: i32 = getSpecialLabel(Ropper::SYNCH_CATCH_2);
                                                                                bb=BasicBlock::new(getSpecialLabel(Ropper::SYNCH_CATCH_1), &insns, IntList::makeImmutable_int(label2), label2);
                                                                                addBlock(&bb, &IntList::EMPTY);
                                                                                insns=InsnList::new(1);
                                                                                insn=ThrowingInsn::new(&Rops::THROW, &pos, RegisterSpecList::make_RegisterSpec(&exReg), &StdTypeList::EMPTY);
                                                                                insns.set_int_Insn(0, &insn);
                                                                                insns.setImmutable();
                                                                                bb=BasicBlock::new(label2, &insns, &IntList::EMPTY, -1);
                                                                                addBlock(&bb, &IntList::EMPTY);
                                                                            }
                                                                            pub fn addExceptionSetupBlocks(&self)                                                                            {
                                                                                let len: i32 = self.catchInfos.len();
                                                                                for(                                                                                let i: i32 = 0;;i<leni += 1)                                                                                {
                                                                                    let catches: CatchInfo = self.catchInfos[i];
                                                                                    if catches!=None                                                                                    {
                                                                                        for one in catches.getSetups()                                                                                        {
                                                                                            let proto: Insn = labelToBlock(i).getFirstInsn();
                                                                                            let pos: SourcePosition = proto.getPosition();
                                                                                            let il: InsnList = InsnList::new(2);
                                                                                            let insn: Insn = PlainInsn::new(Rops::opMoveException(one.getCaughtType()), &pos, RegisterSpec::make_int_TypeBearer(self.maxLocals, one.getCaughtType()), &RegisterSpecList::EMPTY);
                                                                                            il.set_int_Insn(0, &insn);
                                                                                            insn=PlainInsn::new(&Rops::GOTO, &pos, None, &RegisterSpecList::EMPTY);
                                                                                            il.set_int_Insn(1, &insn);
                                                                                            il.setImmutable();
                                                                                            let bb: BasicBlock = BasicBlock::new(one.getLabel(), &il, IntList::makeImmutable_int(i), i);
                                                                                            addBlock(&bb, self.startFrames[i].getSubroutines());
                                                                                        }
                                                                                    }                                                                                    
                                                                                }
                                                                            }
                                                                            pub fn isSubroutineCaller(&self, bb: &BasicBlock) -> boolean                                                                            {
                                                                                let successors: IntList = bb.getSuccessors();
                                                                                if successors.size()<2{                                                                                    return false;                                                                                }                                                                                
                                                                                let subLabel: i32 = successors.get(1);
                                                                                return (subLabel<self.subroutines.len())&&(self.subroutines[subLabel]!=None);
                                                                            }
                                                                            pub fn inlineSubroutines(&self)                                                                            {
                                                                                let reachableSubroutineCallerLabels: IntList = IntList::new(4);
                                                                                forEachNonSubBlockDepthFirst(0, /*new BasicBlock.Visitor(){
  @Override public void visitBlock(  BasicBlock b){
    if (isSubroutineCaller(b)) {
      reachableSubroutineCallerLabels.add(b.getLabel());
    }
  }
}
*/);
                                                                                let largestAllocedLabel: i32 = getAvailableLabel();
                                                                                let labelToSubroutines: ArrayList<IntList> = ArrayList<IntList>::new(largestAllocedLabel);
                                                                                for(                                                                                let i: i32 = 0;;i<largestAllocedLabeli += 1)                                                                                {
                                                                                    labelToSubroutines.add_IntList(None);
                                                                                }
                                                                                for(                                                                                let i: i32 = 0;;i<self.result.size()i += 1)                                                                                {
                                                                                    let b: BasicBlock = self.result.get(i);
                                                                                    if b==None                                                                                    {
                                                                                        continue;
                                                                                    }                                                                                    
                                                                                    let subroutineList: IntList = self.resultSubroutines.get(i);
                                                                                    labelToSubroutines.set(b.getLabel(), &subroutineList);
                                                                                }
                                                                                let sz: i32 = reachableSubroutineCallerLabels.size();
                                                                                for(                                                                                let i: i32 = 0;;i<szi += 1)                                                                                {
                                                                                    let label: i32 = reachableSubroutineCallerLabels.get(i);
                                                                                    SubroutineInliner::new(LabelAllocator::new(getAvailableLabel()), &labelToSubroutines, self).inlineSubroutineCalledFrom(labelToBlock(label));
                                                                                }
                                                                                deleteUnreachableBlocks();
                                                                            }
                                                                            pub fn deleteUnreachableBlocks(&self)                                                                            {
                                                                                let reachableLabels: IntList = IntList::new(self.result.size());
                                                                                self.resultSubroutines.clear();
                                                                                forEachNonSubBlockDepthFirst(getSpecialLabel(Ropper::PARAM_ASSIGNMENT), /*new BasicBlock.Visitor(){
  @Override public void visitBlock(  BasicBlock b){
    reachableLabels.add(b.getLabel());
  }
}
*/);
                                                                                reachableLabels.sort();
                                                                                for(                                                                                let i: i32 = self.result.size()-1;;i>=0i -= 1)                                                                                {
                                                                                    if reachableLabels.indexOf(self.result.get(i).getLabel())<0                                                                                    {
                                                                                        self.result.remove_int(i);
                                                                                    }                                                                                    
                                                                                }
                                                                            }
                                                                            pub fn subroutineFromRetBlock(&self, label: i32) -> Subroutine                                                                            {
                                                                                for(                                                                                let i: i32 = self.subroutines.len()-1;;i>=0i -= 1)                                                                                {
                                                                                    if self.subroutines[i]!=None                                                                                    {
                                                                                        let subroutine: Subroutine = self.subroutines[i];
                                                                                        if .get_int(label)                                                                                        {
                                                                                            return subroutine;
                                                                                        }                                                                                        
                                                                                    }                                                                                    
                                                                                }
                                                                                return None;
                                                                            }
                                                                            pub fn filterMoveReturnAddressInsns(&self, insns: &InsnList) -> InsnList                                                                            {
                                                                                let sz: i32;
                                                                                let newSz: i32 = 0;
                                                                                sz=insns.size();
                                                                                for(                                                                                let i: i32 = 0;;i<szi += 1)                                                                                {
                                                                                    if insns.get(i).getOpcode()!=Rops::MOVE_RETURN_ADDRESS                                                                                    {
                                                                                        newSz += 1;
                                                                                    }                                                                                    
                                                                                }
                                                                                if newSz==sz                                                                                {
                                                                                    return insns;
                                                                                }                                                                                
                                                                                let newInsns: InsnList = InsnList::new(newSz);
                                                                                let newIndex: i32 = 0;
                                                                                for(                                                                                let i: i32 = 0;;i<szi += 1)                                                                                {
                                                                                    let insn: Insn = insns.get(i);
                                                                                    if insn.getOpcode()!=Rops::MOVE_RETURN_ADDRESS                                                                                    {
                                                                                        newInsns.set_int_Insn(newIndex += 1, &insn);
                                                                                    }                                                                                    
                                                                                }
                                                                                newInsns.setImmutable();
                                                                                return newInsns;
                                                                            }
                                                                            pub fn forEachNonSubBlockDepthFirst(&self, firstLabel: i32, v: &BasicBlock.Visitor)                                                                            {
                                                                                forEachNonSubBlockDepthFirst0(labelToBlock(firstLabel), &v, BitSet::new(self.maxLabel));
                                                                            }
                                                                            pub fn forEachNonSubBlockDepthFirst0(&self, next: &BasicBlock, v: &BasicBlock.Visitor, visited: &BitSet)                                                                            {
                                                                                v.visitBlock(&next);
                                                                                visited.set_int(next.getLabel());
                                                                                let successors: IntList = next.getSuccessors();
                                                                                let sz: i32 = successors.size();
                                                                                for(                                                                                let i: i32 = 0;;i<szi += 1)                                                                                {
                                                                                    let succ: i32 = successors.get(i);
                                                                                    if visited.get_int(succ)                                                                                    {
                                                                                        continue;
                                                                                    }                                                                                    
                                                                                    if isSubroutineCaller(&next)&&i>0                                                                                    {
                                                                                        continue;
                                                                                    }                                                                                    
                                                                                    let idx: i32 = labelToResultIndex(succ);
                                                                                    if idx>=0                                                                                    {
                                                                                        forEachNonSubBlockDepthFirst0(self.result.get(idx), &v, &visited);
                                                                                    }                                                                                    
                                                                                }
                                                                            }
}
