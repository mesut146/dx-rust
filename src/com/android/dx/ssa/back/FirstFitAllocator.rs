use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::util::BitIntSet;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::rop::code::CstInsn;
use crate::com::android::dx::ssa::BasicRegisterMapper;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::ssa::back::InterferenceGraph;

struct FirstFitAllocator{
    pub mapped: BitSet,
}
impl FirstFitAllocator{
    pub const PRESLOT_PARAMS: boolean = true;
    pub fn new(&self, ssaMeth: &SsaMethod, interference: &InterferenceGraph)    {
        super(ssaMeth,interference);

        self.mapped=BitSet::new(ssaMeth.getRegCount());
    }
    pub fn wantsParamsMovedHigh(&self) -> boolean    {
        return FirstFitAllocator::PRESLOT_PARAMS;
    }
    pub fn allocateRegisters(&self) -> RegisterMapper    {
        let oldRegCount: i32 = self.super_.ssaMeth.getRegCount();
        let mapper: BasicRegisterMapper = BasicRegisterMapper::new(oldRegCount);
        let nextNewRegister: i32 = 0;
        if FirstFitAllocator::PRESLOT_PARAMS        {
            nextNewRegister=self.super_.ssaMeth.getParamWidth();
        }        
        for(        let i: i32 = 0;;i<oldRegCounti += 1)        {
            if self.mapped.get_int(i)            {
                continue;
            }            
            let maxCategory: i32 = getCategoryForSsaReg(i);
            let current: IntSet = BitIntSet::new(oldRegCount);
            self.super_.interference.mergeInterferenceSet(i, &current);
            let isPreslotted: boolean = false;
            let newReg: i32 = 0;
            if FirstFitAllocator::PRESLOT_PARAMS&&isDefinitionMoveParam(i)            {
                let defInsn: NormalSsaInsn = (NormalSsaInsn*)self.super_.ssaMeth.getDefinitionForRegister(i);
                newReg=paramNumberFromMoveParam(&defInsn);
                mapper.addMapping(i, newReg, maxCategory);
                isPreslotted=true;
            }            else             {
                mapper.addMapping(i, nextNewRegister, maxCategory);
                newReg=nextNewRegister;
            }
            for(            let j: i32 = i+1;;j<oldRegCountj += 1)            {
                if self.mapped.get_int(j)||isDefinitionMoveParam(j)                {
                    continue;
                }                
                if !current.has(j)&&!(isPreslotted&&(maxCategory<getCategoryForSsaReg(j)))                {
                    self.super_.interference.mergeInterferenceSet(j, &current);
                    maxCategory=Math::max_int_int(maxCategory, getCategoryForSsaReg(j));
                    mapper.addMapping(j, newReg, maxCategory);
                    self.mapped.set_int(j);
                }                
            }
            self.mapped.set_int(i);
            if !isPreslotted            {
                nextNewRegister+=maxCategory;
            }            
        }
        return mapper;
    }
    pub fn paramNumberFromMoveParam(&self, ndefInsn: &NormalSsaInsn) -> i32    {
        let origInsn: CstInsn = (CstInsn*)ndefInsn.getOriginalRopInsn();
        return ((CstInteger*)origInsn.getConstant()).getValue();
    }
}
