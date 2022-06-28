use crate::helper;
use crate::com::android::dx::ssa::InterferenceRegisterMapper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::ssa::back::FirstFitLocalCombiningAllocator::Multiset;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::rop::code::TranslationAdvice;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::CstInsn;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::util::IntIterator;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::ssa::back::FirstFitLocalCombiningAllocator::Alignment;
use crate::com::android::dx::ssa::SsaInsn;

struct FirstFitLocalCombiningAllocator{
    pub localVariables: Map<LocalItem,ArrayList<RegisterSpec>>,
    pub moveResultPseudoInsns: ArrayList<NormalSsaInsn>,
    pub invokeRangeInsns: ArrayList<NormalSsaInsn>,
    pub phiInsns: ArrayList<PhiInsn>,
    pub ssaRegsMapped: BitSet,
    pub mapper: InterferenceRegisterMapper,
    pub paramRangeEnd: i32,
    pub reservedRopRegs: BitSet,
    pub usedRopRegs: BitSet,
    pub minimizeRegisters: boolean,
}
impl FirstFitLocalCombiningAllocator{
    pub const DEBUG: boolean = false;
    pub fn new(&self, ssaMeth: &SsaMethod, interference: &InterferenceGraph, minimizeRegisters: boolean)    {
        super(ssaMeth,interference);

        self.ssaRegsMapped=BitSet::new(ssaMeth.getRegCount());
        self.mapper=InterferenceRegisterMapper::new(&interference, ssaMeth.getRegCount());
        self->minimizeRegisters=minimizeRegisters;
        self.paramRangeEnd=ssaMeth.getParamWidth();
        self.reservedRopRegs=BitSet::new(self.paramRangeEnd*2);
        self.reservedRopRegs.set_int_int(0, self.paramRangeEnd);
        self.usedRopRegs=BitSet::new(self.paramRangeEnd*2);
        self.localVariables=TreeMap<LocalItem,ArrayList<RegisterSpec>>::new();
        self.moveResultPseudoInsns=ArrayList<NormalSsaInsn>::new();
        self.invokeRangeInsns=ArrayList<NormalSsaInsn>::new();
        self.phiInsns=ArrayList<PhiInsn>::new();
    }
    pub fn wantsParamsMovedHigh(&self) -> boolean    {
        return true;
    }
    pub fn allocateRegisters(&self) -> RegisterMapper    {
        analyzeInstructions();
        if FirstFitLocalCombiningAllocator::DEBUG        {
            printLocalVars();
        }        
        if FirstFitLocalCombiningAllocator::DEBUG{            System::out.println_String("--->Mapping local-associated params");        }        
        handleLocalAssociatedParams();
        if FirstFitLocalCombiningAllocator::DEBUG{            System::out.println_String("--->Mapping other params");        }        
        handleUnassociatedParameters();
        if FirstFitLocalCombiningAllocator::DEBUG{            System::out.println_String("--->Mapping invoke-range");        }        
        handleInvokeRangeInsns();
        if FirstFitLocalCombiningAllocator::DEBUG        {
            System::out.println_String("--->Mapping local-associated non-params");
        }        
        handleLocalAssociatedOther();
        if FirstFitLocalCombiningAllocator::DEBUG{            System::out.println_String("--->Mapping check-cast results");        }        
        handleCheckCastResults();
        if FirstFitLocalCombiningAllocator::DEBUG{            System::out.println_String("--->Mapping phis");        }        
        handlePhiInsns();
        if FirstFitLocalCombiningAllocator::DEBUG{            System::out.println_String("--->Mapping others");        }        
        handleNormalUnassociated();
        return self.mapper;
    }
    pub fn printLocalVars(&self)    {
        System::out.println_String("Printing local vars");
        for e in self.localVariables.entrySet()        {
            let regs: StringBuilder = StringBuilder::new();
            regs.append_char('{');
            regs.append_char(' ');
            for reg in e.getValue()            {
                regs.append_char('v');
                regs.append_int(reg.getReg());
                regs.append_char(' ');
            }
            regs.append_char('}');
            System::out.printf_String_Object[]("Local: %s Registers: %s\n", e.getKey(), &regs);
        }
    }
    pub fn handleLocalAssociatedParams(&self)    {
        for ssaRegs in self.localVariables.values()        {
            let sz: i32 = ssaRegs.size();
            let paramIndex: i32 = -1;
            let paramCategory: i32 = 0;
            for(            let i: i32 = 0;;i<szi += 1)            {
                let ssaSpec: RegisterSpec = ssaRegs.get(i);
                let ssaReg: i32 = ssaSpec.getReg();
                paramIndex=getParameterIndexForReg(ssaReg);
                if paramIndex>=0                {
                    paramCategory=ssaSpec.getCategory();
                    addMapping(&ssaSpec, paramIndex);
                    break;
                }                
            }
            if paramIndex<0            {
                continue;
            }            
            tryMapRegs(&ssaRegs, paramIndex, paramCategory, true);
        }
    }
    pub fn getParameterIndexForReg(&self, ssaReg: i32) -> i32    {
        let defInsn: SsaInsn = self.super_.ssaMeth.getDefinitionForRegister(ssaReg);
        if defInsn==None        {
            return -1;
        }        
        let opcode: Rop = defInsn.getOpcode();
        if opcode!=None&&opcode.getOpcode()==RegOps::MOVE_PARAM        {
            let origInsn: CstInsn = (CstInsn*)defInsn.getOriginalRopInsn();
            return ((CstInteger*)origInsn.getConstant()).getValue();
        }        
        return -1;
    }
    pub fn handleLocalAssociatedOther(&self)    {
        for specs in self.localVariables.values()        {
            let ropReg: i32 = self.paramRangeEnd;
            let done: boolean = false;
            loop            {
                let maxCategory: i32 = 1;                let sz: i32 = specs.size();                for(                let i: i32 = 0;;i<szi += 1)                {
                    let ssaSpec: RegisterSpec = specs.get(i);
                    let category: i32 = ssaSpec.getCategory();
                    if !self.ssaRegsMapped.get_int(ssaSpec.getReg())&&category>maxCategory                    {
                        maxCategory=category;
                    }                    
                }                ropReg=findRopRegForLocal(ropReg, maxCategory);                if canMapRegs(&specs, ropReg)                {
                    done=tryMapRegs(&specs, ropReg, maxCategory, true);
                }                                ropReg += 1;                if !!done{ break; }            }
        }
    }
    pub fn tryMapRegs(&self, specs: &ArrayList<RegisterSpec>, ropReg: i32, maxAllowedCategory: i32, markReserved: boolean) -> boolean    {
        let remaining: boolean = false;
        for spec in specs        {
            if self.ssaRegsMapped.get_int(spec.getReg())            {
                continue;
            }            
            let succeeded: boolean;
            succeeded=tryMapReg(&spec, ropReg, maxAllowedCategory);
            remaining=!succeeded||remaining;
            if succeeded&&markReserved            {
                markReserved(ropReg, spec.getCategory());
            }            
        }
        return !remaining;
    }
    pub fn tryMapReg(&self, ssaSpec: &RegisterSpec, ropReg: i32, maxAllowedCategory: i32) -> boolean    {
        if ssaSpec.getCategory()<=maxAllowedCategory&&!self.ssaRegsMapped.get_int(ssaSpec.getReg())&&canMapReg(&ssaSpec, ropReg)        {
            addMapping(&ssaSpec, ropReg);
            return true;
        }        
        return false;
    }
    pub fn markReserved(&self, ropReg: i32, category: i32)    {
        self.reservedRopRegs.set_int_int_boolean(ropReg, ropReg+category, true);
    }
    pub fn rangeContainsReserved(&self, ropRangeStart: i32, width: i32) -> boolean    {
        for(        let i: i32 = ropRangeStart;;i<(ropRangeStart+width)i += 1)        {
            if self.reservedRopRegs.get_int(i)            {
                return true;
            }            
        }
        return false;
    }
    pub fn isThisPointerReg(&self, startReg: i32) -> boolean    {
        return startReg==0&&!self.super_.ssaMeth.isStatic();
    }
    pub fn getAlignment(&self, regCategory: i32) -> Alignment    {
        let alignment: Alignment = Alignment::UNSPECIFIED;
        if DexOptions::ALIGN_64BIT_REGS_SUPPORT&&regCategory==2        {
            if FirstFitLocalCombiningAllocator::isEven(self.paramRangeEnd)            {
                alignment=Alignment::EVEN;
            }            else             {
                alignment=Alignment::ODD;
            }
        }        
        return alignment;
    }
    pub fn findNextUnreservedRopReg(&self, startReg: i32, regCategory: i32) -> i32    {
        return findNextUnreservedRopReg_int_int_Alignment(startReg, regCategory, getAlignment(regCategory));
    }
    pub fn findNextUnreservedRopReg(&self, startReg: i32, width: i32, alignment: &Alignment) -> i32    {
        let reg: i32 = alignment.nextClearBit(&self.reservedRopRegs, startReg);
        while true        {
            let i: i32 = 1;
            while i<width&&!self.reservedRopRegs.get_int(reg+i)            {
                i += 1;
            }
            if i==width            {
                return reg;
            }            
            reg=alignment.nextClearBit(&self.reservedRopRegs, reg+i);
        }
    }
    pub fn findRopRegForLocal(&self, startReg: i32, category: i32) -> i32    {
        let alignment: Alignment = getAlignment(category);
        let reg: i32 = alignment.nextClearBit(&self.usedRopRegs, startReg);
        while true        {
            let i: i32 = 1;
            while i<category&&!self.usedRopRegs.get_int(reg+i)            {
                i += 1;
            }
            if i==category            {
                return reg;
            }            
            reg=alignment.nextClearBit(&self.usedRopRegs, reg+i);
        }
    }
    pub fn handleUnassociatedParameters(&self)    {
        let szSsaRegs: i32 = self.super_.ssaMeth.getRegCount();
        for(        let ssaReg: i32 = 0;;ssaReg<szSsaRegsssaReg += 1)        {
            if self.ssaRegsMapped.get_int(ssaReg)            {
                continue;
            }            
            let paramIndex: i32 = getParameterIndexForReg(ssaReg);
            let ssaSpec: RegisterSpec = getDefinitionSpecForSsaReg(ssaReg);
            if paramIndex>=0            {
                addMapping(&ssaSpec, paramIndex);
            }            
        }
    }
    pub fn handleInvokeRangeInsns(&self)    {
        for insn in self.invokeRangeInsns        {
            adjustAndMapSourceRangeRange(&insn);
        }
    }
    pub fn handleCheckCastResults(&self)    {
        for insn in self.moveResultPseudoInsns        {
            let moveRegSpec: RegisterSpec = insn.getResult();
            let moveReg: i32 = moveRegSpec.getReg();
            let predBlocks: BitSet = insn.getBlock().getPredecessors();
            if predBlocks.cardinality()!=1            {
                continue;
            }            
            let predBlock: SsaBasicBlock = self.super_.ssaMeth.getBlocks().get(predBlocks.nextSetBit_int(0));
            let insnList: ArrayList<SsaInsn> = predBlock.getInsns();
            let checkCastInsn: SsaInsn = insnList.get(insnList.size()-1);
            if checkCastInsn.getOpcode().getOpcode()!=RegOps::CHECK_CAST            {
                continue;
            }            
            let checkRegSpec: RegisterSpec = checkCastInsn.getSources().get(0);
            let checkReg: i32 = checkRegSpec.getReg();
            let category: i32 = checkRegSpec.getCategory();
            let moveMapped: boolean = self.ssaRegsMapped.get_int(moveReg);
            let checkMapped: boolean = self.ssaRegsMapped.get_int(checkReg);
            if moveMapped&!checkMapped            {
                let moveRopReg: i32 = self.mapper.oldToNew(moveReg);
                checkMapped=tryMapReg(&checkRegSpec, moveRopReg, category);
            }            
            if checkMapped&!moveMapped            {
                let checkRopReg: i32 = self.mapper.oldToNew(checkReg);
                moveMapped=tryMapReg(&moveRegSpec, checkRopReg, category);
            }            
            if !moveMapped||!checkMapped            {
                let ropReg: i32 = findNextUnreservedRopReg_int_int(self.paramRangeEnd, category);
                let ssaRegs: ArrayList<RegisterSpec> = ArrayList<RegisterSpec>::new(2);
                ssaRegs.add_RegisterSpec(&moveRegSpec);
                ssaRegs.add_RegisterSpec(&checkRegSpec);
                while !tryMapRegs(&ssaRegs, ropReg, category, false)                {
                    ropReg=findNextUnreservedRopReg_int_int(ropReg+1, category);
                }
            }            
            let hasExceptionHandlers: boolean = checkCastInsn.getOriginalRopInsn().getCatches().size()!=0;
            let moveRopReg: i32 = self.mapper.oldToNew(moveReg);
            let checkRopReg: i32 = self.mapper.oldToNew(checkReg);
            if moveRopReg!=checkRopReg&&!hasExceptionHandlers            {
                ((NormalSsaInsn*)checkCastInsn).changeOneSource(0, insertMoveBefore(&checkCastInsn, &checkRegSpec));
                addMapping(checkCastInsn.getSources().get(0), moveRopReg);
            }            
        }
    }
    pub fn handlePhiInsns(&self)    {
        for insn in self.phiInsns        {
            processPhiInsn(&insn);
        }
    }
    pub fn handleNormalUnassociated(&self)    {
        let szSsaRegs: i32 = self.super_.ssaMeth.getRegCount();
        for(        let ssaReg: i32 = 0;;ssaReg<szSsaRegsssaReg += 1)        {
            if self.ssaRegsMapped.get_int(ssaReg)            {
                continue;
            }            
            let ssaSpec: RegisterSpec = getDefinitionSpecForSsaReg(ssaReg);
            if ssaSpec==None{                continue;            }            
            let category: i32 = ssaSpec.getCategory();
            let ropReg: i32 = findNextUnreservedRopReg_int_int(self.paramRangeEnd, category);
            while !canMapReg(&ssaSpec, ropReg)            {
                ropReg=findNextUnreservedRopReg_int_int(ropReg+1, category);
            }
            addMapping(&ssaSpec, ropReg);
        }
    }
    pub fn canMapRegs(&self, specs: &ArrayList<RegisterSpec>, ropReg: i32) -> boolean    {
        for spec in specs        {
            if self.ssaRegsMapped.get_int(spec.getReg()){                continue;            }            
            if !canMapReg(&spec, ropReg){                return false;            }            
        }
        return true;
    }
    pub fn canMapReg(&self, ssaSpec: &RegisterSpec, ropReg: i32) -> boolean    {
        let category: i32 = ssaSpec.getCategory();
        return !(spansParamRange(ropReg, category)||self.mapper.interferes_RegisterSpec_int(&ssaSpec, ropReg));
    }
    pub fn spansParamRange(&self, ssaReg: i32, category: i32) -> boolean    {
        return ((ssaReg<self.paramRangeEnd)&&((ssaReg+category)>self.paramRangeEnd));
    }
    pub fn analyzeInstructions(&self)    {
        self.super_.ssaMeth.forEachInsn(/*new SsaInsn.Visitor(){
  /** 
 * {@inheritDoc} 
 */
  @Override public void visitMoveInsn(  NormalSsaInsn insn){
    processInsn(insn);
  }
  /** 
 * {@inheritDoc} 
 */
  @Override public void visitPhiInsn(  PhiInsn insn){
    processInsn(insn);
  }
  /** 
 * {@inheritDoc} 
 */
  @Override public void visitNonMoveInsn(  NormalSsaInsn insn){
    processInsn(insn);
  }
  /** 
 * This method collects three types of instructions: 1) Adds a local variable assignment to the {@code localVariables} map.2) Add move-result-pseudo to the {@code moveResultPseudoInsns} list.3) Add invoke-range to the {@code invokeRangeInsns} list.
 * @param insn {@code non-null;} insn that may represent alocal variable assignment
 */
  private void processInsn(  SsaInsn insn){
    RegisterSpec assignment;
    assignment=insn.getLocalAssignment();
    if (assignment != null) {
      LocalItem local=assignment.getLocalItem();
      ArrayList<RegisterSpec> regList=localVariables.get(local);
      if (regList == null) {
        regList=new ArrayList<RegisterSpec>();
        localVariables.put(local,regList);
      }
      regList.add(assignment);
    }
    if (insn instanceof NormalSsaInsn) {
      if (insn.getOpcode().getOpcode() == RegOps.MOVE_RESULT_PSEUDO) {
        moveResultPseudoInsns.add((NormalSsaInsn)insn);
      }
 else       if (Optimizer.getAdvice().requiresSourcesInOrder(insn.getOriginalRopInsn().getOpcode(),insn.getSources())) {
        invokeRangeInsns.add((NormalSsaInsn)insn);
      }
    }
 else     if (insn instanceof PhiInsn) {
      phiInsns.add((PhiInsn)insn);
    }
  }
}
*/);
    }
    pub fn addMapping(&self, ssaSpec: &RegisterSpec, ropReg: i32)    {
        let ssaReg: i32 = ssaSpec.getReg();
        if self.ssaRegsMapped.get_int(ssaReg)||!canMapReg(&ssaSpec, ropReg)        {
            throw RuntimeException::new("attempt to add invalid register mapping");
        }        
        if FirstFitLocalCombiningAllocator::DEBUG        {
            System::out.printf_String_Object[]("Add mapping s%d -> v%d c:%d\n", ssaSpec.getReg(), ropReg, ssaSpec.getCategory());
        }        
        let category: i32 = ssaSpec.getCategory();
        self.mapper.addMapping(ssaSpec.getReg(), ropReg, category);
        self.ssaRegsMapped.set_int(ssaReg);
        self.usedRopRegs.set_int_int(ropReg, ropReg+category);
    }
    pub fn adjustAndMapSourceRangeRange(&self, insn: &NormalSsaInsn)    {
        let newRegStart: i32 = findRangeAndAdjust(&insn);
        let sources: RegisterSpecList = insn.getSources();
        let szSources: i32 = sources.size();
        let nextRopReg: i32 = newRegStart;
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            let source: RegisterSpec = sources.get(i);
            let sourceReg: i32 = source.getReg();
            let category: i32 = source.getCategory();
            let curRopReg: i32 = nextRopReg;
            nextRopReg+=category;
            if self.ssaRegsMapped.get_int(sourceReg)            {
                continue;
            }            
            let localItem: LocalItem = getLocalItemForReg(sourceReg);
            addMapping(&source, curRopReg);
            if localItem!=None            {
                markReserved(curRopReg, category);
                let similarRegisters: ArrayList<RegisterSpec> = self.localVariables.get(&localItem);
                let szSimilar: i32 = similarRegisters.size();
                for(                let j: i32 = 0;;j<szSimilarj += 1)                {
                    let similarSpec: RegisterSpec = similarRegisters.get(j);
                    let similarReg: i32 = similarSpec.getReg();
                    if -1!=sources.indexOfRegister(similarReg)                    {
                        continue;
                    }                    
                    tryMapReg(&similarSpec, curRopReg, category);
                }
            }            
        }
    }
    pub fn findRangeAndAdjust(&self, insn: &NormalSsaInsn) -> i32    {
        let sources: RegisterSpecList = insn.getSources();
        let szSources: i32 = sources.size();
        let categoriesForIndex: i32 = new int[szSources];
        let rangeLength: i32 = 0;
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            let category: i32 = sources.get(i).getCategory();
            categoriesForIndex[i]=category;
            rangeLength+=categoriesForIndex[i];
        }
        let maxScore: i32 = Integer::MIN_VALUE;
        let resultRangeStart: i32 = -1;
        let resultMovesRequired: BitSet = None;
        let rangeStartOffset: i32 = 0;
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            let ssaCenterReg: i32 = sources.get(i).getReg();
            if i!=0            {
                rangeStartOffset-=categoriesForIndex[i-1];
            }            
            if !self.ssaRegsMapped.get_int(ssaCenterReg)            {
                continue;
            }            
            let rangeStart: i32 = self.mapper.oldToNew(ssaCenterReg)+rangeStartOffset;
            if rangeStart<0||spansParamRange(rangeStart, rangeLength)            {
                continue;
            }            
            let curMovesRequired: BitSet = BitSet::new(szSources);
            let fitWidth: i32 = fitPlanForRange(rangeStart, &insn, &categoriesForIndex, &curMovesRequired);
            if fitWidth<0            {
                continue;
            }            
            let score: i32 = fitWidth-curMovesRequired.cardinality();
            if score>maxScore            {
                maxScore=score;
                resultRangeStart=rangeStart;
                resultMovesRequired=curMovesRequired;
            }            
            if fitWidth==rangeLength            {
                break;
            }            
        }
        if resultRangeStart==-1        {
            resultMovesRequired=BitSet::new(szSources);
            resultRangeStart=findAnyFittingRange(&insn, rangeLength, &categoriesForIndex, &resultMovesRequired);
        }        
        for(        let i: i32 = resultMovesRequired.nextSetBit_int(0);;i>=0i=resultMovesRequired.nextSetBit_int(i+1))        {
            insn.changeOneSource(i, insertMoveBefore(&insn, sources.get(i)));
        }
        return resultRangeStart;
    }
    pub fn findAnyFittingRange(&self, insn: &NormalSsaInsn, rangeLength: i32, categoriesForIndex: &Vec<i32>, outMovesRequired: &BitSet) -> i32    {
        let alignment: Alignment = Alignment::UNSPECIFIED;
        if DexOptions::ALIGN_64BIT_REGS_SUPPORT        {
            let regNumber: i32 = 0;
            let p64bitsAligned: i32 = 0;
            let p64bitsNotAligned: i32 = 0;
            for category in categoriesForIndex            {
                if category==2                {
                    if FirstFitLocalCombiningAllocator::isEven(regNumber)                    {
                        p64bitsAligned += 1;
                    }                    else                     {
                        p64bitsNotAligned += 1;
                    }
                    regNumber+=2;
                }                else                 {
                    regNumber+=1;
                }
            }
            if p64bitsNotAligned>p64bitsAligned            {
                if FirstFitLocalCombiningAllocator::isEven(self.paramRangeEnd)                {
                    alignment=Alignment::ODD;
                }                else                 {
                    alignment=Alignment::EVEN;
                }
            }            else             if p64bitsAligned>0            {
                if FirstFitLocalCombiningAllocator::isEven(self.paramRangeEnd)                {
                    alignment=Alignment::EVEN;
                }                else                 {
                    alignment=Alignment::ODD;
                }
            }            
        }        
        let rangeStart: i32 = self.paramRangeEnd;
        while true        {
            rangeStart=findNextUnreservedRopReg_int_int_Alignment(rangeStart, rangeLength, &alignment);
            let fitWidth: i32 = fitPlanForRange(rangeStart, &insn, &categoriesForIndex, &outMovesRequired);
            if fitWidth>=0            {
                break;
            }            
            rangeStart += 1;
            outMovesRequired.clear();
        }
        return rangeStart;
    }
    pub fn fitPlanForRange(&self, ropReg: i32, insn: &NormalSsaInsn, categoriesForIndex: &Vec<i32>, outMovesRequired: &BitSet) -> i32    {
        let sources: RegisterSpecList = insn.getSources();
        let szSources: i32 = sources.size();
        let fitWidth: i32 = 0;
        let liveOut: IntSet = insn.getBlock().getLiveOutRegs();
        let liveOutSpecs: RegisterSpecList = ssaSetToSpecs(&liveOut);
        let seen: BitSet = BitSet::new(self.super_.ssaMeth.getRegCount());
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            let ssaSpec: RegisterSpec = sources.get(i);
            let ssaReg: i32 = ssaSpec.getReg();
            let category: i32 = categoriesForIndex[i];
            if i!=0            {
                ropReg+=categoriesForIndex[i-1];
            }            
            if self.ssaRegsMapped.get_int(ssaReg)&&self.mapper.oldToNew(ssaReg)==ropReg            {
                fitWidth+=category;
            }            else             if rangeContainsReserved(ropReg, category)            {
                fitWidth=-1;
                break;
            }            else             if !self.ssaRegsMapped.get_int(ssaReg)&&canMapReg(&ssaSpec, ropReg)&&!seen.get_int(ssaReg)            {
                fitWidth+=category;
            }            else             if !self.mapper.areAnyPinned(&liveOutSpecs, ropReg, category)&&!self.mapper.areAnyPinned(&sources, ropReg, category)            {
                outMovesRequired.set_int(i);
            }            else             {
                fitWidth=-1;
                break;
            }
            seen.set_int(ssaReg);
        }
        return fitWidth;
    }
    pub fn ssaSetToSpecs(&self, ssaSet: &IntSet) -> RegisterSpecList    {
        let result: RegisterSpecList = RegisterSpecList::new(ssaSet.elements());
        let iter: IntIterator = ssaSet.iterator();
        let i: i32 = 0;
        while iter.hasNext()        {
            result.set_int_RegisterSpec(i += 1, getDefinitionSpecForSsaReg(iter.next()));
        }
        return result;
    }
    pub fn getLocalItemForReg(&self, ssaReg: i32) -> LocalItem    {
        for entry in self.localVariables.entrySet()        {
            for spec in entry.getValue()            {
                if spec.getReg()==ssaReg                {
                    return entry.getKey();
                }                
            }
        }
        return None;
    }
    pub fn processPhiInsn(&self, insn: &PhiInsn)    {
        let result: RegisterSpec = insn.getResult();
        let resultReg: i32 = result.getReg();
        let category: i32 = result.getCategory();
        let sources: RegisterSpecList = insn.getSources();
        let sourcesSize: i32 = sources.size();
        let ssaRegs: ArrayList<RegisterSpec> = ArrayList<RegisterSpec>::new();
        let mapSet: Multiset = Multiset::new(sourcesSize+1);
        if self.ssaRegsMapped.get_int(resultReg)        {
            mapSet.add(self.mapper.oldToNew(resultReg));
        }        else         {
            ssaRegs.add_RegisterSpec(&result);
        }
        for(        let i: i32 = 0;;i<sourcesSizei += 1)        {
            let source: RegisterSpec = sources.get(i);
            let def: SsaInsn = self.super_.ssaMeth.getDefinitionForRegister(source.getReg());
            let sourceDef: RegisterSpec = def.getResult();
            let sourceReg: i32 = sourceDef.getReg();
            if self.ssaRegsMapped.get_int(sourceReg)            {
                mapSet.add(self.mapper.oldToNew(sourceReg));
            }            else             {
                ssaRegs.add_RegisterSpec(&sourceDef);
            }
        }
        for(        let i: i32 = 0;;i<mapSet.getSize()i += 1)        {
            let maxReg: i32 = mapSet.getAndRemoveHighestCount();
            tryMapRegs(&ssaRegs, maxReg, category, false);
        }
        let mapReg: i32 = findNextUnreservedRopReg_int_int(self.paramRangeEnd, category);
        while !tryMapRegs(&ssaRegs, mapReg, category, false)        {
            mapReg=findNextUnreservedRopReg_int_int(mapReg+1, category);
        }
    }
    pub fn isEven(regNumger: i32) -> boolean    {
        return ((regNumger&1)==0);
    }
}
