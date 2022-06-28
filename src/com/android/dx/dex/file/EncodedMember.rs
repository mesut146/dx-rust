use crate::helper;

struct EncodedMember{
    pub accessFlags: i32,
}
impl EncodedMember{
    pub fn new(&self, accessFlags: i32)    {
        self->accessFlags=accessFlags;
    }
    pub fn getAccessFlags(&self) -> i32    {
        return self.accessFlags;
    }
    pub fn getName(&self) -> CstString;
    pub fn debugPrint(&self, out: &PrintWriter, verbose: boolean);
    pub fn addContents(&self, file: &DexFile);
    pub fn encode(&self, file: &DexFile, out: &AnnotatedOutput, lastIndex: i32, dumpSeq: i32) -> i32;
}
