use crate::helper;
use crate::com::android::dx::dex::file::HeaderItem;

struct HeaderSection{
    pub list: List<HeaderItem>,
}
impl HeaderSection{
    pub fn new(&self, file: &DexFile)    {
        super(null,file,4);

        let item: HeaderItem = HeaderItem::new();
        item.setIndex(0);
        self->list=Collections::singletonList(&item);
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem    {
        return None;
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.list;
    }
    pub fn orderItems(&self)    {
    }
}
