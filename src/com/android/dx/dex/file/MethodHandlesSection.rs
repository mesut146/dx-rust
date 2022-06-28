use crate::helper;
use crate::com::android::dx::dex::file::MethodHandleItem;
use crate::com::android::dx::rop::cst::CstMethodHandle;

struct MethodHandlesSection{
    pub methodHandles: TreeMap<CstMethodHandle,MethodHandleItem>,
}
impl MethodHandlesSection{
    pub fn new(&self, dexFile: &DexFile)    {
        super("method_handles",dexFile,8);

    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        throwIfNotPrepared();
        let result: IndexedItem = self.methodHandles.get((CstMethodHandle*)cst);
        if result==None        {
            throw IllegalArgumentException::new("not found");
        }        
        return result;
    }
    pub fn orderItems(&self)    {
        let index: i32 = 0;
        for item in self.methodHandles.values()        {
            item.setIndex(index += 1);
        }
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.methodHandles.values();
    }
    pub fn intern(&self, methodHandle: &CstMethodHandle)    {
        if methodHandle==None        {
            throw NullPointerException::new("methodHandle == null");
        }        
        throwIfPrepared();
        let result: MethodHandleItem = self.methodHandles.get(&methodHandle);
        if result==None        {
            result=MethodHandleItem::new(&methodHandle);
            self.methodHandles.put(&methodHandle, &result);
        }        
    }
    pub fn indexOf(&self, cstMethodHandle: &CstMethodHandle) -> i32    {
        return self.methodHandles.get(&cstMethodHandle).getIndex();
    }
}
