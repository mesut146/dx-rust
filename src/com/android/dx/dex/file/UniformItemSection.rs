use crate::helper;
use crate::com::android::dx::dex::file::Item;
use crate::;
use crate::com::android::dx::dex::file::IndexedItem;
use crate::com::android::dx::util::AnnotatedOutput;

struct UniformItemSection{
}
impl UniformItemSection{
    pub fn new(&self, name: &String, file: &DexFile, alignment: i32)    {
        super(name,file,alignment);

    }
    pub fn writeSize(&self) -> i32    {
        let items: Collection<? extends Item> = items();
        let sz: i32 = items.size();
        if sz==0        {
            return 0;
        }        
        return sz*items.iterator().next().writeSize();
    }
    pub fn get(&self, cst: &Constant) -> IndexedItem;
    pub fn prepare0(&self)    {
        let file: DexFile = getFile();
        orderItems();
        for one in items()        {
            one.addContents(&file);
        }
    }
    pub fn writeTo0(&self, out: &AnnotatedOutput)    {
        let file: DexFile = getFile();
        let alignment: i32 = getAlignment();
        for one in items()        {
            one.writeTo(&file, &out);
            out.alignTo(alignment);
        }
    }
    pub fn getAbsoluteItemOffset(&self, item: &Item) -> i32    {
        let ii: IndexedItem = (IndexedItem*)item;
        let relativeOffset: i32 = ii.getIndex()*ii.writeSize();
        return getAbsoluteOffset(relativeOffset);
    }
    pub fn orderItems(&self);
}
