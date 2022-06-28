use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::rop::code::RegisterSpecList;

struct RegisterMapper{
}
impl RegisterMapper{
    pub fn getNewRegisterCount(&self) -> i32;
    pub fn map(&self, registerSpec: &RegisterSpec) -> RegisterSpec;
    pub fn map(&self, sources: &RegisterSpecList) -> RegisterSpecList    {
        let sz: i32 = sources.size();
        let newSources: RegisterSpecList = RegisterSpecList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            newSources.set_int_RegisterSpec(i, map_RegisterSpec(sources.get(i)));
        }
        newSources.setImmutable();
        return if newSources.equals(&sources) { sources } else { newSources };
            }
            pub fn map(&self, sources: &RegisterSpecSet) -> RegisterSpecSet            {
                let sz: i32 = sources.getMaxSize();
                let newSources: RegisterSpecSet = RegisterSpecSet::new(getNewRegisterCount());
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let registerSpec: RegisterSpec = sources.get_int(i);
                    if registerSpec!=None                    {
                        newSources.put(map_RegisterSpec(&registerSpec));
                    }                    
                }
                newSources.setImmutable();
                return if newSources.equals(&sources) { sources } else { newSources };
                    }
}
