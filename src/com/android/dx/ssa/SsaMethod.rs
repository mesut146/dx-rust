use crate::helper;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::ssa::SsaBasicBlock::Visitor;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::rop::code::RopMethod;

struct SsaMethod{
    pub blocks: ArrayList<SsaBasicBlock>,
    pub entryBlockIndex: i32,
    pub exitBlockIndex: i32,
    pub registerCount: i32,
    pub spareRegisterBase: i32,
    pub borrowedSpareRegisters: i32,
    pub maxLabel: i32,
    pub paramWidth: i32,
    pub isStatic: boolean,
    pub definitionList: Vec<SsaInsn>,
    pub useList: Vec<ArrayList<SsaInsn>>,
    pub unmodifiableUseList: Vec<List<SsaInsn>>,
    pub backMode: boolean,
}
impl SsaMethod{
    pub fn newFromRopMethod(ropMethod: &RopMethod, paramWidth: i32, isStatic: boolean) -> SsaMethod    {
        let result: SsaMethod = SsaMethod::new(&ropMethod, paramWidth, isStatic);
        result.convertRopToSsaBlocks(&ropMethod);
        return result;
    }
    pub fn new(&self, ropMethod: &RopMethod, paramWidth: i32, isStatic: boolean)    {
        self->paramWidth=paramWidth;
        self->isStatic=isStatic;
        self->backMode=false;
        self->maxLabel=ropMethod.getBlocks().getMaxLabel();
        self->registerCount=ropMethod.getBlocks().getRegCount();
        self->spareRegisterBase=self.registerCount;
    }
    pub fn bitSetFromLabelList(blocks: &BasicBlockList, labelList: &IntList) -> BitSet    {
        let result: BitSet = BitSet::new(blocks.size());
        for(        let i: i32 = 0;        let sz: i32 = labelList.size();;i<szi += 1)        {
            result.set_int(blocks.indexOfLabel(labelList.get(i)));
        }
        return result;
    }
    pub fn indexListFromLabelList(ropBlocks: &BasicBlockList, labelList: &IntList) -> IntList    {
        let result: IntList = IntList::new(labelList.size());
        for(        let i: i32 = 0;        let sz: i32 = labelList.size();;i<szi += 1)        {
            result.add(ropBlocks.indexOfLabel(labelList.get(i)));
        }
        return result;
    }
    pub fn convertRopToSsaBlocks(&self, rmeth: &RopMethod)    {
        let ropBlocks: BasicBlockList = rmeth.getBlocks();
        let sz: i32 = ropBlocks.size();
        self.blocks=ArrayList<SsaBasicBlock>::new(sz+2);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let sbb: SsaBasicBlock = SsaBasicBlock::newFromRop(&rmeth, i, self);
            self.blocks.add_SsaBasicBlock(&sbb);
        }
        let origEntryBlockIndex: i32 = rmeth.getBlocks().indexOfLabel(rmeth.getFirstLabel());
        let entryBlock: SsaBasicBlock = self.blocks.get(origEntryBlockIndex).insertNewPredecessor();
        self.entryBlockIndex=entryBlock.getIndex();
        self.exitBlockIndex=-1;
    }
    pub fn makeExitBlock(&self)    {
        if self.exitBlockIndex>=0        {
            throw RuntimeException::new("must be called at most once");
        }        
        self.exitBlockIndex=self.blocks.size();
        let exitBlock: SsaBasicBlock = SsaBasicBlock::new(self.exitBlockIndex, self.maxLabel += 1, self);
        self.blocks.add_SsaBasicBlock(&exitBlock);
        for block in self.blocks        {
            block.exitBlockFixup(&exitBlock);
        }
        if exitBlock.getPredecessors().cardinality()==0        {
            self.blocks.remove_int(self.exitBlockIndex);
            self.exitBlockIndex=-1;
            self.maxLabel -= 1;
        }        
    }
    pub fn getGoto(block: &SsaBasicBlock) -> SsaInsn    {
        return NormalSsaInsn::new(PlainInsn::new(&Rops::GOTO, &SourcePosition::NO_INFO, None, &RegisterSpecList::EMPTY), &block);
    }
    pub fn makeNewGotoBlock(&self) -> SsaBasicBlock    {
        let newIndex: i32 = self.blocks.size();
        let newBlock: SsaBasicBlock = SsaBasicBlock::new(newIndex, self.maxLabel += 1, self);
        newBlock.getInsns().add_SsaInsn(SsaMethod::getGoto(&newBlock));
        self.blocks.add_SsaBasicBlock(&newBlock);
        return newBlock;
    }
    pub fn getEntryBlockIndex(&self) -> i32    {
        return self.entryBlockIndex;
    }
    pub fn getEntryBlock(&self) -> SsaBasicBlock    {
        return self.blocks.get(self.entryBlockIndex);
    }
    pub fn getExitBlockIndex(&self) -> i32    {
        return self.exitBlockIndex;
    }
    pub fn getExitBlock(&self) -> SsaBasicBlock    {
        return if self.exitBlockIndex<0 { None } else { self.blocks.get(self.exitBlockIndex) };
            }
            pub fn blockIndexToRopLabel(&self, bi: i32) -> i32            {
                if bi<0                {
                    return -1;
                }                
                return self.blocks.get(bi).getRopLabel();
            }
            pub fn getRegCount(&self) -> i32            {
                return self.registerCount;
            }
            pub fn getParamWidth(&self) -> i32            {
                return self.paramWidth;
            }
            pub fn isStatic(&self) -> boolean            {
                return self.isStatic;
            }
            pub fn borrowSpareRegister(&self, category: i32) -> i32            {
                let result: i32 = self.spareRegisterBase+self.borrowedSpareRegisters;
                self.borrowedSpareRegisters+=category;
                self.registerCount=Math::max_int_int(self.registerCount, result+category);
                return result;
            }
            pub fn returnSpareRegisters(&self)            {
                self.borrowedSpareRegisters=0;
            }
            pub fn getBlocks(&self) -> ArrayList<SsaBasicBlock>            {
                return self.blocks;
            }
            pub fn computeReachability(&self) -> BitSet            {
                let size: i32 = self.blocks.size();
                let reachableUnvisited: BitSet = BitSet::new(size);
                let reachableVisited: BitSet = BitSet::new(size);
                reachableUnvisited.set_int(getEntryBlock().getIndex());
                let index: i32;
                while (index=reachableUnvisited.nextSetBit_int(0))!=-1                {
                    reachableVisited.set_int(index);
                    reachableUnvisited.or_BitSet(self.blocks.get(index).getSuccessors());
                    reachableUnvisited.andNot(&reachableVisited);
                }
                return reachableVisited;
            }
            pub fn mapRegisters(&self, mapper: &RegisterMapper)            {
                for block in getBlocks()                {
                    for insn in block.getInsns()                    {
                        insn.mapRegisters(&mapper);
                    }
                }
                self.registerCount=mapper.getNewRegisterCount();
                self.spareRegisterBase=self.registerCount;
            }
            pub fn getDefinitionForRegister(&self, reg: i32) -> SsaInsn            {
                if self.backMode                {
                    throw RuntimeException::new("No def list in back mode");
                }                
                if self.definitionList!=None                {
                    return self.definitionList[reg];
                }                
                self.definitionList=new SsaInsn[getRegCount()];
                forEachInsn(/*new SsaInsn.Visitor(){
  @Override public void visitMoveInsn(  NormalSsaInsn insn){
    definitionList[insn.getResult().getReg()]=insn;
  }
  @Override public void visitPhiInsn(  PhiInsn phi){
    definitionList[phi.getResult().getReg()]=phi;
  }
  @Override public void visitNonMoveInsn(  NormalSsaInsn insn){
    RegisterSpec result=insn.getResult();
    if (result != null) {
      definitionList[insn.getResult().getReg()]=insn;
    }
  }
}
*/);
                return self.definitionList[reg];
            }
            pub fn buildUseList(&self)            {
                if self.backMode                {
                    throw RuntimeException::new("No use list in back mode");
                }                
                self.useList=new ArrayList[registerCount];
                for(                let i: i32 = 0;;i<self.registerCounti += 1)                {
                    self.useList[i]=ArrayList::new();
                }
                forEachInsn(/*new SsaInsn.Visitor(){
  /** 
 * {@inheritDoc} 
 */
  @Override public void visitMoveInsn(  NormalSsaInsn insn){
    addToUses(insn);
  }
  /** 
 * {@inheritDoc} 
 */
  @Override public void visitPhiInsn(  PhiInsn phi){
    addToUses(phi);
  }
  /** 
 * {@inheritDoc} 
 */
  @Override public void visitNonMoveInsn(  NormalSsaInsn insn){
    addToUses(insn);
  }
  /** 
 * Adds specified insn to the uses list for all of its sources.
 * @param insn {@code non-null;} insn to process
 */
  private void addToUses(  SsaInsn insn){
    RegisterSpecList rl=insn.getSources();
    int sz=rl.size();
    for (int i=0; i < sz; i++) {
      useList[rl.get(i).getReg()].add(insn);
    }
  }
}
*/);
                self.unmodifiableUseList=new List[registerCount];
                for(                let i: i32 = 0;;i<self.registerCounti += 1)                {
                    self.unmodifiableUseList[i]=Collections::unmodifiableList(self.useList[i]);
                }
            }
            pub fn onSourceChanged(&self, insn: &SsaInsn, oldSource: &RegisterSpec, newSource: &RegisterSpec)            {
                if self.useList==None{                    return;                }                
                if oldSource!=None                {
                    let reg: i32 = oldSource.getReg();
                    self.useList[reg].remove_Object(&insn);
                }                
                let reg: i32 = newSource.getReg();
                if self.useList.len()<=reg                {
                    self.useList=None;
                    return;
                }                
                self.useList[reg].add_SsaInsn(&insn);
            }
            pub fn onSourcesChanged(&self, insn: &SsaInsn, oldSources: &RegisterSpecList)            {
                if self.useList==None{                    return;                }                
                if oldSources!=None                {
                    removeFromUseList(&insn, &oldSources);
                }                
                let sources: RegisterSpecList = insn.getSources();
                let szNew: i32 = sources.size();
                for(                let i: i32 = 0;;i<szNewi += 1)                {
                    let reg: i32 = sources.get(i).getReg();
                    self.useList[reg].add_SsaInsn(&insn);
                }
            }
            pub fn removeFromUseList(&self, insn: &SsaInsn, oldSources: &RegisterSpecList)            {
                if oldSources==None                {
                    return;
                }                
                let szNew: i32 = oldSources.size();
                for(                let i: i32 = 0;;i<szNewi += 1)                {
                    if !self.useList[oldSources.get(i).getReg()].remove_Object(&insn)                    {
                        throw RuntimeException::new("use not found");
                    }                    
                }
            }
            pub fn onInsnAdded(&self, insn: &SsaInsn)            {
                onSourcesChanged(&insn, None);
                updateOneDefinition(&insn, None);
            }
            pub fn onInsnRemoved(&self, insn: &SsaInsn)            {
                if self.useList!=None                {
                    removeFromUseList(&insn, insn.getSources());
                }                
                let resultReg: RegisterSpec = insn.getResult();
                if self.definitionList!=None&&resultReg!=None                {
                    self.definitionList[resultReg.getReg()]=None;
                }                
            }
            pub fn onInsnsChanged(&self)            {
                self.definitionList=None;
                self.useList=None;
                self.unmodifiableUseList=None;
            }
            pub fn updateOneDefinition(&self, insn: &SsaInsn, oldResult: &RegisterSpec)            {
                if self.definitionList==None{                    return;                }                
                if oldResult!=None                {
                    let reg: i32 = oldResult.getReg();
                    self.definitionList[reg]=None;
                }                
                let resultReg: RegisterSpec = insn.getResult();
                if resultReg!=None                {
                    let reg: i32 = resultReg.getReg();
                    if self.definitionList[reg]!=None                    {
                        throw RuntimeException::new("Duplicate add of insn");
                    }                    else                     {
                        self.definitionList[resultReg.getReg()]=insn;
                    }
                }                
            }
            pub fn getUseListForRegister(&self, reg: i32) -> List<SsaInsn>            {
                if self.unmodifiableUseList==None                {
                    buildUseList();
                }                
                return self.unmodifiableUseList[reg];
            }
            pub fn getUseListCopy(&self) -> Vec<ArrayList<SsaInsn>>            {
                if self.useList==None                {
                    buildUseList();
                }                
                let useListCopy: Vec<ArrayList<SsaInsn>> = (Vec<ArrayList<SsaInsn>>*)(new ArrayList[registerCount]);
                for(                let i: i32 = 0;;i<self.registerCounti += 1)                {
                    useListCopy[i]=(ArrayList<SsaInsn>*)(ArrayList::new(self.useList[i]));
                }
                return useListCopy;
            }
            pub fn isRegALocal(&self, spec: &RegisterSpec) -> boolean            {
                let defn: SsaInsn = getDefinitionForRegister(spec.getReg());
                if defn==None                {
                    return false;
                }                
                if defn.getLocalAssignment()!=None{                    return true;                }                
                for use in getUseListForRegister(spec.getReg())                {
                    let insn: Insn = use_renamed.getOriginalRopInsn();
                    if insn!=None&&insn.getOpcode().getOpcode()==RegOps::MARK_LOCAL                    {
                        return true;
                    }                    
                }
                return false;
            }
            pub fn setNewRegCount(&self, newRegCount: i32)            {
                self.registerCount=newRegCount;
                self.spareRegisterBase=self.registerCount;
                onInsnsChanged();
            }
            pub fn makeNewSsaReg(&self) -> i32            {
                let reg: i32 = self.registerCount += 1;
                self.spareRegisterBase=self.registerCount;
                onInsnsChanged();
                return reg;
            }
            pub fn forEachInsn(&self, visitor: &SsaInsn.Visitor)            {
                for block in self.blocks                {
                    block.forEachInsn(&visitor);
                }
            }
            pub fn forEachPhiInsn(&self, v: &PhiInsn.Visitor)            {
                for block in self.blocks                {
                    block.forEachPhiInsn(&v);
                }
            }
            pub fn forEachBlockDepthFirst(&self, reverse: boolean, v: &SsaBasicBlock.Visitor)            {
                let visited: BitSet = BitSet::new(self.blocks.size());
                let stack: Stack<SsaBasicBlock> = Stack<SsaBasicBlock>::new();
                let rootBlock: SsaBasicBlock = if reverse { getExitBlock() } else { getEntryBlock() };
                        if rootBlock==None                        {
                            return;
                        }                        
                        stack.add_SsaBasicBlock(None);
                        stack.add_SsaBasicBlock(&rootBlock);
                        while stack.size()>0                        {
                            let cur: SsaBasicBlock = stack.pop();
                            let parent: SsaBasicBlock = stack.pop();
                            if !visited.get_int(cur.getIndex())                            {
                                let children: BitSet = if reverse { cur.getPredecessors() } else { cur.getSuccessors() };
                                        for(                                        let i: i32 = children.nextSetBit_int(0);;i>=0i=children.nextSetBit_int(i+1))                                        {
                                            stack.add_SsaBasicBlock(&cur);
                                            stack.add_SsaBasicBlock(self.blocks.get(i));
                                        }
                                        visited.set_int(cur.getIndex());
                                        v.visitBlock(&cur, &parent);
                                    }                                    
                                }
                            }
                            pub fn forEachBlockDepthFirstDom(&self, v: &SsaBasicBlock.Visitor)                            {
                                let visited: BitSet = BitSet::new(getBlocks().size());
                                let stack: Stack<SsaBasicBlock> = Stack<SsaBasicBlock>::new();
                                stack.add_SsaBasicBlock(getEntryBlock());
                                while stack.size()>0                                {
                                    let cur: SsaBasicBlock = stack.pop();
                                    let curDomChildren: ArrayList<SsaBasicBlock> = cur.getDomChildren();
                                    if !visited.get_int(cur.getIndex())                                    {
                                        for(                                        let i: i32 = curDomChildren.size()-1;;i>=0i -= 1)                                        {
                                            let child: SsaBasicBlock = curDomChildren.get(i);
                                            stack.add_SsaBasicBlock(&child);
                                        }
                                        visited.set_int(cur.getIndex());
                                        v.visitBlock(&cur, None);
                                    }                                    
                                }
                            }
                            pub fn deleteInsns(&self, deletedInsns: &Set<SsaInsn>)                            {
                                for deletedInsn in deletedInsns                                {
                                    let block: SsaBasicBlock = deletedInsn.getBlock();
                                    let insns: ArrayList<SsaInsn> = block.getInsns();
                                    for(                                    let i: i32 = insns.size()-1;;i>=0i -= 1)                                    {
                                        let insn: SsaInsn = insns.get(i);
                                        if deletedInsn==insn                                        {
                                            onInsnRemoved(&insn);
                                            insns.remove_int(i);
                                            break;
                                        }                                        
                                    }
                                    let insnsSz: i32 = insns.size();
                                    let lastInsn: SsaInsn = if (insnsSz==0) { None } else { insns.get(insnsSz-1) };
                                            if block!=getExitBlock()&&(insnsSz==0||lastInsn.getOriginalRopInsn()==None||lastInsn.getOriginalRopInsn().getOpcode().getBranchingness()==Rop::BRANCH_NONE)                                            {
                                                let gotoInsn: Insn = PlainInsn::new(&Rops::GOTO, &SourcePosition::NO_INFO, None, &RegisterSpecList::EMPTY);
                                                insns.add_SsaInsn(SsaInsn::makeFromRop(&gotoInsn, &block));
                                                let succs: BitSet = block.getSuccessors();
                                                for(                                                let i: i32 = succs.nextSetBit_int(0);;i>=0i=succs.nextSetBit_int(i+1))                                                {
                                                    if i!=block.getPrimarySuccessorIndex()                                                    {
                                                        block.removeSuccessor(i);
                                                    }                                                    
                                                }
                                            }                                            
                                        }
                                    }
                                    pub fn setBackMode(&self)                                    {
                                        self.backMode=true;
                                        self.useList=None;
                                        self.definitionList=None;
                                    }
}
