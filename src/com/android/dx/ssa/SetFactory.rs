use crate::helper;
use crate::com::android::dx::util::ListIntSet;
use crate::com::android::dx::util::BitIntSet;

struct SetFactory{
}
impl SetFactory{
    pub const DOMFRONT_SET_THRESHOLD_SIZE: i32 = 3072;
    pub const INTERFERENCE_SET_THRESHOLD_SIZE: i32 = 3072;
    pub const LIVENESS_SET_THRESHOLD_SIZE: i32 = 3072;
    pub fn makeDomFrontSet(szBlocks: i32) -> IntSet    {
        return if szBlocks<=SetFactory::DOMFRONT_SET_THRESHOLD_SIZE { BitIntSet::new(szBlocks) } else { ListIntSet::new() };
            }
            pub fn makeInterferenceSet(countRegs: i32) -> IntSet            {
                return if countRegs<=SetFactory::INTERFERENCE_SET_THRESHOLD_SIZE { BitIntSet::new(countRegs) } else { ListIntSet::new() };
                    }
                    pub fn makeLivenessSet(countRegs: i32) -> IntSet                    {
                        return if countRegs<=SetFactory::LIVENESS_SET_THRESHOLD_SIZE { BitIntSet::new(countRegs) } else { ListIntSet::new() };
                            }
}
