use crate::helper;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::dex::code::ZeroSizeInsn;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::MultiCstInsn;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::dex::code::InsnFormat;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::dex::code::LocalSnapshot;
use crate::com::android::dx::dex::code::TargetInsn;
use crate::com::android::dx::dex::code::LocalStart;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstMemberRef;
use crate::com::android::dex::DexException;
use crate::com::android::dx::ssa::BasicRegisterMapper;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::dex::code::Dops;
use crate::com::android::dx::dex::code::DalvCode::AssignIndicesCallback;

struct OutputFinisher{
    pub dexOptions: DexOptions,
    pub unreservedRegCount: i32,
    pub insns: ArrayList<DalvInsn>,
    pub hasAnyPositionInfo: boolean,
    pub hasAnyLocalInfo: boolean,
    pub reservedCount: i32,
    pub reservedParameterCount: i32,
    pub paramSize: i32,
}
impl OutputFinisher{
    pub fn new(&self, dexOptions: &DexOptions, initialCapacity: i32, regCount: i32, paramSize: i32)    {
        self->dexOptions=dexOptions;
        self->unreservedRegCount=regCount;
        self->insns=ArrayList<DalvInsn>::new(initialCapacity);
        self->reservedCount=-1;
        self->hasAnyPositionInfo=false;
        self->hasAnyLocalInfo=false;
        self->paramSize=paramSize;
    }
    pub fn hasAnyPositionInfo(&self) -> boolean    {
        return self.hasAnyPositionInfo;
    }
    pub fn hasAnyLocalInfo(&self) -> boolean    {
        return self.hasAnyLocalInfo;
    }
    pub fn hasLocalInfo(insn: &DalvInsn) -> boolean    {
        if //insn instanceof LocalSnapshot        {
            let specs: RegisterSpecSet = ((LocalSnapshot*)insn).getLocals();
            let size: i32 = specs.size();
            for(            let i: i32 = 0;;i<sizei += 1)            {
                if OutputFinisher::hasLocalInfo_RegisterSpec(specs.get_int(i))                {
                    return true;
                }                
            }
        }        else         if //insn instanceof LocalStart        {
            let spec: RegisterSpec = ((LocalStart*)insn).getLocal();
            if OutputFinisher::hasLocalInfo_RegisterSpec(&spec)            {
                return true;
            }            
        }        
        return false;
    }
    pub fn hasLocalInfo(spec: &RegisterSpec) -> boolean    {
        return (spec!=None)&&(spec.getLocalItem().getName()!=None);
    }
    pub fn getAllConstants(&self) -> HashSet<Constant>    {
        let result: HashSet<Constant> = HashSet<Constant>::new(20);
        for insn in self.insns        {
            OutputFinisher::addConstants_HashSet<Constant>_DalvInsn(&result, &insn);
        }
        return result;
    }
    pub fn addConstants(result: &HashSet<Constant>, insn: &DalvInsn)    {
        if //insn instanceof CstInsn        {
            let cst: Constant = ((CstInsn*)insn).getConstant();
            result.add(&cst);
        }        else         if //insn instanceof MultiCstInsn        {
            let m: MultiCstInsn = (MultiCstInsn*)insn;
            for(            let i: i32 = 0;;i<m.getNumberOfConstants()i += 1)            {
                result.add(m.getConstant(i));
            }
        }        else         if //insn instanceof LocalSnapshot        {
            let specs: RegisterSpecSet = ((LocalSnapshot*)insn).getLocals();
            let size: i32 = specs.size();
            for(            let i: i32 = 0;;i<sizei += 1)            {
                OutputFinisher::addConstants_HashSet<Constant>_RegisterSpec(&result, specs.get_int(i));
            }
        }        else         if //insn instanceof LocalStart        {
            let spec: RegisterSpec = ((LocalStart*)insn).getLocal();
            OutputFinisher::addConstants_HashSet<Constant>_RegisterSpec(&result, &spec);
        }        
    }
    pub fn addConstants(result: &HashSet<Constant>, spec: &RegisterSpec)    {
        if spec==None        {
            return;
        }        
        let local: LocalItem = spec.getLocalItem();
        let name: CstString = local.getName();
        let signature: CstString = local.getSignature();
        let type: Type = spec.getType();
        if type_renamed!=Type::KNOWN_NULL        {
            result.add(CstType::intern(&type_renamed));
        }        else         {
            result.add(CstType::intern(&Type::OBJECT));
        }
        if name!=None        {
            result.add(&name);
        }        
        if signature!=None        {
            result.add(&signature);
        }        
    }
    pub fn add(&self, insn: &DalvInsn)    {
        self.insns.add_DalvInsn(&insn);
        updateInfo(&insn);
    }
    pub fn insert(&self, at: i32, insn: &DalvInsn)    {
        self.insns.add_int_DalvInsn(at, &insn);
        updateInfo(&insn);
    }
    pub fn get(&self, at: i32) -> DalvInsn    {
        return self.insns.get(at);
    }
    pub fn size(&self) -> i32    {
        return self.insns.size();
    }
    pub fn updateInfo(&self, insn: &DalvInsn)    {
        if !self.hasAnyPositionInfo        {
            let pos: SourcePosition = insn.getPosition();
            if pos.getLine()>=0            {
                self.hasAnyPositionInfo=true;
            }            
        }        
        if !self.hasAnyLocalInfo        {
            if OutputFinisher::hasLocalInfo_DalvInsn(&insn)            {
                self.hasAnyLocalInfo=true;
            }            
        }        
    }
    pub fn reverseBranch(&self, which: i32, newTarget: &CodeAddress)    {
        let size: i32 = self.insns.size();
        let index: i32 = size-which-1;
        let targetInsn: TargetInsn;
        try        {
            targetInsn=(TargetInsn*)self.insns.get(index);
        }        catch(        let ex: IndexOutOfBoundsException)        {
            throw IllegalArgumentException::new("too few instructions");
        }        catch(        let ex: ClassCastException)        {
            throw IllegalArgumentException::new("non-reversible instruction");
        }
        self.insns.set(index, targetInsn.withNewTargetAndReversed(&newTarget));
    }
    pub fn assignIndices(&self, callback: &DalvCode.AssignIndicesCallback)    {
        for insn in self.insns        {
            if //insn instanceof CstInsn            {
                OutputFinisher::assignIndices_CstInsn_AssignIndicesCallback((CstInsn*)insn, &callback);
            }            else             if //insn instanceof MultiCstInsn            {
                OutputFinisher::assignIndices_MultiCstInsn_AssignIndicesCallback((MultiCstInsn*)insn, &callback);
            }            
        }
    }
    pub fn assignIndices(insn: &CstInsn, callback: &DalvCode.AssignIndicesCallback)    {
        let cst: Constant = insn.getConstant();
        let index: i32 = callback.getIndex(&cst);
        if index>=0        {
            insn.setIndex(index);
        }        
        if //cst instanceof CstMemberRef        {
            let member: CstMemberRef = (CstMemberRef*)cst;
            let definer: CstType = member.getDefiningClass();
            index=callback.getIndex(&definer);
            if index>=0            {
                insn.setClassIndex(index);
            }            
        }        
    }
    pub fn assignIndices(insn: &MultiCstInsn, callback: &DalvCode.AssignIndicesCallback)    {
        for(        let i: i32 = 0;;i<insn.getNumberOfConstants()++i)        {
            let cst: Constant = insn.getConstant(i);
            let index: i32 = callback.getIndex(&cst);
            insn.setIndex(i, index);
            if //cst instanceof CstMemberRef            {
                let member: CstMemberRef = (CstMemberRef*)cst;
                let definer: CstType = member.getDefiningClass();
                index=callback.getIndex(&definer);
                insn.setClassIndex(index);
            }            
        }
    }
    pub fn finishProcessingAndGetList(&self) -> DalvInsnList    {
        if self.reservedCount>=0        {
            throw UnsupportedOperationException::new("already processed");
        }        
        let opcodes: Vec<Dop> = makeOpcodesArray();
        reserveRegisters(&opcodes);
        if         {
            align64bits(&opcodes);
        }        
        massageInstructions(&opcodes);
        assignAddressesAndFixBranches();
        return DalvInsnList::makeImmutable(&self.insns, self.reservedCount+self.unreservedRegCount+self.reservedParameterCount);
    }
    pub fn makeOpcodesArray(&self) -> Vec<Dop>    {
        let size: i32 = self.insns.size();
        let result: Vec<Dop> = new Dop[size];
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let insn: DalvInsn = self.insns.get(i);
            result[i]=insn.getOpcode();
        }
        return result;
    }
    pub fn reserveRegisters(&self, opcodes: &Vec<Dop>) -> boolean    {
        let reservedCountExpanded: boolean = false;
        let oldReservedCount: i32 = if (self.reservedCount<0) { 0 } else { self.reservedCount };
                for(;)                {
                    let newReservedCount: i32 = calculateReservedCount(&opcodes);
                    if oldReservedCount>=newReservedCount                    {
                        break;
                    }                    
                    reservedCountExpanded=true;
                    let reservedDifference: i32 = newReservedCount-oldReservedCount;
                    let size: i32 = self.insns.size();
                    for(                    let i: i32 = 0;;i<sizei += 1)                    {
                        let insn: DalvInsn = self.insns.get(i);
                        if !(//insn instanceof CodeAddress)                        {
                            self.insns.set(i, insn.withRegisterOffset(reservedDifference));
                        }                        
                    }
                    oldReservedCount=newReservedCount;
                }
                self.reservedCount=oldReservedCount;
                return reservedCountExpanded;
            }
            pub fn calculateReservedCount(&self, opcodes: &Vec<Dop>) -> i32            {
                let size: i32 = self.insns.size();
                let newReservedCount: i32 = self.reservedCount;
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    let insn: DalvInsn = self.insns.get(i);
                    let originalOpcode: Dop = opcodes[i];
                    let newOpcode: Dop = findOpcodeForInsn(&insn, &originalOpcode);
                    if newOpcode==None                    {
                        let expandedOp: Dop = findExpandedOpcodeForInsn(&insn);
                        let compatRegs: BitSet = expandedOp.getFormat().compatibleRegs(&insn);
                        let reserve: i32 = insn.getMinimumRegisterRequirement(&compatRegs);
                        if reserve>newReservedCount                        {
                            newReservedCount=reserve;
                        }                        
                    }                    else                     if originalOpcode==newOpcode                    {
                        continue;
                    }                    
                    opcodes[i]=newOpcode;
                }
                return newReservedCount;
            }
            pub fn findOpcodeForInsn(&self, insn: &DalvInsn, guess: &Dop) -> Dop            {
                while guess!=None                {
                    if guess.getFormat().isCompatible(&insn)                    {
                        if !||guess.getOpcode()!=Opcodes::CONST_STRING                        {
                            break;
                        }                        
                    }                    
                    guess=Dops::getNextOrNull(&guess, &self.dexOptions);
                }
                return guess;
            }
            pub fn findExpandedOpcodeForInsn(&self, insn: &DalvInsn) -> Dop            {
                let result: Dop = findOpcodeForInsn(insn.getLowRegVersion(), insn.getOpcode());
                if result==None                {
                    throw DexException::new("No expanded opcode for "+insn);
                }                
                return result;
            }
            pub fn massageInstructions(&self, opcodes: &Vec<Dop>)            {
                if self.reservedCount==0                {
                    let size: i32 = self.insns.size();
                    for(                    let i: i32 = 0;;i<sizei += 1)                    {
                        let insn: DalvInsn = self.insns.get(i);
                        let originalOpcode: Dop = insn.getOpcode();
                        let currentOpcode: Dop = opcodes[i];
                        if originalOpcode!=currentOpcode                        {
                            self.insns.set(i, insn.withOpcode(&currentOpcode));
                        }                        
                    }
                }                else                 {
                    self.insns=performExpansion(&opcodes);
                }
            }
            pub fn performExpansion(&self, opcodes: &Vec<Dop>) -> ArrayList<DalvInsn>            {
                let size: i32 = self.insns.size();
                let result: ArrayList<DalvInsn> = ArrayList<DalvInsn>::new(size*2);
                let closelyBoundAddresses: ArrayList<CodeAddress> = ArrayList<CodeAddress>::new();
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    let insn: DalvInsn = self.insns.get(i);
                    let originalOpcode: Dop = insn.getOpcode();
                    let currentOpcode: Dop = opcodes[i];
                    let prefix: DalvInsn;
                    let suffix: DalvInsn;
                    if currentOpcode!=None                    {
                        prefix=None;
                        suffix=None;
                    }                    else                     {
                        currentOpcode=findExpandedOpcodeForInsn(&insn);
                        let compatRegs: BitSet = currentOpcode.getFormat().compatibleRegs(&insn);
                        prefix=insn.expandedPrefix(&compatRegs);
                        suffix=insn.expandedSuffix(&compatRegs);
                        insn=insn.expandedVersion(&compatRegs);
                    }
                    if //insn instanceof CodeAddress                    {
                        if ((CodeAddress*)insn).getBindsClosely()                        {
                            closelyBoundAddresses.add_CodeAddress((CodeAddress*)insn);
                            continue;
                        }                        
                    }                    
                    if prefix!=None                    {
                        result.add_DalvInsn(&prefix);
                    }                    
                    if !(//insn instanceof ZeroSizeInsn)&&closelyBoundAddresses.size()>0                    {
                        for codeAddress in closelyBoundAddresses                        {
                            result.add_DalvInsn(&codeAddress);
                        }
                        closelyBoundAddresses.clear();
                    }                    
                    if currentOpcode!=originalOpcode                    {
                        insn=insn.withOpcode(&currentOpcode);
                    }                    
                    result.add_DalvInsn(&insn);
                    if suffix!=None                    {
                        result.add_DalvInsn(&suffix);
                    }                    
                }
                return result;
            }
            pub fn assignAddressesAndFixBranches(&self)            {
                for(;)                {
                    assignAddresses();
                    if !fixBranches()                    {
                        break;
                    }                    
                }
            }
            pub fn assignAddresses(&self)            {
                let address: i32 = 0;
                let size: i32 = self.insns.size();
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    let insn: DalvInsn = self.insns.get(i);
                    insn.setAddress(address);
                    address+=insn.codeSize();
                }
            }
            pub fn fixBranches(&self) -> boolean            {
                let size: i32 = self.insns.size();
                let anyFixed: boolean = false;
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    let insn: DalvInsn = self.insns.get(i);
                    if !(//insn instanceof TargetInsn)                    {
                        continue;
                    }                    
                    let opcode: Dop = insn.getOpcode();
                    let target: TargetInsn = (TargetInsn*)insn;
                    if opcode.getFormat().branchFits(&target)                    {
                        continue;
                    }                    
                    if opcode.getFamily()==Opcodes::GOTO                    {
                        opcode=findOpcodeForInsn(&insn, &opcode);
                        if opcode==None                        {
                            throw UnsupportedOperationException::new("method too long");
                        }                        
                        self.insns.set(i, insn.withOpcode(&opcode));
                    }                    else                     {
                        let newTarget: CodeAddress;
                        try                        {
                            newTarget=(CodeAddress*)self.insns.get(i+1);
                        }                        catch(                        let ex: IndexOutOfBoundsException)                        {
                            throw IllegalStateException::new("unpaired TargetInsn (dangling)");
                        }                        catch(                        let ex: ClassCastException)                        {
                            throw IllegalStateException::new("unpaired TargetInsn");
                        }
                        let gotoInsn: TargetInsn = TargetInsn::new(&Dops::GOTO, target.getPosition(), &RegisterSpecList::EMPTY, target.getTarget());
                        self.insns.set(i, &gotoInsn);
                        self.insns.add_int_DalvInsn(i, target.withNewTargetAndReversed(&newTarget));
                        size += 1;
                        i += 1;
                    }
                    anyFixed=true;
                }
                return anyFixed;
            }
            pub fn align64bits(&self, opcodes: &Vec<Dop>)            {
                while true                {
                    let notAligned64bitRegAccess: i32 = 0;
                    let aligned64bitRegAccess: i32 = 0;
                    let notAligned64bitParamAccess: i32 = 0;
                    let aligned64bitParamAccess: i32 = 0;
                    let lastParameter: i32 = self.unreservedRegCount+self.reservedCount+self.reservedParameterCount;
                    let firstParameter: i32 = lastParameter-self.paramSize;
                    for insn in self.insns                    {
                        let regs: RegisterSpecList = insn.getRegisters();
                        for(                        let usedRegIdx: i32 = 0;;usedRegIdx<regs.size()usedRegIdx += 1)                        {
                            let reg: RegisterSpec = regs.get(usedRegIdx);
                            if reg.isCategory2()                            {
                                let isParameter: boolean = reg.getReg()>=firstParameter;
                                if reg.isEvenRegister()                                {
                                    if isParameter                                    {
                                        aligned64bitParamAccess += 1;
                                    }                                    else                                     {
                                        aligned64bitRegAccess += 1;
                                    }
                                }                                else                                 {
                                    if isParameter                                    {
                                        notAligned64bitParamAccess += 1;
                                    }                                    else                                     {
                                        notAligned64bitRegAccess += 1;
                                    }
                                }
                            }                            
                        }
                    }
                    if notAligned64bitParamAccess>aligned64bitParamAccess&&notAligned64bitRegAccess>aligned64bitRegAccess                    {
                        addReservedRegisters(1);
                    }                    else                     if notAligned64bitParamAccess>aligned64bitParamAccess                    {
                        addReservedParameters(1);
                    }                    else                     if notAligned64bitRegAccess>aligned64bitRegAccess                    {
                        addReservedRegisters(1);
                        if self.paramSize!=0&&aligned64bitParamAccess>notAligned64bitParamAccess                        {
                            addReservedParameters(1);
                        }                        
                    }                    else                     {
                        break;
                    }
                    if !reserveRegisters(&opcodes)                    {
                        break;
                    }                    
                }
            }
            pub fn addReservedParameters(&self, delta: i32)            {
                shiftParameters(delta);
                self.reservedParameterCount+=delta;
            }
            pub fn addReservedRegisters(&self, delta: i32)            {
                shiftAllRegisters(delta);
                self.reservedCount+=delta;
            }
            pub fn shiftAllRegisters(&self, delta: i32)            {
                let insnSize: i32 = self.insns.size();
                for(                let i: i32 = 0;;i<insnSizei += 1)                {
                    let insn: DalvInsn = self.insns.get(i);
                    if !(//insn instanceof CodeAddress)                    {
                        self.insns.set(i, insn.withRegisterOffset(delta));
                    }                    
                }
            }
            pub fn shiftParameters(&self, delta: i32)            {
                let insnSize: i32 = self.insns.size();
                let lastParameter: i32 = self.unreservedRegCount+self.reservedCount+self.reservedParameterCount;
                let firstParameter: i32 = lastParameter-self.paramSize;
                let mapper: BasicRegisterMapper = BasicRegisterMapper::new(lastParameter);
                for(                let i: i32 = 0;;i<lastParameteri += 1)                {
                    if i>=firstParameter                    {
                        mapper.addMapping(i, i+delta, 1);
                    }                    else                     {
                        mapper.addMapping(i, i, 1);
                    }
                }
                for(                let i: i32 = 0;;i<insnSizei += 1)                {
                    let insn: DalvInsn = self.insns.get(i);
                    if !(//insn instanceof CodeAddress)                    {
                        self.insns.set(i, insn.withMapper(&mapper));
                    }                    
                }
            }
}
