use crate::helper;

trait CatchBuilder{
    pub fn build(&self) -> CatchTable;
    pub fn hasAnyCatches(&self) -> boolean;
    pub fn getCatchTypes(&self) -> HashSet<Type>;
}
