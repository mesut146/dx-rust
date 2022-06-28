use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::ConstCollector;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::rop::cst::TypedConstant;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::PlainInsn;

struct ConstCollector{
    pub ssaMeth: SsaMethod,
}
impl ConstCollector{
    pub const MAX_COLLECTED_CONSTANTS: i32 = 5;
    pub const COLLECT_STRINGS: boolean = false;
    pub const COLLECT_ONE_LOCAL: boolean = false;
    pub fn process(ssaMethod: &SsaMethod)    {
        let cc: ConstCollector = ConstCollector::new(&ssaMethod);
        cc.run();
    }
    pub fn new(&self, ssaMethod: &SsaMethod)    {
        self->ssaMeth=ssaMethod;
    }
    pub fn run(&self)    {
        let regSz: i32 = self.ssaMeth.getRegCount();
        let constantList: ArrayList<TypedConstant> = getConstsSortedByCountUse();
        let toCollect: i32 = Math::min_int_int(constantList.size(), ConstCollector::MAX_COLLECTED_CONSTANTS);
        let start: SsaBasicBlock = self.ssaMeth.getEntryBlock();
        let newRegs: HashMap<TypedConstant,RegisterSpec> = HashMap<TypedConstant,RegisterSpec>::new(toCollect);
        for(        let i: i32 = 0;;i<toCollecti += 1)        {
            let cst: TypedConstant = constantList.get(i);
            let result: RegisterSpec = RegisterSpec::make_int_TypeBearer(self.ssaMeth.makeNewSsaReg(), &cst);
            let constRop: Rop = Rops::opConst(&cst);
            if constRop.getBranchingness()==Rop::BRANCH_NONE            {
                start.addInsnToHead(PlainCstInsn::new(Rops::opConst(&cst), &SourcePosition::NO_INFO, &result, &RegisterSpecList::EMPTY, &cst));
            }            else             {
                let entryBlock: SsaBasicBlock = self.ssaMeth.getEntryBlock();
                let successorBlock: SsaBasicBlock = entryBlock.getPrimarySuccessor();
                let constBlock: SsaBasicBlock = entryBlock.insertNewSuccessor(&successorBlock);
                constBlock.replaceLastInsn(ThrowingCstInsn::new(&constRop, &SourcePosition::NO_INFO, &RegisterSpecList::EMPTY, &StdTypeList::EMPTY, &cst));
                let resultBlock: SsaBasicBlock = constBlock.insertNewSuccessor(&successorBlock);
                let insn: PlainInsn = PlainInsn::new(Rops::opMoveResultPseudo(result.getTypeBearer()), &SourcePosition::NO_INFO, &result, &RegisterSpecList::EMPTY);
                resultBlock.addInsnToHead(&insn);
            }
            newRegs.put(&cst, &result);
        }
        updateConstUses(&newRegs, regSz);
    }
    pub fn getConstsSortedByCountUse(&self) -> ArrayList<TypedConstant>    {
        let regSz: i32 = self.ssaMeth.getRegCount();
        let countUses: HashMap<TypedConstant,Integer> = HashMap<TypedConstant,Integer>::new();
        let usedByLocal: HashSet<TypedConstant> = HashSet<TypedConstant>::new();
        for(        let i: i32 = 0;;i<regSzi += 1)        {
            let insn: SsaInsn = self.ssaMeth.getDefinitionForRegister(i);
            if insn==None||insn.getOpcode()==None{                continue;            }            
            let result: RegisterSpec = insn.getResult();
            let typeBearer: TypeBearer = result.getTypeBearer();
            if !typeBearer.isConstant(){                continue;            }            
            let cst: TypedConstant = (TypedConstant*)typeBearer;
            if insn.getOpcode().getOpcode()==RegOps::MOVE_RESULT_PSEUDO            {
                let pred: i32 = insn.getBlock().getPredecessors().nextSetBit_int(0);
                let predInsns: ArrayList<SsaInsn>;
                predInsns=self.ssaMeth.getBlocks().get(pred).getInsns();
                insn=predInsns.get(predInsns.size()-1);
            }            
            if insn.canThrow()            {
                if !(//cst instanceof CstString)||!ConstCollector::COLLECT_STRINGS                {
                    continue;
                }                
                if insn.getBlock().getSuccessors().cardinality()>1                {
                    continue;
                }                
            }            
            if self.ssaMeth.isRegALocal(&result)            {
                if !ConstCollector::COLLECT_ONE_LOCAL                {
                    continue;
                }                else                 {
                    if usedByLocal.contains(&cst)                    {
                        continue;
                    }                    else                     {
                        usedByLocal.add(&cst);
                    }
                }
            }            
            let has: Integer = countUses.get(&cst);
            if has==None            {
                countUses.put(&cst, 1);
            }            else             {
                countUses.put(&cst, has+1);
            }
        }
        let constantList: ArrayList<TypedConstant> = ArrayList<TypedConstant>::new();
        for entry in countUses.entrySet()        {
            if entry.getValue()>1            {
                constantList.add_TypedConstant(entry.getKey());
            }            
        }
        Collections::sort_List<TypedConstant>_Comparator<? super TypedConstant>(&constantList, /*new Comparator<Constant>(){
  @Override public int compare(  Constant a,  Constant b){
    int ret;
    ret=countUses.get(b) - countUses.get(a);
    if (ret == 0) {
      ret=a.compareTo(b);
    }
    return ret;
  }
  @Override public boolean equals(  Object obj){
    return obj == this;
  }
}
*/);
        return constantList;
    }
    pub fn fixLocalAssignment(&self, origReg: &RegisterSpec, newReg: &RegisterSpec)    {
        for use in self.ssaMeth.getUseListForRegister(origReg.getReg())        {
            let localAssignment: RegisterSpec = use_renamed.getLocalAssignment();
            if localAssignment==None            {
                continue;
            }            
            if use_renamed.getResult()==None            {
                continue;
            }            
            let local: LocalItem = localAssignment.getLocalItem();
            use_renamed.setResultLocal(None);
            newReg=newReg.withLocalItem(&local);
            let newInsn: SsaInsn = SsaInsn::makeFromRop(PlainInsn::new(Rops::opMarkLocal(&newReg), &SourcePosition::NO_INFO, None, RegisterSpecList::make_RegisterSpec(&newReg)), use_renamed.getBlock());
            let insns: ArrayList<SsaInsn> = use_renamed.getBlock().getInsns();
            insns.add_int_SsaInsn(insns.indexOf(&use_renamed)+1, &newInsn);
        }
    }
    pub fn updateConstUses(&self, newRegs: &HashMap<TypedConstant,RegisterSpec>, origRegCount: i32)    {
        let usedByLocal: HashSet<TypedConstant> = HashSet<TypedConstant>::new();
        let useList: Vec<ArrayList<SsaInsn>> = self.ssaMeth.getUseListCopy();
        for(        let i: i32 = 0;;i<origRegCounti += 1)        {
            let insn: SsaInsn = self.ssaMeth.getDefinitionForRegister(i);
            if insn==None            {
                continue;
            }            
            let origReg: RegisterSpec = insn.getResult();
            let typeBearer: TypeBearer = insn.getResult().getTypeBearer();
            if !typeBearer.isConstant(){                continue;            }            
            let cst: TypedConstant = (TypedConstant*)typeBearer;
            let newReg: RegisterSpec = newRegs.get(&cst);
            if newReg==None            {
                continue;
            }            
            if self.ssaMeth.isRegALocal(&origReg)            {
                if !ConstCollector::COLLECT_ONE_LOCAL                {
                    continue;
                }                else                 {
                    if usedByLocal.contains(&cst)                    {
                        continue;
                    }                    else                     {
                        usedByLocal.add(&cst);
                        fixLocalAssignment(&origReg, newRegs.get(&cst));
                    }
                }
            }            
            let mapper: RegisterMapper = /*new RegisterMapper(){
  @Override public int getNewRegisterCount(){
    return ssaMeth.getRegCount();
  }
  @Override public RegisterSpec map(  RegisterSpec registerSpec){
    if (registerSpec.getReg() == origReg.getReg()) {
      return newReg.withLocalItem(registerSpec.getLocalItem());
    }
    return registerSpec;
  }
}
*/;
            for use in useList[origReg.getReg()]            {
                if use_renamed.canThrow()&&use_renamed.getBlock().getSuccessors().cardinality()>1                {
                    continue;
                }                
                use_renamed.mapSourceRegisters(&mapper);
            }
        }
    }
}
