use crate::helper;

struct LocalsArray{
}
impl LocalsArray{
    pub fn new(&self, mutable: boolean)    {
        super(mutable);

    }
    pub fn copy(&self) -> LocalsArray;
    pub fn annotate(&self, ex: &ExceptionWithContext);
    pub fn makeInitialized(&self, type: &Type);
    pub fn getMaxLocals(&self) -> i32;
    pub fn set(&self, idx: i32, type: &TypeBearer);
    pub fn set(&self, spec: &RegisterSpec);
    pub fn invalidate(&self, idx: i32);
    pub fn getOrNull(&self, idx: i32) -> TypeBearer;
    pub fn get(&self, idx: i32) -> TypeBearer;
    pub fn getCategory1(&self, idx: i32) -> TypeBearer;
    pub fn getCategory2(&self, idx: i32) -> TypeBearer;
    pub fn merge(&self, other: &LocalsArray) -> LocalsArray;
    pub fn mergeWithSubroutineCaller(&self, other: &LocalsArray, predLabel: i32) -> LocalsArraySet;
    pub fn getPrimary(&self) -> OneLocalsArray;
}
