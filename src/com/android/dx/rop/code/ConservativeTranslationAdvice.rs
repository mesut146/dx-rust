use crate::helper;
use crate::com::android::dx::rop::code::ConservativeTranslationAdvice;

let static THE_ONE: ConservativeTranslationAdvice = ConservativeTranslationAdvice::new();
struct ConservativeTranslationAdvice{
}
impl ConservativeTranslationAdvice{
    pub fn new(&self)    {
    }
    pub fn hasConstantOperation(&self, opcode: &Rop, sourceA: &RegisterSpec, sourceB: &RegisterSpec) -> boolean    {
        return false;
    }
    pub fn requiresSourcesInOrder(&self, opcode: &Rop, sources: &RegisterSpecList) -> boolean    {
        return false;
    }
    pub fn getMaxOptimalRegisterCount(&self) -> i32    {
        return Integer::MAX_VALUE;
    }
}
