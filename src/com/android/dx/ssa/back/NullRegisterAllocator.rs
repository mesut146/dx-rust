use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::ssa::BasicRegisterMapper;

struct NullRegisterAllocator{
}
impl NullRegisterAllocator{
    pub fn new(&self, ssaMeth: &SsaMethod, interference: &InterferenceGraph)    {
        super(ssaMeth,interference);

    }
    pub fn wantsParamsMovedHigh(&self) -> boolean    {
        return false;
    }
    pub fn allocateRegisters(&self) -> RegisterMapper    {
        let oldRegCount: i32 = self.super_.ssaMeth.getRegCount();
        let mapper: BasicRegisterMapper = BasicRegisterMapper::new(oldRegCount);
        for(        let i: i32 = 0;;i<oldRegCounti += 1)        {
            mapper.addMapping(i, i*2, 2);
        }
        return mapper;
    }
}
