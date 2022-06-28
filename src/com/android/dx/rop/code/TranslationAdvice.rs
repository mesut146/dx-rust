use crate::helper;

trait TranslationAdvice{
    pub fn hasConstantOperation(&self, opcode: &Rop, sourceA: &RegisterSpec, sourceB: &RegisterSpec) -> boolean;
    pub fn requiresSourcesInOrder(&self, opcode: &Rop, sources: &RegisterSpecList) -> boolean;
    pub fn getMaxOptimalRegisterCount(&self) -> i32;
}
