use crate::helper;
use crate::com::android::dx::ssa::EscapeAnalysis::EscapeSet;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::ssa::EscapeAnalysis::EscapeState;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::ssa::SsaConverter;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::ssa::EscapeAnalysis;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::Exceptions;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::FillArrayDataInsn;
use crate::com::android::dx::ssa::SsaBasicBlock::Visitor;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::rop::cst::Zeroes;
use crate::com::android::dx::rop::cst::TypedConstant;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::rop::cst::CstNat;

struct EscapeAnalysis{
    pub ssaMeth: SsaMethod,
    pub regCount: i32,
    pub latticeValues: ArrayList<EscapeSet>,
}
impl EscapeAnalysis{
    pub fn new(&self, ssaMeth: &SsaMethod)    {
        self->ssaMeth=ssaMeth;
        self->regCount=ssaMeth.getRegCount();
        self->latticeValues=ArrayList<EscapeSet>::new();
    }
    pub fn findSetIndex(&self, reg: &RegisterSpec) -> i32    {
        let i: i32;
        for(i=0;i<self.latticeValues.size()i += 1)        {
            let e: EscapeSet = self.latticeValues.get(i);
            if .get_int(reg.getReg())            {
                return i;
            }            
        }
        return i;
    }
    pub fn getInsnForMove(&self, moveInsn: &SsaInsn) -> SsaInsn    {
        let pred: i32 = moveInsn.getBlock().getPredecessors().nextSetBit_int(0);
        let predInsns: ArrayList<SsaInsn> = self.ssaMeth.getBlocks().get(pred).getInsns();
        return predInsns.get(predInsns.size()-1);
    }
    pub fn getMoveForInsn(&self, insn: &SsaInsn) -> SsaInsn    {
        let succ: i32 = insn.getBlock().getSuccessors().nextSetBit_int(0);
        let succInsns: ArrayList<SsaInsn> = self.ssaMeth.getBlocks().get(succ).getInsns();
        return succInsns.get(0);
    }
    pub fn addEdge(&self, parentSet: &EscapeSet, childSet: &EscapeSet)    {
        if !.contains(&parentSet)        {
            .add_EscapeSet(&parentSet);
        }        
        if !.contains(&childSet)        {
            .add_EscapeSet(&childSet);
        }        
    }
    pub fn replaceNode(&self, newNode: &EscapeSet, oldNode: &EscapeSet)    {
        for e in         {
            .remove_Object(&oldNode);
            .add_EscapeSet(&newNode);
            .add_EscapeSet(&e);
        }
        for e in         {
            .remove_Object(&oldNode);
            .add_EscapeSet(&newNode);
            .add_EscapeSet(&e);
        }
    }
    pub fn process(ssaMethod: &SsaMethod)    {
        EscapeAnalysis::new(&ssaMethod).run();
    }
    pub fn processInsn(&self, insn: &SsaInsn)    {
        let op: i32 = insn.getOpcode().getOpcode();
        let result: RegisterSpec = insn.getResult();
        let escSet: EscapeSet;
        if op==RegOps::MOVE_RESULT_PSEUDO&&result.getTypeBearer().getBasicType()==Type::BT_OBJECT        {
            escSet=processMoveResultPseudoInsn(&insn);
            processRegister(&result, &escSet);
        }        else         if op==RegOps::MOVE_PARAM&&result.getTypeBearer().getBasicType()==Type::BT_OBJECT        {
            escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::NONE);
            self.latticeValues.add_EscapeSet(&escSet);
            processRegister(&result, &escSet);
        }        else         if op==RegOps::MOVE_RESULT&&result.getTypeBearer().getBasicType()==Type::BT_OBJECT        {
            escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::NONE);
            self.latticeValues.add_EscapeSet(&escSet);
            processRegister(&result, &escSet);
        }        
    }
    pub fn processMoveResultPseudoInsn(&self, insn: &SsaInsn) -> EscapeSet    {
        let result: RegisterSpec = insn.getResult();
        let prevSsaInsn: SsaInsn = getInsnForMove(&insn);
        let prevOpcode: i32 = prevSsaInsn.getOpcode().getOpcode();
        let escSet: EscapeSet;
        let prevSource: RegisterSpec;
        match prevOpcode{RegOps::NEW_INSTANCE => RegOps::CONST =>             escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::NONE);            break;RegOps::NEW_ARRAY => RegOps::FILLED_NEW_ARRAY =>             prevSource=prevSsaInsn.getSources().get(0);            if prevSource.getTypeBearer().isConstant()            {
                escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::NONE);
                =true;
            }            else             {
                escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::GLOBAL);
            }            break;RegOps::GET_STATIC =>             escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::GLOBAL);            break;RegOps::CHECK_CAST => RegOps::GET_FIELD => RegOps::AGET =>             prevSource=prevSsaInsn.getSources().get(0);            let setIndex: i32 = findSetIndex(&prevSource);            if setIndex!=self.latticeValues.size()            {
                escSet=self.latticeValues.get(setIndex);
                .set_int(result.getReg());
                return escSet;
            }                        if prevSource.getType()==Type::KNOWN_NULL            {
                escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::NONE);
            }            else             {
                escSet=EscapeSet::new(result.getReg(), self.regCount, &EscapeState::GLOBAL);
            }            break;        _ => {}        return None;    }
    self.latticeValues.add_EscapeSet(&escSet);
    return escSet;
}
pub fn processRegister(&self, result: &RegisterSpec, escSet: &EscapeSet){
    let regWorklist: ArrayList<RegisterSpec> = ArrayList<RegisterSpec>::new();
    regWorklist.add_RegisterSpec(&result);
    while !regWorklist.isEmpty()    {
        let listSize: i32 = regWorklist.size()-1;
        let def: RegisterSpec = regWorklist.remove_int(listSize);
        let useList: List<SsaInsn> = self.ssaMeth.getUseListForRegister(def.getReg());
        for use in useList        {
            let useOpcode: Rop = use_renamed.getOpcode();
            if useOpcode==None            {
                processPhiUse(&use_renamed, &escSet, &regWorklist);
            }            else             {
                processUse(&def, &use_renamed, &escSet, &regWorklist);
            }
        }
    }
}
pub fn processPhiUse(&self, use: &SsaInsn, escSet: &EscapeSet, regWorklist: &ArrayList<RegisterSpec>){
    let setIndex: i32 = findSetIndex(use.getResult());
    if setIndex!=self.latticeValues.size()    {
        let mergeSet: EscapeSet = self.latticeValues.get(setIndex);
        if mergeSet!=escSet        {
            =false;
            .or_BitSet(&);
            if .compareTo(&)<0            {
                =;
            }            
            replaceNode(&escSet, &mergeSet);
            self.latticeValues.remove_int(setIndex);
        }        
    }    else     {
        .set_int(use.getResult().getReg());
        regWorklist.add_RegisterSpec(use.getResult());
    }
}
pub fn processUse(&self, def: &RegisterSpec, use: &SsaInsn, escSet: &EscapeSet, regWorklist: &ArrayList<RegisterSpec>){
    let useOpcode: i32 = use.getOpcode().getOpcode();
    match useOpcode{RegOps::MOVE =>         .set_int(use.getResult().getReg());        regWorklist.add_RegisterSpec(use.getResult());        break;RegOps::IF_EQ => RegOps::IF_NE => RegOps::CHECK_CAST =>         if .compareTo(&EscapeState::METHOD)<0        {
            =EscapeState::METHOD;
        }                break;RegOps::APUT =>         let putIndex: RegisterSpec = use.getSources().get(2);        if !putIndex.getTypeBearer().isConstant()        {
            =false;
        }        RegOps::PUT_FIELD =>         let putValue: RegisterSpec = use.getSources().get(0);        if putValue.getTypeBearer().getBasicType()!=Type::BT_OBJECT        {
            break;
        }                =false;        let sources: RegisterSpecList = use.getSources();        if sources.get(0).getReg()==def.getReg()        {
            let setIndex: i32 = findSetIndex(sources.get(1));
            if setIndex!=self.latticeValues.size()            {
                let parentSet: EscapeSet = self.latticeValues.get(setIndex);
                addEdge(&parentSet, &escSet);
                if .compareTo(&)<0                {
                    =;
                }                
            }            
        }        else         {
            let setIndex: i32 = findSetIndex(sources.get(0));
            if setIndex!=self.latticeValues.size()            {
                let childSet: EscapeSet = self.latticeValues.get(setIndex);
                addEdge(&escSet, &childSet);
                if .compareTo(&)<0                {
                    =;
                }                
            }            
        }        break;RegOps::AGET =>         let getIndex: RegisterSpec = use.getSources().get(1);        if !getIndex.getTypeBearer().isConstant()        {
            =false;
        }                break;RegOps::PUT_STATIC =>         =EscapeState::GLOBAL;        break;RegOps::INVOKE_STATIC => RegOps::INVOKE_VIRTUAL => RegOps::INVOKE_SUPER => RegOps::INVOKE_DIRECT => RegOps::INVOKE_INTERFACE => RegOps::RETURN => RegOps::THROW =>         =EscapeState::INTER;        break;    _ => {}    break;}
}
pub fn scalarReplacement(&self){
for escSet in self.latticeValues{
    if !||!=EscapeState::NONE    {
        continue;
    }    
    let e: i32 = .nextSetBit_int(0);
    let def: SsaInsn = self.ssaMeth.getDefinitionForRegister(e);
    let prev: SsaInsn = getInsnForMove(&def);
    let lengthReg: TypeBearer = prev.getSources().get(0).getTypeBearer();
    let length: i32 = ((CstLiteralBits*)lengthReg).getIntBits();
    let newRegs: ArrayList<RegisterSpec> = ArrayList<RegisterSpec>::new(length);
    let deletedInsns: HashSet<SsaInsn> = HashSet<SsaInsn>::new();
    replaceDef(&def, &prev, length, &newRegs);
    deletedInsns.add(&prev);
    deletedInsns.add(&def);
    let useList: List<SsaInsn> = self.ssaMeth.getUseListForRegister(e);
    for use in useList    {
        replaceUse(&use_renamed, &prev, &newRegs, &deletedInsns);
        deletedInsns.add(&use_renamed);
    }
    self.ssaMeth.deleteInsns(&deletedInsns);
    self.ssaMeth.onInsnsChanged();
    SsaConverter::updateSsaMethod(&self.ssaMeth, self.regCount);
    movePropagate();
}
}
pub fn replaceDef(&self, def: &SsaInsn, prev: &SsaInsn, length: i32, newRegs: &ArrayList<RegisterSpec>){
let resultType: Type = def.getResult().getType();
for(let i: i32 = 0;;i<lengthi += 1){
    let newZero: Constant = Zeroes::zeroFor(resultType.getComponentType());
    let typedZero: TypedConstant = (TypedConstant*)newZero;
    let newReg: RegisterSpec = RegisterSpec::make_int_TypeBearer(self.ssaMeth.makeNewSsaReg(), &typedZero);
    newRegs.add_RegisterSpec(&newReg);
    insertPlainInsnBefore(&def, &RegisterSpecList::EMPTY, &newReg, RegOps::CONST, &newZero);
}
}
pub fn replaceUse(&self, use: &SsaInsn, prev: &SsaInsn, newRegs: &ArrayList<RegisterSpec>, deletedInsns: &HashSet<SsaInsn>){
let index: i32;
let length: i32 = newRegs.size();
let next: SsaInsn;
let sources: RegisterSpecList;
let source: RegisterSpec;let result: RegisterSpec;
let indexReg: CstLiteralBits;
match use.getOpcode().getOpcode(){RegOps::AGET =>     next=getMoveForInsn(&use);    sources=use.getSources();    indexReg=((CstLiteralBits*)sources.get(1).getTypeBearer());    index=indexReg.getIntBits();    if index<length    {
        source=newRegs.get(index);
        result=source.withReg(next.getResult().getReg());
        insertPlainInsnBefore(&next, RegisterSpecList::make_RegisterSpec(&source), &result, RegOps::MOVE, None);
    }    else     {
        insertExceptionThrow(&next, sources.get(1), &deletedInsns);
        deletedInsns.add(next.getBlock().getInsns().get(2));
    }    deletedInsns.add(&next);    break;RegOps::APUT =>     sources=use.getSources();    indexReg=((CstLiteralBits*)sources.get(2).getTypeBearer());    index=indexReg.getIntBits();    if index<length    {
        source=sources.get(0);
        result=source.withReg(newRegs.get(index).getReg());
        insertPlainInsnBefore(&use, RegisterSpecList::make_RegisterSpec(&source), &result, RegOps::MOVE, None);
        newRegs.set(index, result.withSimpleType());
    }    else     {
        insertExceptionThrow(&use, sources.get(2), &deletedInsns);
    }    break;RegOps::ARRAY_LENGTH =>     let lengthReg: TypeBearer = prev.getSources().get(0).getTypeBearer();    next=getMoveForInsn(&use);    insertPlainInsnBefore(&next, &RegisterSpecList::EMPTY, next.getResult(), RegOps::CONST, (Constant*)lengthReg);    deletedInsns.add(&next);    break;RegOps::MARK_LOCAL =>     break;RegOps::FILL_ARRAY_DATA =>     let ropUse: Insn = use.getOriginalRopInsn();    let fill: FillArrayDataInsn = (FillArrayDataInsn*)ropUse;    let constList: ArrayList<Constant> = fill.getInitValues();    for(    let i: i32 = 0;;i<lengthi += 1)    {
        let newFill: RegisterSpec = RegisterSpec::make_int_TypeBearer(newRegs.get(i).getReg(), (TypeBearer*)constList.get(i));
        insertPlainInsnBefore(&use, &RegisterSpecList::EMPTY, &newFill, RegOps::CONST, constList.get(i));
        newRegs.set(i, &newFill);
    }    break;_ => {}}
}
pub fn movePropagate(&self){
for(let i: i32 = 0;;i<self.ssaMeth.getRegCount()i += 1){
let insn: SsaInsn = self.ssaMeth.getDefinitionForRegister(i);
if insn==None||insn.getOpcode()==None||insn.getOpcode().getOpcode()!=RegOps::MOVE{
    continue;
}
let useList: Vec<ArrayList<SsaInsn>> = self.ssaMeth.getUseListCopy();
let source: RegisterSpec = insn.getSources().get(0);
let result: RegisterSpec = insn.getResult();
if source.getReg()<self.regCount&&result.getReg()<self.regCount{
    continue;
}
let mapper: RegisterMapper = /*new RegisterMapper(){
  @Override public int getNewRegisterCount(){
    return ssaMeth.getRegCount();
  }
  @Override public RegisterSpec map(  RegisterSpec registerSpec){
    if (registerSpec.getReg() == result.getReg()) {
      return source;
    }
    return registerSpec;
  }
}
*/;
for use in useList[result.getReg()]{
    use_renamed.mapSourceRegisters(&mapper);
}
}
}
pub fn run(&self){
self.ssaMeth.forEachBlockDepthFirstDom(/*new SsaBasicBlock.Visitor(){
  @Override public void visitBlock(  SsaBasicBlock block,  SsaBasicBlock unused){
    block.forEachInsn(new SsaInsn.Visitor(){
      @Override public void visitMoveInsn(      NormalSsaInsn insn){
      }
      @Override public void visitPhiInsn(      PhiInsn insn){
      }
      @Override public void visitNonMoveInsn(      NormalSsaInsn insn){
        processInsn(insn);
      }
    }
);
  }
}
*/);
for e in self.latticeValues{
if !=EscapeState::NONE{
    for field in     {
        if .compareTo(&)>0        {
            =;
        }        
    }
}
}
scalarReplacement();
}
pub fn insertExceptionThrow(&self, insn: &SsaInsn, index: &RegisterSpec, deletedInsns: &HashSet<SsaInsn>){
let exception: CstType = CstType::new(&Exceptions::TYPE_ArrayIndexOutOfBoundsException);
insertThrowingInsnBefore(&insn, &RegisterSpecList::EMPTY, None, RegOps::NEW_INSTANCE, &exception);
let currBlock: SsaBasicBlock = insn.getBlock();
let newBlock: SsaBasicBlock = currBlock.insertNewSuccessor(currBlock.getPrimarySuccessor());
let newInsn: SsaInsn = newBlock.getInsns().get(0);
let newReg: RegisterSpec = RegisterSpec::make_int_TypeBearer(self.ssaMeth.makeNewSsaReg(), &exception);
insertPlainInsnBefore(&newInsn, &RegisterSpecList::EMPTY, &newReg, RegOps::MOVE_RESULT_PSEUDO, None);
let newBlock2: SsaBasicBlock = newBlock.insertNewSuccessor(newBlock.getPrimarySuccessor());
let newInsn2: SsaInsn = newBlock2.getInsns().get(0);
let newNat: CstNat = CstNat::new(CstString::new("<init>"), CstString::new("(I)V"));
let newRef: CstMethodRef = CstMethodRef::new(&exception, &newNat);
insertThrowingInsnBefore(&newInsn2, RegisterSpecList::make_RegisterSpec_RegisterSpec(&newReg, &index), None, RegOps::INVOKE_DIRECT, &newRef);
deletedInsns.add(&newInsn2);
let newBlock3: SsaBasicBlock = newBlock2.insertNewSuccessor(newBlock2.getPrimarySuccessor());
let newInsn3: SsaInsn = newBlock3.getInsns().get(0);
insertThrowingInsnBefore(&newInsn3, RegisterSpecList::make_RegisterSpec(&newReg), None, RegOps::THROW, None);
newBlock3.replaceSuccessor(newBlock3.getPrimarySuccessorIndex(), self.ssaMeth.getExitBlock().getIndex());
deletedInsns.add(&newInsn3);
}
pub fn insertPlainInsnBefore(&self, insn: &SsaInsn, newSources: &RegisterSpecList, newResult: &RegisterSpec, newOpcode: i32, cst: &Constant){
let originalRopInsn: Insn = insn.getOriginalRopInsn();
let newRop: Rop;
if newOpcode==RegOps::MOVE_RESULT_PSEUDO{
newRop=Rops::opMoveResultPseudo(newResult.getType());
}else {
newRop=Rops::ropFor(newOpcode, &newResult, &newSources, &cst);
}
let newRopInsn: Insn;
if cst==None{
newRopInsn=PlainInsn::new(&newRop, originalRopInsn.getPosition(), &newResult, &newSources);
}else {
newRopInsn=PlainCstInsn::new(&newRop, originalRopInsn.getPosition(), &newResult, &newSources, &cst);
}
let newInsn: NormalSsaInsn = NormalSsaInsn::new(&newRopInsn, insn.getBlock());
let insns: List<SsaInsn> = insn.getBlock().getInsns();
insns.add_int_SsaInsn(insns.lastIndexOf(&insn), &newInsn);
self.ssaMeth.onInsnAdded(&newInsn);
}
pub fn insertThrowingInsnBefore(&self, insn: &SsaInsn, newSources: &RegisterSpecList, newResult: &RegisterSpec, newOpcode: i32, cst: &Constant){
let origRopInsn: Insn = insn.getOriginalRopInsn();
let newRop: Rop = Rops::ropFor(newOpcode, &newResult, &newSources, &cst);
let newRopInsn: Insn;
if cst==None{
newRopInsn=ThrowingInsn::new(&newRop, origRopInsn.getPosition(), &newSources, &StdTypeList::EMPTY);
}else {
newRopInsn=ThrowingCstInsn::new(&newRop, origRopInsn.getPosition(), &newSources, &StdTypeList::EMPTY, &cst);
}
let newInsn: NormalSsaInsn = NormalSsaInsn::new(&newRopInsn, insn.getBlock());
let insns: List<SsaInsn> = insn.getBlock().getInsns();
insns.add_int_SsaInsn(insns.lastIndexOf(&insn), &newInsn);
self.ssaMeth.onInsnAdded(&newInsn);
}
}
