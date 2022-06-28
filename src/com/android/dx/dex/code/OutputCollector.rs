use crate::helper;
use crate::com::android::dx::dex::code::OutputFinisher;

struct OutputCollector{
    pub finisher: OutputFinisher,
    pub suffix: ArrayList<DalvInsn>,
}
impl OutputCollector{
    pub fn new(&self, dexOptions: &DexOptions, initialCapacity: i32, suffixInitialCapacity: i32, regCount: i32, paramSize: i32)    {
        self->finisher=OutputFinisher::new(&dexOptions, initialCapacity, regCount, paramSize);
        self->suffix=ArrayList<DalvInsn>::new(suffixInitialCapacity);
    }
    pub fn add(&self, insn: &DalvInsn)    {
        self.finisher.add(&insn);
    }
    pub fn get(&self, at: i32) -> DalvInsn    {
        if at>=self.finisher.size()||at<0        {
            return None;
        }        else         {
            return self.finisher.get(at);
        }
    }
    pub fn size(&self) -> i32    {
        return self.finisher.size();
    }
    pub fn reverseBranch(&self, which: i32, newTarget: &CodeAddress)    {
        self.finisher.reverseBranch(which, &newTarget);
    }
    pub fn addSuffix(&self, insn: &DalvInsn)    {
        self.suffix.add_DalvInsn(&insn);
    }
    pub fn getFinisher(&self) -> OutputFinisher    {
        if self.suffix==None        {
            throw UnsupportedOperationException::new("already processed");
        }        
        appendSuffixToOutput();
        return self.finisher;
    }
    pub fn appendSuffixToOutput(&self)    {
        let size: i32 = self.suffix.size();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            self.finisher.add(self.suffix.get(i));
        }
        self.suffix=None;
    }
}
