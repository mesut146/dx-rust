use crate::helper;
use crate::com::android::dx::io::instructions::AddressMap;

struct BaseCodeCursor{
    pub baseAddressMap: AddressMap,
    pub cursor: i32,
}
impl BaseCodeCursor{
    pub fn new(&self)    {
        self->baseAddressMap=AddressMap::new();
        self->cursor=0;
    }
    pub fn cursor(&self) -> i32    {
        return self.cursor;
    }
    pub fn baseAddressForCursor(&self) -> i32    {
        let mapped: i32 = self.baseAddressMap.get(self.cursor);
        return if (mapped>=0) { mapped } else { self.cursor };
            }
            pub fn setBaseAddress(&self, targetAddress: i32, baseAddress: i32)            {
                self.baseAddressMap.put(targetAddress, baseAddress);
            }
            pub fn advance(&self, amount: i32)            {
                self.cursor+=amount;
            }
}
