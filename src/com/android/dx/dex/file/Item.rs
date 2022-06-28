use crate::helper;
use crate::com::android::dx::dex::file::ItemType;

struct Item{
}
impl Item{
    pub fn new(&self)    {
    }
    pub fn itemType(&self) -> ItemType;
    pub fn typeName(&self) -> String    {
        return itemType().toHuman();
    }
    pub fn writeSize(&self) -> i32;
    pub fn addContents(&self, file: &DexFile);
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput);
}
