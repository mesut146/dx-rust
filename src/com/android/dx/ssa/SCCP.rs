use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::CstInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::ssa::SCCP;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::TypedConstant;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::PlainInsn;

struct SCCP{
    pub ssaMeth: SsaMethod,
    pub regCount: i32,
    pub latticeValues: Vec<i32>,
    pub latticeConstants: Vec<Constant>,
    pub cfgWorklist: ArrayList<SsaBasicBlock>,
    pub cfgPhiWorklist: ArrayList<SsaBasicBlock>,
    pub executableBlocks: BitSet,
    pub ssaWorklist: ArrayList<SsaInsn>,
    pub varyingWorklist: ArrayList<SsaInsn>,
    pub branchWorklist: ArrayList<SsaInsn>,
}
impl SCCP{
    pub const TOP: i32 = 0;
    pub const CONSTANT: i32 = 1;
    pub const VARYING: i32 = 2;
    pub fn new(&self, ssaMeth: &SsaMethod)    {
        self->ssaMeth=ssaMeth;
        self->regCount=ssaMeth.getRegCount();
        self->latticeValues=new int[this.regCount];
        self->latticeConstants=new Constant[this.regCount];
        self->cfgWorklist=ArrayList<SsaBasicBlock>::new();
        self->cfgPhiWorklist=ArrayList<SsaBasicBlock>::new();
        self->executableBlocks=BitSet::new(ssaMeth.getBlocks().size());
        self->ssaWorklist=ArrayList<SsaInsn>::new();
        self->varyingWorklist=ArrayList<SsaInsn>::new();
        self->branchWorklist=ArrayList<SsaInsn>::new();
        for(        let i: i32 = 0;;i<self->regCounti += 1)        {
            self.latticeValues[i]=SCCP::TOP;
            self.latticeConstants[i]=None;
        }
    }
    pub fn process(ssaMethod: &SsaMethod)    {
        SCCP::new(&ssaMethod).run();
    }
    pub fn addBlockToWorklist(&self, ssaBlock: &SsaBasicBlock)    {
        if !self.executableBlocks.get_int(ssaBlock.getIndex())        {
            self.cfgWorklist.add_SsaBasicBlock(&ssaBlock);
            self.executableBlocks.set_int(ssaBlock.getIndex());
        }        else         {
            self.cfgPhiWorklist.add_SsaBasicBlock(&ssaBlock);
        }
    }
    pub fn addUsersToWorklist(&self, reg: i32, latticeValue: i32)    {
        if latticeValue==SCCP::VARYING        {
            for insn in self.ssaMeth.getUseListForRegister(reg)            {
                self.varyingWorklist.add_SsaInsn(&insn);
            }
        }        else         {
            for insn in self.ssaMeth.getUseListForRegister(reg)            {
                self.ssaWorklist.add_SsaInsn(&insn);
            }
        }
    }
    pub fn setLatticeValueTo(&self, reg: i32, value: i32, cst: &Constant) -> boolean    {
        if value!=SCCP::CONSTANT        {
            if self.latticeValues[reg]!=value            {
                self.latticeValues[reg]=value;
                return true;
            }            
            return false;
        }        else         {
            if self.latticeValues[reg]!=value||!self.latticeConstants[reg].equals(&cst)            {
                self.latticeValues[reg]=value;
                self.latticeConstants[reg]=cst;
                return true;
            }            
            return false;
        }
    }
    pub fn simulatePhi(&self, insn: &PhiInsn)    {
        let phiResultReg: i32 = insn.getResult().getReg();
        if self.latticeValues[phiResultReg]==SCCP::VARYING        {
            return;
        }        
        let sources: RegisterSpecList = insn.getSources();
        let phiResultValue: i32 = SCCP::TOP;
        let phiConstant: Constant = None;
        let sourceSize: i32 = sources.size();
        for(        let i: i32 = 0;;i<sourceSizei += 1)        {
            let predBlockIndex: i32 = insn.predBlockIndexForSourcesIndex(i);
            let sourceReg: i32 = sources.get(i).getReg();
            let sourceRegValue: i32 = self.latticeValues[sourceReg];
            if !self.executableBlocks.get_int(predBlockIndex)            {
                continue;
            }            
            if sourceRegValue==SCCP::CONSTANT            {
                if phiConstant==None                {
                    phiConstant=self.latticeConstants[sourceReg];
                    phiResultValue=SCCP::CONSTANT;
                }                else                 if !self.latticeConstants[sourceReg].equals(&phiConstant)                {
                    phiResultValue=SCCP::VARYING;
                    break;
                }                
            }            else             {
                phiResultValue=sourceRegValue;
                break;
            }
        }
        if setLatticeValueTo(phiResultReg, phiResultValue, &phiConstant)        {
            addUsersToWorklist(phiResultReg, phiResultValue);
        }        
    }
    pub fn simulateBlock(&self, block: &SsaBasicBlock)    {
        for insn in block.getInsns()        {
            if //insn instanceof PhiInsn            {
                simulatePhi((PhiInsn*)insn);
            }            else             {
                simulateStmt(&insn);
            }
        }
    }
    pub fn simulatePhiBlock(&self, block: &SsaBasicBlock)    {
        for insn in block.getInsns()        {
            if //insn instanceof PhiInsn            {
                simulatePhi((PhiInsn*)insn);
            }            else             {
                return;
            }
        }
    }
    pub fn latticeValName(latticeVal: i32) -> String    {
        match latticeVal{SCCP::TOP =>             return "TOP";SCCP::CONSTANT =>             return "CONSTANT";SCCP::VARYING =>             return "VARYING";        _ => {}        return "UNKNOWN";    }
}
pub fn simulateBranch(&self, insn: &SsaInsn){
    let opcode: Rop = insn.getOpcode();
    let sources: RegisterSpecList = insn.getSources();
    let constantBranch: boolean = false;
    let constantSuccessor: boolean = false;
    if opcode.getBranchingness()==Rop::BRANCH_IF    {
        let cA: Constant = None;
        let cB: Constant = None;
        let specA: RegisterSpec = sources.get(0);
        let regA: i32 = specA.getReg();
        if !self.ssaMeth.isRegALocal(&specA)&&self.latticeValues[regA]==SCCP::CONSTANT        {
            cA=self.latticeConstants[regA];
        }        
        if sources.size()==2        {
            let specB: RegisterSpec = sources.get(1);
            let regB: i32 = specB.getReg();
            if !self.ssaMeth.isRegALocal(&specB)&&self.latticeValues[regB]==SCCP::CONSTANT            {
                cB=self.latticeConstants[regB];
            }            
        }        
        if cA!=None&&sources.size()==1        {
            match ((TypedConstant*)cA).getBasicType(){Type::BT_INT =>                 constantBranch=true;                let vA: i32 = ((CstInteger*)cA).getValue();                match opcode.getOpcode(){RegOps::IF_EQ =>                     constantSuccessor=(vA==0);                    break;RegOps::IF_NE =>                     constantSuccessor=(vA!=0);                    break;RegOps::IF_LT =>                     constantSuccessor=(vA<0);                    break;RegOps::IF_GE =>                     constantSuccessor=(vA>=0);                    break;RegOps::IF_LE =>                     constantSuccessor=(vA<=0);                    break;RegOps::IF_GT =>                     constantSuccessor=(vA>0);                    break;                _ => {}                throw RuntimeException::new("Unexpected op");            }            break;        _ => {}    }
}else if cA!=None&&cB!=None{
    match ((TypedConstant*)cA).getBasicType(){Type::BT_INT =>         constantBranch=true;        let vA: i32 = ((CstInteger*)cA).getValue();        let vB: i32 = ((CstInteger*)cB).getValue();        match opcode.getOpcode(){RegOps::IF_EQ =>             constantSuccessor=(vA==vB);            break;RegOps::IF_NE =>             constantSuccessor=(vA!=vB);            break;RegOps::IF_LT =>             constantSuccessor=(vA<vB);            break;RegOps::IF_GE =>             constantSuccessor=(vA>=vB);            break;RegOps::IF_LE =>             constantSuccessor=(vA<=vB);            break;RegOps::IF_GT =>             constantSuccessor=(vA>vB);            break;        _ => {}        throw RuntimeException::new("Unexpected op");    }    break;_ => {}}
}
}
let block: SsaBasicBlock = insn.getBlock();
if constantBranch{
let successorBlock: i32;
if constantSuccessor{
successorBlock=block.getSuccessorList().get(1);
}else {
successorBlock=block.getSuccessorList().get(0);
}
addBlockToWorklist(self.ssaMeth.getBlocks().get(successorBlock));
self.branchWorklist.add_SsaInsn(&insn);
}else {
for(let i: i32 = 0;;i<block.getSuccessorList().size()i += 1){
let successorBlock: i32 = block.getSuccessorList().get(i);
addBlockToWorklist(self.ssaMeth.getBlocks().get(successorBlock));
}
}
}
pub fn simulateMath(&self, insn: &SsaInsn, resultType: i32) -> Constant{
let ropInsn: Insn = insn.getOriginalRopInsn();
let opcode: i32 = insn.getOpcode().getOpcode();
let sources: RegisterSpecList = insn.getSources();
let regA: i32 = sources.get(0).getReg();
let cA: Constant;
let cB: Constant;
if self.latticeValues[regA]!=SCCP::CONSTANT{
cA=None;
}else {
cA=self.latticeConstants[regA];
}
if sources.size()==1{
let cstInsn: CstInsn = (CstInsn*)ropInsn;
cB=cstInsn.getConstant();
}else {
let regB: i32 = sources.get(1).getReg();
if self.latticeValues[regB]!=SCCP::CONSTANT{
cB=None;
}else {
cB=self.latticeConstants[regB];
}
}
if cA==None||cB==None{
return None;
}
match resultType{Type::BT_INT => let vR: i32;let skip: boolean = false;let vA: i32 = ((CstInteger*)cA).getValue();let vB: i32 = ((CstInteger*)cB).getValue();match opcode{RegOps::ADD => vR=vA+vB;break;RegOps::SUB => if sources.size()==1{
vR=vB-vA;
}else {
vR=vA-vB;
}break;RegOps::MUL => vR=vA*vB;break;RegOps::DIV => if vB==0{
skip=true;
vR=0;
}else {
vR=vA/vB;
}break;RegOps::AND => vR=vA&vB;break;RegOps::OR => vR=vA|vB;break;RegOps::XOR => vR=vA^vB;break;RegOps::SHL => vR=vA<<vB;break;RegOps::SHR => vR=vA>>vB;break;RegOps::USHR => vR=vA>>>vB;break;RegOps::REM => if vB==0{
skip=true;
vR=0;
}else {
vR=vA%vB;
}break;_ => {}throw RuntimeException::new("Unexpected op");}return if skip { None } else { CstInteger::make(vR) };_ => {}return None;}
}
pub fn simulateStmt(&self, insn: &SsaInsn){
let ropInsn: Insn = insn.getOriginalRopInsn();
if ropInsn.getOpcode().getBranchingness()!=Rop::BRANCH_NONE||ropInsn.getOpcode().isCallLike(){
simulateBranch(&insn);
}
let opcode: i32 = insn.getOpcode().getOpcode();
let result: RegisterSpec = insn.getResult();
if result==None{
if opcode==RegOps::DIV||opcode==RegOps::REM{
let succ: SsaBasicBlock = insn.getBlock().getPrimarySuccessor();
result=succ.getInsns().get(0).getResult();
}else {
return;
}
}
let resultReg: i32 = result.getReg();
let resultValue: i32 = SCCP::VARYING;
let resultConstant: Constant = None;
match opcode{RegOps::CONST => {
let cstInsn: CstInsn = (CstInsn*)ropInsn;
resultValue=SCCP::CONSTANT;
resultConstant=cstInsn.getConstant();
break;
}RegOps::MOVE => {
if insn.getSources().size()==1{
let sourceReg: i32 = insn.getSources().get(0).getReg();
resultValue=self.latticeValues[sourceReg];
resultConstant=self.latticeConstants[sourceReg];
}
break;
}RegOps::ADD => RegOps::SUB => RegOps::MUL => RegOps::DIV => RegOps::AND => RegOps::OR => RegOps::XOR => RegOps::SHL => RegOps::SHR => RegOps::USHR => RegOps::REM => {
resultConstant=simulateMath(&insn, result.getBasicType());
if resultConstant!=None{
resultValue=SCCP::CONSTANT;
}
break;
}RegOps::MOVE_RESULT_PSEUDO => {
if self.latticeValues[resultReg]==SCCP::CONSTANT{
resultValue=self.latticeValues[resultReg];
resultConstant=self.latticeConstants[resultReg];
}
break;
}_ => {}{
}}
if setLatticeValueTo(resultReg, resultValue, &resultConstant){
addUsersToWorklist(resultReg, resultValue);
}
}
pub fn run(&self){
let firstBlock: SsaBasicBlock = self.ssaMeth.getEntryBlock();
addBlockToWorklist(&firstBlock);
while !self.cfgWorklist.isEmpty()||!self.cfgPhiWorklist.isEmpty()||!self.ssaWorklist.isEmpty()||!self.varyingWorklist.isEmpty(){
while !self.cfgWorklist.isEmpty(){
let listSize: i32 = self.cfgWorklist.size()-1;
let block: SsaBasicBlock = self.cfgWorklist.remove_int(listSize);
simulateBlock(&block);
}
while !self.cfgPhiWorklist.isEmpty(){
let listSize: i32 = self.cfgPhiWorklist.size()-1;
let block: SsaBasicBlock = self.cfgPhiWorklist.remove_int(listSize);
simulatePhiBlock(&block);
}
while !self.varyingWorklist.isEmpty(){
let listSize: i32 = self.varyingWorklist.size()-1;
let insn: SsaInsn = self.varyingWorklist.remove_int(listSize);
if !self.executableBlocks.get_int(insn.getBlock().getIndex()){
continue;
}
if //insn instanceof PhiInsn{
simulatePhi((PhiInsn*)insn);
}else {
simulateStmt(&insn);
}
}
while !self.ssaWorklist.isEmpty(){
let listSize: i32 = self.ssaWorklist.size()-1;
let insn: SsaInsn = self.ssaWorklist.remove_int(listSize);
if !self.executableBlocks.get_int(insn.getBlock().getIndex()){
continue;
}
if //insn instanceof PhiInsn{
simulatePhi((PhiInsn*)insn);
}else {
simulateStmt(&insn);
}
}
}
replaceConstants();
replaceBranches();
}
pub fn replaceConstants(&self){
for(let reg: i32 = 0;;reg<self.regCountreg += 1){
if self.latticeValues[reg]!=SCCP::CONSTANT{
continue;
}
if !(//latticeConstants[reg] instanceof TypedConstant){
continue;
}
let defn: SsaInsn = self.ssaMeth.getDefinitionForRegister(reg);
let typeBearer: TypeBearer = defn.getResult().getTypeBearer();
if typeBearer.isConstant(){
continue;
}
let dest: RegisterSpec = defn.getResult();
let newDest: RegisterSpec = dest.withType((TypedConstant*)self.latticeConstants[reg]);
defn.setResult(&newDest);
for insn in self.ssaMeth.getUseListForRegister(reg){
if insn.isPhiOrMove(){
continue;
}
let nInsn: NormalSsaInsn = (NormalSsaInsn*)insn;
let sources: RegisterSpecList = insn.getSources();
let index: i32 = sources.indexOfRegister(reg);
let spec: RegisterSpec = sources.get(index);
let newSpec: RegisterSpec = spec.withType((TypedConstant*)self.latticeConstants[reg]);
nInsn.changeOneSource(index, &newSpec);
}
}
}
pub fn replaceBranches(&self){
for insn in self.branchWorklist{
let oldSuccessor: i32 = -1;
let block: SsaBasicBlock = insn.getBlock();
let successorSize: i32 = block.getSuccessorList().size();
for(let i: i32 = 0;;i<successorSizei += 1){
let successorBlock: i32 = block.getSuccessorList().get(i);
if !self.executableBlocks.get_int(successorBlock){
oldSuccessor=successorBlock;
}
}
if successorSize!=2||oldSuccessor==-1{continue;}
let originalRopInsn: Insn = insn.getOriginalRopInsn();
block.replaceLastInsn(PlainInsn::new(&Rops::GOTO, originalRopInsn.getPosition(), None, &RegisterSpecList::EMPTY));
block.removeSuccessor(oldSuccessor);
}
}
}
