use crate::helper;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::PhiInsn::Visitor;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::ssa::SetFactory;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::ssa::SsaBasicBlock::LabelComparator;
use crate::com::android::dx::rop::code::RopMethod;

let static LABEL_COMPARATOR: Comparator<SsaBasicBlock> = LabelComparator::new();
struct SsaBasicBlock{
    pub insns: ArrayList<SsaInsn>,
    pub predecessors: BitSet,
    pub successors: BitSet,
    pub successorList: IntList,
    pub primarySuccessor: i32,
    pub ropLabel: i32,
    pub parent: SsaMethod,
    pub index: i32,
    pub domChildren: ArrayList<SsaBasicBlock>,
    pub movesFromPhisAtEnd: i32,
    pub movesFromPhisAtBeginning: i32,
    pub liveIn: IntSet,
    pub liveOut: IntSet,
}
impl SsaBasicBlock{
    pub fn new(&self, basicBlockIndex: i32, ropLabel: i32, parent: &SsaMethod)    {
        self->parent=parent;
        self->index=basicBlockIndex;
        self->insns=ArrayList<SsaInsn>::new();
        self->ropLabel=ropLabel;
        self->predecessors=BitSet::new(parent.getBlocks().size());
        self->successors=BitSet::new(parent.getBlocks().size());
        self->successorList=IntList::new();
        self.domChildren=ArrayList<SsaBasicBlock>::new();
    }
    pub fn newFromRop(rmeth: &RopMethod, basicBlockIndex: i32, parent: &SsaMethod) -> SsaBasicBlock    {
        let ropBlocks: BasicBlockList = rmeth.getBlocks();
        let bb: BasicBlock = ropBlocks.get(basicBlockIndex);
        let result: SsaBasicBlock = SsaBasicBlock::new(basicBlockIndex, bb.getLabel(), &parent);
        let ropInsns: InsnList = bb.getInsns();
        .ensureCapacity(ropInsns.size());
        for(        let i: i32 = 0;        let sz: i32 = ropInsns.size();;i<szi += 1)        {
            .add_SsaInsn(NormalSsaInsn::new(ropInsns.get(i), &result));
        }
        =SsaMethod::bitSetFromLabelList(&ropBlocks, rmeth.labelToPredecessors(bb.getLabel()));
        =SsaMethod::bitSetFromLabelList(&ropBlocks, bb.getSuccessors());
        =SsaMethod::indexListFromLabelList(&ropBlocks, bb.getSuccessors());
        if .size()!=0        {
            let primarySuccessor: i32 = bb.getPrimarySuccessor();
            =if (primarySuccessor<0) { -1 } else { ropBlocks.indexOfLabel(primarySuccessor) };
                }                
                return result;
            }
            pub fn addDomChild(&self, child: &SsaBasicBlock)            {
                self.domChildren.add_SsaBasicBlock(&child);
            }
            pub fn getDomChildren(&self) -> ArrayList<SsaBasicBlock>            {
                return self.domChildren;
            }
            pub fn addPhiInsnForReg(&self, reg: i32)            {
                self.insns.add_int_SsaInsn(0, PhiInsn::new(reg, self));
            }
            pub fn addPhiInsnForReg(&self, resultSpec: &RegisterSpec)            {
                self.insns.add_int_SsaInsn(0, PhiInsn::new(&resultSpec, self));
            }
            pub fn addInsnToHead(&self, insn: &Insn)            {
                let newInsn: SsaInsn = SsaInsn::makeFromRop(&insn, self);
                self.insns.add_int_SsaInsn(getCountPhiInsns(), &newInsn);
                self.parent.onInsnAdded(&newInsn);
            }
            pub fn replaceLastInsn(&self, insn: &Insn)            {
                if insn.getOpcode().getBranchingness()==Rop::BRANCH_NONE                {
                    throw IllegalArgumentException::new("last insn must branch");
                }                
                let oldInsn: SsaInsn = self.insns.get(self.insns.size()-1);
                let newInsn: SsaInsn = SsaInsn::makeFromRop(&insn, self);
                self.insns.set(self.insns.size()-1, &newInsn);
                self.parent.onInsnRemoved(&oldInsn);
                self.parent.onInsnAdded(&newInsn);
            }
            pub fn forEachPhiInsn(&self, v: &PhiInsn.Visitor)            {
                let sz: i32 = self.insns.size();
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let insn: SsaInsn = self.insns.get(i);
                    if //insn instanceof PhiInsn                    {
                        v.visitPhiInsn((PhiInsn*)insn);
                    }                    else                     {
                        break;
                    }
                }
            }
            pub fn removeAllPhiInsns(&self)            {
                self.insns.subList(0, getCountPhiInsns()).clear();
            }
            pub fn getCountPhiInsns(&self) -> i32            {
                let countPhiInsns: i32;
                let sz: i32 = self.insns.size();
                for(countPhiInsns=0;countPhiInsns<szcountPhiInsns += 1)                {
                    let insn: SsaInsn = self.insns.get(countPhiInsns);
                    if !(//insn instanceof PhiInsn)                    {
                        break;
                    }                    
                }
                return countPhiInsns;
            }
            pub fn getInsns(&self) -> ArrayList<SsaInsn>            {
                return self.insns;
            }
            pub fn getPhiInsns(&self) -> List<SsaInsn>            {
                return self.insns.subList(0, getCountPhiInsns());
            }
            pub fn getIndex(&self) -> i32            {
                return self.index;
            }
            pub fn getRopLabel(&self) -> i32            {
                return self.ropLabel;
            }
            pub fn getRopLabelString(&self) -> String            {
                return Hex::u2(self.ropLabel);
            }
            pub fn getPredecessors(&self) -> BitSet            {
                return self.predecessors;
            }
            pub fn getSuccessors(&self) -> BitSet            {
                return self.successors;
            }
            pub fn getSuccessorList(&self) -> IntList            {
                return self.successorList;
            }
            pub fn getPrimarySuccessorIndex(&self) -> i32            {
                return self.primarySuccessor;
            }
            pub fn getPrimarySuccessorRopLabel(&self) -> i32            {
                return self.parent.blockIndexToRopLabel(self.primarySuccessor);
            }
            pub fn getPrimarySuccessor(&self) -> SsaBasicBlock            {
                if self.primarySuccessor<0                {
                    return None;
                }                else                 {
                    return self.parent.getBlocks().get(self.primarySuccessor);
                }
            }
            pub fn getRopLabelSuccessorList(&self) -> IntList            {
                let result: IntList = IntList::new(self.successorList.size());
                let sz: i32 = self.successorList.size();
                for(                let i: i32 = 0;;i<szi += 1)                {
                    result.add(self.parent.blockIndexToRopLabel(self.successorList.get(i)));
                }
                return result;
            }
            pub fn getParent(&self) -> SsaMethod            {
                return self.parent;
            }
            pub fn insertNewPredecessor(&self) -> SsaBasicBlock            {
                let newPred: SsaBasicBlock = self.parent.makeNewGotoBlock();
                =self.predecessors;
                .set_int(self.index);
                .add(self.index);
                =self.index;
                self.predecessors=BitSet::new(self.parent.getBlocks().size());
                self.predecessors.set_int();
                for(                let i: i32 = .nextSetBit_int(0);;i>=0i=.nextSetBit_int(i+1))                {
                    let predBlock: SsaBasicBlock = self.parent.getBlocks().get(i);
                    predBlock.replaceSuccessor(self.index, );
                }
                return newPred;
            }
            pub fn insertNewSuccessor(&self, other: &SsaBasicBlock) -> SsaBasicBlock            {
                let newSucc: SsaBasicBlock = self.parent.makeNewGotoBlock();
                if !self.successors.get_int()                {
                    throw RuntimeException::new("Block "+other.getRopLabelString()+" not successor of "+getRopLabelString());
                }                
                .set_int(self->index);
                .set_int();
                .add();
                =;
                for(                let i: i32 = self.successorList.size()-1;;i>=0i -= 1)                {
                    if self.successorList.get(i)==                    {
                        self.successorList.set(i, );
                    }                    
                }
                if self.primarySuccessor==                {
                    self.primarySuccessor=;
                }                
                self.successors.clear_int();
                self.successors.set_int();
                .set_int();
                .set_int_boolean(self.index, self.successors.get_int());
                return newSucc;
            }
            pub fn replaceSuccessor(&self, oldIndex: i32, newIndex: i32)            {
                if oldIndex==newIndex                {
                    return;
                }                
                self.successors.set_int(newIndex);
                if self.primarySuccessor==oldIndex                {
                    self.primarySuccessor=newIndex;
                }                
                for(                let i: i32 = self.successorList.size()-1;;i>=0i -= 1)                {
                    if self.successorList.get(i)==oldIndex                    {
                        self.successorList.set(i, newIndex);
                    }                    
                }
                self.successors.clear_int(oldIndex);
                self.parent.getBlocks().get(newIndex)->predecessors.set_int(self.index);
                self.parent.getBlocks().get(oldIndex)->predecessors.clear_int(self.index);
            }
            pub fn removeSuccessor(&self, oldIndex: i32)            {
                let removeIndex: i32 = 0;
                for(                let i: i32 = self.successorList.size()-1;;i>=0i -= 1)                {
                    if self.successorList.get(i)==oldIndex                    {
                        removeIndex=i;
                    }                    else                     {
                        self.primarySuccessor=self.successorList.get(i);
                    }
                }
                self.successorList.removeIndex(removeIndex);
                self.successors.clear_int(oldIndex);
                self.parent.getBlocks().get(oldIndex)->predecessors.clear_int(self.index);
            }
            pub fn exitBlockFixup(&self, exitBlock: &SsaBasicBlock)            {
                if self==exitBlock                {
                    return;
                }                
                if self.successorList.size()==0                {
                    self.successors.set_int();
                    self.successorList.add();
                    self.primarySuccessor=;
                    .set_int(self->index);
                }                
            }
            pub fn addMoveToEnd(&self, result: &RegisterSpec, source: &RegisterSpec)            {
                if self.successors.cardinality()>1                {
                    throw IllegalStateException::new("Inserting a move to a block with multiple successors");
                }                
                if result.getReg()==source.getReg()                {
                    return;
                }                
                let lastInsn: NormalSsaInsn;
                lastInsn=(NormalSsaInsn*)self.insns.get(self.insns.size()-1);
                if lastInsn.getResult()!=None||lastInsn.getSources().size()>0                {
                    for(                    let i: i32 = self.successors.nextSetBit_int(0);;i>=0i=self.successors.nextSetBit_int(i+1))                    {
                        let succ: SsaBasicBlock;
                        succ=self.parent.getBlocks().get(i);
                        succ.addMoveToBeginning(&result, &source);
                    }
                }                else                 {
                    let sources: RegisterSpecList = RegisterSpecList::make_RegisterSpec(&source);
                    let toAdd: NormalSsaInsn = NormalSsaInsn::new(PlainInsn::new(Rops::opMove(result.getType()), &SourcePosition::NO_INFO, &result, &sources), self);
                    self.insns.add_int_SsaInsn(self.insns.size()-1, &toAdd);
                    self.movesFromPhisAtEnd += 1;
                }
            }
            pub fn addMoveToBeginning(&self, result: &RegisterSpec, source: &RegisterSpec)            {
                if result.getReg()==source.getReg()                {
                    return;
                }                
                let sources: RegisterSpecList = RegisterSpecList::make_RegisterSpec(&source);
                let toAdd: NormalSsaInsn = NormalSsaInsn::new(PlainInsn::new(Rops::opMove(result.getType()), &SourcePosition::NO_INFO, &result, &sources), self);
                self.insns.add_int_SsaInsn(getCountPhiInsns(), &toAdd);
                self.movesFromPhisAtBeginning += 1;
            }
            pub fn setRegsUsed(regsUsed: &BitSet, rs: &RegisterSpec)            {
                regsUsed.set_int(rs.getReg());
                if rs.getCategory()>1                {
                    regsUsed.set_int(rs.getReg()+1);
                }                
            }
            pub fn checkRegUsed(regsUsed: &BitSet, rs: &RegisterSpec) -> boolean            {
                let reg: i32 = rs.getReg();
                let category: i32 = rs.getCategory();
                return regsUsed.get_int(reg)||(if category==2 { regsUsed.get_int(reg+1) } else { false });
                    }
                    pub fn scheduleUseBeforeAssigned(&self, toSchedule: &List<SsaInsn>)                    {
                        let regsUsedAsSources: BitSet = BitSet::new(self.parent.getRegCount());
                        let regsUsedAsResults: BitSet = BitSet::new(self.parent.getRegCount());
                        let sz: i32 = toSchedule.size();
                        let insertPlace: i32 = 0;
                        while insertPlace<sz                        {
                            let oldInsertPlace: i32 = insertPlace;
                            for(                            let i: i32 = insertPlace;;i<szi += 1)                            {
                                SsaBasicBlock::setRegsUsed(&regsUsedAsSources, toSchedule.get(i).getSources().get(0));
                                SsaBasicBlock::setRegsUsed(&regsUsedAsResults, toSchedule.get(i).getResult());
                            }
                            for(                            let i: i32 = insertPlace;;i<szi += 1)                            {
                                let insn: SsaInsn = toSchedule.get(i);
                                if !SsaBasicBlock::checkRegUsed(&regsUsedAsSources, insn.getResult())                                {
                                    Collections::swap_List<?>_int_int(&toSchedule, i, insertPlace += 1);
                                }                                
                            }
                            if oldInsertPlace==insertPlace                            {
                                let insnToSplit: SsaInsn = None;
                                for(                                let i: i32 = insertPlace;;i<szi += 1)                                {
                                    let insn: SsaInsn = toSchedule.get(i);
                                    if SsaBasicBlock::checkRegUsed(&regsUsedAsSources, insn.getResult())&&SsaBasicBlock::checkRegUsed(&regsUsedAsResults, insn.getSources().get(0))                                    {
                                        insnToSplit=insn;
                                        Collections::swap_List<?>_int_int(&toSchedule, insertPlace, i);
                                        break;
                                    }                                    
                                }
                                let result: RegisterSpec = insnToSplit.getResult();
                                let tempSpec: RegisterSpec = result.withReg(self.parent.borrowSpareRegister(result.getCategory()));
                                let toAdd: NormalSsaInsn = NormalSsaInsn::new(PlainInsn::new(Rops::opMove(result.getType()), &SourcePosition::NO_INFO, &tempSpec, insnToSplit.getSources()), self);
                                toSchedule.add_int_SsaInsn(insertPlace += 1, &toAdd);
                                let newSources: RegisterSpecList = RegisterSpecList::make_RegisterSpec(&tempSpec);
                                let toReplace: NormalSsaInsn = NormalSsaInsn::new(PlainInsn::new(Rops::opMove(result.getType()), &SourcePosition::NO_INFO, &result, &newSources), self);
                                toSchedule.set(insertPlace, &toReplace);
                                sz=toSchedule.size();
                            }                            
                            regsUsedAsSources.clear();
                            regsUsedAsResults.clear();
                        }
                    }
                    pub fn addLiveOut(&self, regV: i32)                    {
                        if self.liveOut==None                        {
                            self.liveOut=SetFactory::makeLivenessSet(self.parent.getRegCount());
                        }                        
                        self.liveOut.add(regV);
                    }
                    pub fn addLiveIn(&self, regV: i32)                    {
                        if self.liveIn==None                        {
                            self.liveIn=SetFactory::makeLivenessSet(self.parent.getRegCount());
                        }                        
                        self.liveIn.add(regV);
                    }
                    pub fn getLiveInRegs(&self) -> IntSet                    {
                        if self.liveIn==None                        {
                            self.liveIn=SetFactory::makeLivenessSet(self.parent.getRegCount());
                        }                        
                        return self.liveIn;
                    }
                    pub fn getLiveOutRegs(&self) -> IntSet                    {
                        if self.liveOut==None                        {
                            self.liveOut=SetFactory::makeLivenessSet(self.parent.getRegCount());
                        }                        
                        return self.liveOut;
                    }
                    pub fn isExitBlock(&self) -> boolean                    {
                        return self.index==self.parent.getExitBlockIndex();
                    }
                    pub fn scheduleMovesFromPhis(&self)                    {
                        if self.movesFromPhisAtBeginning>1                        {
                            let toSchedule: List<SsaInsn>;
                            toSchedule=self.insns.subList(0, self.movesFromPhisAtBeginning);
                            scheduleUseBeforeAssigned(&toSchedule);
                            let firstNonPhiMoveInsn: SsaInsn = self.insns.get(self.movesFromPhisAtBeginning);
                            if firstNonPhiMoveInsn.isMoveException()                            {
                                if true                                {
                                    throw RuntimeException::new("Unexpected: moves from "+"phis before move-exception");
                                }                                else                                 {
                                    let moveExceptionInterferes: boolean = false;
                                    let moveExceptionResult: i32 = firstNonPhiMoveInsn.getResult().getReg();
                                    for insn in toSchedule                                    {
                                        if insn.isResultReg(moveExceptionResult)||insn.isRegASource(moveExceptionResult)                                        {
                                            moveExceptionInterferes=true;
                                            break;
                                        }                                        
                                    }
                                    if !moveExceptionInterferes                                    {
                                        self.insns.remove_int(self.movesFromPhisAtBeginning);
                                        self.insns.add_int_SsaInsn(0, &firstNonPhiMoveInsn);
                                    }                                    else                                     {
                                        let originalResultSpec: RegisterSpec = firstNonPhiMoveInsn.getResult();
                                        let spareRegister: i32 = self.parent.borrowSpareRegister(originalResultSpec.getCategory());
                                        firstNonPhiMoveInsn.changeResultReg(spareRegister);
                                        let tempSpec: RegisterSpec = firstNonPhiMoveInsn.getResult();
                                        self.insns.add_int_SsaInsn(0, &firstNonPhiMoveInsn);
                                        let toAdd: NormalSsaInsn = NormalSsaInsn::new(PlainInsn::new(Rops::opMove(tempSpec.getType()), &SourcePosition::NO_INFO, &originalResultSpec, RegisterSpecList::make_RegisterSpec(&tempSpec)), self);
                                        self.insns.set(self.movesFromPhisAtBeginning+1, &toAdd);
                                    }
                                }
                            }                            
                        }                        
                        if self.movesFromPhisAtEnd>1                        {
                            scheduleUseBeforeAssigned(self.insns.subList(self.insns.size()-self.movesFromPhisAtEnd-1, self.insns.size()-1));
                        }                        
                        self.parent.returnSpareRegisters();
                    }
                    pub fn forEachInsn(&self, visitor: &SsaInsn.Visitor)                    {
                        let len: i32 = self.insns.size();
                        for(                        let i: i32 = 0;;i<leni += 1)                        {
                            self.insns.get(i).accept(&visitor);
                        }
                    }
                    pub fn toString(&self) -> String                    {
                        return "{"+self.index+":"+Hex::u2(self.ropLabel)+'}';
                    }
}
