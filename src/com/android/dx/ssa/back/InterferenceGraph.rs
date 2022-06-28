use crate::helper;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::ssa::SetFactory;

struct InterferenceGraph{
    pub interference: ArrayList<IntSet>,
}
impl InterferenceGraph{
    pub fn new(&self, countRegs: i32)    {
        self.interference=ArrayList<IntSet>::new(countRegs);
        for(        let i: i32 = 0;;i<countRegsi += 1)        {
            self.interference.add_IntSet(SetFactory::makeInterferenceSet(countRegs));
        }
    }
    pub fn add(&self, regV: i32, regW: i32)    {
        ensureCapacity(Math::max_int_int(regV, regW)+1);
        self.interference.get(regV).add(regW);
        self.interference.get(regW).add(regV);
    }
    pub fn dumpToStdout(&self)    {
        let oldRegCount: i32 = self.interference.size();
        for(        let i: i32 = 0;;i<oldRegCounti += 1)        {
            let sb: StringBuilder = StringBuilder::new();
            sb.append_String("Reg "+i+":"+self.interference.get(i).toString());
            System::out.println_String(sb.toString());
        }
    }
    pub fn mergeInterferenceSet(&self, reg: i32, set: &IntSet)    {
        if reg<self.interference.size()        {
            set.merge(self.interference.get(reg));
        }        
    }
    pub fn ensureCapacity(&self, size: i32)    {
        let countRegs: i32 = self.interference.size();
        self.interference.ensureCapacity(size);
        for(        let i: i32 = countRegs;;i<sizei += 1)        {
            self.interference.add_IntSet(SetFactory::makeInterferenceSet(size));
        }
    }
}
