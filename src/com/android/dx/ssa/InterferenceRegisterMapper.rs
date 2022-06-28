use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::util::BitIntSet;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::back::InterferenceGraph;

struct InterferenceRegisterMapper{
    pub newRegInterference: ArrayList<BitIntSet>,
    pub oldRegInterference: InterferenceGraph,
}
impl InterferenceRegisterMapper{
    pub fn new(&self, oldRegInterference: &InterferenceGraph, countOldRegisters: i32)    {
        super(countOldRegisters);

        self.newRegInterference=ArrayList<BitIntSet>::new();
        self->oldRegInterference=oldRegInterference;
    }
    pub fn addMapping(&self, oldReg: i32, newReg: i32, category: i32)    {
        super.addMapping(oldReg,newReg,category);
        addInterfence(newReg, oldReg);
        if category==2        {
            addInterfence(newReg+1, oldReg);
        }        
    }
    pub fn interferes(&self, oldReg: i32, newReg: i32, category: i32) -> boolean    {
        if newReg>=self.newRegInterference.size()        {
            return false;
        }        else         {
            let existing: IntSet = self.newRegInterference.get(newReg);
            if existing==None            {
                return false;
            }            else             if category==1            {
                return existing.has(oldReg);
            }            else             {
                return existing.has(oldReg)||(interferes_int_int_int(oldReg, newReg+1, category-1));
            }
        }
    }
    pub fn interferes(&self, oldSpec: &RegisterSpec, newReg: i32) -> boolean    {
        return interferes_int_int_int(oldSpec.getReg(), newReg, oldSpec.getCategory());
    }
    pub fn addInterfence(&self, newReg: i32, oldReg: i32)    {
        self.newRegInterference.ensureCapacity(newReg+1);
        while newReg>=self.newRegInterference.size()        {
            self.newRegInterference.add_BitIntSet(BitIntSet::new(newReg+1));
        }
        self.oldRegInterference.mergeInterferenceSet(oldReg, self.newRegInterference.get(newReg));
    }
    pub fn areAnyPinned(&self, oldSpecs: &RegisterSpecList, newReg: i32, targetCategory: i32) -> boolean    {
        let sz: i32 = oldSpecs.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let oldSpec: RegisterSpec = oldSpecs.get(i);
            let r: i32 = oldToNew(oldSpec.getReg());
            if r==newReg||(oldSpec.getCategory()==2&&(r+1)==newReg)||(targetCategory==2&&(r==newReg+1))            {
                return true;
            }            
        }
        return false;
    }
}
