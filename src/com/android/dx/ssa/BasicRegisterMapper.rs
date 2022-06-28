use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::util::IntList;

struct BasicRegisterMapper{
    pub oldToNew: IntList,
    pub runningCountNewRegisters: i32,
}
impl BasicRegisterMapper{
    pub fn new(&self, countOldRegisters: i32)    {
        self.oldToNew=IntList::new(countOldRegisters);
    }
    pub fn getNewRegisterCount(&self) -> i32    {
        return self.runningCountNewRegisters;
    }
    pub fn map(&self, registerSpec: &RegisterSpec) -> RegisterSpec    {
        if registerSpec==None        {
            return None;
        }        
        let newReg: i32;
        try        {
            newReg=self.oldToNew.get(registerSpec.getReg());
        }        catch(        let ex: IndexOutOfBoundsException)        {
            newReg=-1;
        }
        if newReg<0        {
            throw RuntimeException::new("no mapping specified for register");
        }        
        return registerSpec.withReg(newReg);
    }
    pub fn oldToNew(&self, oldReg: i32) -> i32    {
        if oldReg>=self.oldToNew.size()        {
            return -1;
        }        
        return self.oldToNew.get(oldReg);
    }
    pub fn toHuman(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        sb.append_String("Old\tNew\n");
        let sz: i32 = self.oldToNew.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            sb.append_int(i);
            sb.append_char('\t');
            sb.append_int(self.oldToNew.get(i));
            sb.append_char('\n');
        }
        sb.append_String("new reg count:");
        sb.append_int(self.runningCountNewRegisters);
        sb.append_char('\n');
        return sb.toString();
    }
    pub fn addMapping(&self, oldReg: i32, newReg: i32, category: i32)    {
        if oldReg>=self.oldToNew.size()        {
            for(            let i: i32 = oldReg-self.oldToNew.size();;i>=0i -= 1)            {
                self.oldToNew.add(-1);
            }
        }        
        self.oldToNew.set(oldReg, newReg);
        if self.runningCountNewRegisters<(newReg+category)        {
            self.runningCountNewRegisters=newReg+category;
        }        
    }
}
