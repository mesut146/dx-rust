use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::util::IntIterator;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::back::InterferenceGraph;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::code::PlainInsn;

struct RegisterAllocator{
    pub ssaMeth: SsaMethod,
    pub interference: InterferenceGraph,
}
impl RegisterAllocator{
    pub fn new(&self, ssaMeth: &SsaMethod, interference: &InterferenceGraph)    {
        self->ssaMeth=ssaMeth;
        self->interference=interference;
    }
    pub fn wantsParamsMovedHigh(&self) -> boolean;
    pub fn allocateRegisters(&self) -> RegisterMapper;
    pub fn getCategoryForSsaReg(&self, reg: i32) -> i32    {
        let definition: SsaInsn = self.ssaMeth.getDefinitionForRegister(reg);
        if definition==None        {
            return 1;
        }        else         {
            return definition.getResult().getCategory();
        }
    }
    pub fn getDefinitionSpecForSsaReg(&self, reg: i32) -> RegisterSpec    {
        let definition: SsaInsn = self.ssaMeth.getDefinitionForRegister(reg);
        return if definition==None { None } else { definition.getResult() };
            }
            pub fn isDefinitionMoveParam(&self, reg: i32) -> boolean            {
                let defInsn: SsaInsn = self.ssaMeth.getDefinitionForRegister(reg);
                if //defInsn instanceof NormalSsaInsn                {
                    let ndefInsn: NormalSsaInsn = (NormalSsaInsn*)defInsn;
                    return ndefInsn.getOpcode().getOpcode()==RegOps::MOVE_PARAM;
                }                
                return false;
            }
            pub fn insertMoveBefore(&self, insn: &SsaInsn, reg: &RegisterSpec) -> RegisterSpec            {
                let block: SsaBasicBlock = insn.getBlock();
                let insns: ArrayList<SsaInsn> = block.getInsns();
                let insnIndex: i32 = insns.indexOf(&insn);
                if insnIndex<0                {
                    throw IllegalArgumentException::new("specified insn is not in this block");
                }                
                if insnIndex!=insns.size()-1                {
                    throw IllegalArgumentException::new("Adding move here not supported:"+insn.toHuman());
                }                
                let newRegSpec: RegisterSpec = RegisterSpec::make_int_TypeBearer(self.ssaMeth.makeNewSsaReg(), reg.getTypeBearer());
                let toAdd: SsaInsn = SsaInsn::makeFromRop(PlainInsn::new(Rops::opMove(newRegSpec.getType()), &SourcePosition::NO_INFO, &newRegSpec, RegisterSpecList::make_RegisterSpec(&reg)), &block);
                insns.add_int_SsaInsn(insnIndex, &toAdd);
                let newReg: i32 = newRegSpec.getReg();
                let liveOut: IntSet = block.getLiveOutRegs();
                let liveOutIter: IntIterator = liveOut.iterator();
                while liveOutIter.hasNext()                {
                    self.interference.add(newReg, liveOutIter.next());
                }
                let sources: RegisterSpecList = insn.getSources();
                let szSources: i32 = sources.size();
                for(                let i: i32 = 0;;i<szSourcesi += 1)                {
                    self.interference.add(newReg, sources.get(i).getReg());
                }
                self.ssaMeth.onInsnsChanged();
                return newRegSpec;
            }
}
