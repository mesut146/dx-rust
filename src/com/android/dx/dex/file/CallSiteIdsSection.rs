use crate::helper;
use crate::com::android::dx::dex::file::CallSiteIdItem;
use crate::com::android::dx::rop::cst::CstCallSiteRef;

struct CallSiteIdsSection{
    pub callSiteIds: TreeMap<CstCallSiteRef,CallSiteIdItem>,
    pub callSites: TreeMap<CstCallSite,CallSiteItem>,
}
impl CallSiteIdsSection{
    pub fn new(&self, dexFile: &DexFile)    {
        super("call_site_ids",dexFile,4);

    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let result: IndexedItem = self.callSiteIds.get((CstCallSiteRef*)cst);
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn orderItems(&self)    {
        let index: i32 = 0;
        for callSiteId in self.callSiteIds.values()        {
            callSiteId.setIndex(index += 1);
        }
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.callSiteIds.values();
    }
    pub fn intern(&self, cstRef: &CstCallSiteRef)    {
        if cstRef==None        {
            throw NullPointerException::new("cstRef");
        }        
        throwIfPrepared();
        let result: CallSiteIdItem = self.callSiteIds.get(&cstRef);
        if result==None        {
            result=CallSiteIdItem::new(&cstRef);
            self.callSiteIds.put(&cstRef, &result);
        }        
    }
    pub fn addCallSiteItem(&self, callSite: &CstCallSite, callSiteItem: &CallSiteItem)    {
        if callSite==None        {
            throw NullPointerException::new("callSite == null");
        }        
        if callSiteItem==None        {
            throw NullPointerException::new("callSiteItem == null");
        }        
        self.callSites.put(&callSite, &callSiteItem);
    }
    pub fn getCallSiteItem(&self, callSite: &CstCallSite) -> CallSiteItem    {
        if callSite==None        {
            throw NullPointerException::new("callSite == null");
        }        
        return self.callSites.get(&callSite);
    }
}
