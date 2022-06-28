use crate::helper;
use crate::T;
use crate::T;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::MixedItemSection::SortType;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::file::OffsettedItem;

let static TYPE_SORTER: Comparator<OffsettedItem> = /*new Comparator<OffsettedItem>(){
  @Override public int compare(  OffsettedItem item1,  OffsettedItem item2){
    ItemType type1=item1.itemType();
    ItemType type2=item2.itemType();
    return type1.compareTo(type2);
  }
}
*/;
struct MixedItemSection{
    pub items: ArrayList<OffsettedItem>,
    pub interns: HashMap<OffsettedItem,OffsettedItem>,
    pub sort: SortType,
    pub writeSize: i32,
}
impl MixedItemSection{
    pub fn new(&self, name: &String, file: &DexFile, alignment: i32, sort: &SortType)    {
        super(name,file,alignment);

        self->items=ArrayList<OffsettedItem>::new(100);
        self->interns=HashMap<OffsettedItem,OffsettedItem>::new(100);
        self->sort=sort;
        self->writeSize=-1;
    }
    pub fn items(&self) -> Collection<? extends Item>    {
        return self.items;
    }
    pub fn writeSize(&self) -> i32    {
        throwIfNotPrepared();
        return self.writeSize;
    }
    pub fn getAbsoluteItemOffset(&self, item: &Item) -> i32    {
        let oi: OffsettedItem = (OffsettedItem*)item;
        return oi.getAbsoluteOffset();
    }
    pub fn size(&self) -> i32    {
        return self.items.size();
    }
    pub fn writeHeaderPart(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        if self.writeSize==-1        {
            throw RuntimeException::new("write size not yet set");
        }        
        let sz: i32 = self.writeSize;
        let offset: i32 = if (sz==0) { 0 } else { getFileOffset() };
                let name: String = getName();
                if name==None                {
                    name="<unnamed>";
                }                
                let spaceCount: i32 = 15-name.length();
                let spaceArr: Vec<char> = new char[spaceCount];
                Arrays::fill_char[]_char(&spaceArr, ' ');
                let spaces: String = String::new(&spaceArr);
                if out.annotates()                {
                    out.annotate_int_String(4, name+"_size:"+spaces+Hex::u4(sz));
                    out.annotate_int_String(4, name+"_off: "+spaces+Hex::u4(offset));
                }                
                out.writeInt(sz);
                out.writeInt(offset);
            }
            pub fn add(&self, item: &OffsettedItem)            {
                throwIfPrepared();
                try                {
                    if item.getAlignment()>getAlignment()                    {
                        throw IllegalArgumentException::new("incompatible item alignment");
                    }                    
                }                catch(                let ex: NullPointerException)                {
                    throw NullPointerException::new("item == null");
                }
                self.items.add_OffsettedItem(&item);
            }
            pub fn intern(&self, item: &T) -> T            {
                throwIfPrepared();
                let result: OffsettedItem = self.interns.get(&item);
                if result!=None                {
                    return (T*)result;
                }                
                add(&item);
                self.interns.put(&item, &item);
                return item;
            }
            pub fn get(&self, item: &T) -> T            {
                throwIfNotPrepared();
                let result: OffsettedItem = self.interns.get(&item);
                if result!=None                {
                    return (T*)result;
                }                
                throw NoSuchElementException::new(item.toString());
            }
            pub fn writeIndexAnnotation(&self, out: &AnnotatedOutput, itemType: &ItemType, intro: &String)            {
                throwIfNotPrepared();
                let index: TreeMap<String,OffsettedItem> = TreeMap<String,OffsettedItem>::new();
                for item in self.items                {
                    if item.itemType()==itemType                    {
                        let label: String = item.toHuman();
                        index.put(&label, &item);
                    }                    
                }
                if index.size()==0                {
                    return;
                }                
                out.annotate_int_String(0, &intro);
                for entry in index.entrySet()                {
                    let label: String = entry.getKey();
                    let item: OffsettedItem = entry.getValue();
                    out.annotate_int_String(0, item.offsetString()+' '+label+'\n');
                }
            }
            pub fn prepare0(&self)            {
                let file: DexFile = getFile();
                let i: i32 = 0;
                for(;)                {
                    let sz: i32 = self.items.size();
                    if i>=sz                    {
                        break;
                    }                    
                    for(;i<szi += 1)                    {
                        let one: OffsettedItem = self.items.get(i);
                        one.addContents(&file);
                    }
                }
            }
            pub fn placeItems(&self)            {
                throwIfNotPrepared();
                match self.sort{SortType::INSTANCE =>                     {
                        Collections::sort_List<OffsettedItem>(&self.items);
                        break;
                    }SortType::TYPE =>                     {
                        Collections::sort_List<OffsettedItem>_Comparator<? super OffsettedItem>(&self.items, &MixedItemSection::TYPE_SORTER);
                        break;
                    }                }
                let sz: i32 = self.items.size();
                let outAt: i32 = 0;
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let one: OffsettedItem = self.items.get(i);
                    try                    {
                        let placedAt: i32 = one.place(self, outAt);
                        if placedAt<outAt                        {
                            throw RuntimeException::new("bogus place() result for "+one);
                        }                        
                        outAt=placedAt+one.writeSize();
                    }                    catch(                    let ex: RuntimeException)                    {
                        throw ExceptionWithContext::withContext(&ex, "...while placing "+one);
                    }
                }
                self.writeSize=outAt;
            }
            pub fn writeTo0(&self, out: &AnnotatedOutput)            {
                let annotates: boolean = out.annotates();
                let first: boolean = true;
                let file: DexFile = getFile();
                let at: i32 = 0;
                for one in self.items                {
                    if annotates                    {
                        if first                        {
                            first=false;
                        }                        else                         {
                            out.annotate_int_String(0, "\n");
                        }
                    }                    
                    let alignMask: i32 = one.getAlignment()-1;
                    let writeAt: i32 = (at+alignMask)&~alignMask;
                    if at!=writeAt                    {
                        out.writeZeroes(writeAt-at);
                        at=writeAt;
                    }                    
                    one.writeTo(&file, &out);
                    at+=one.writeSize();
                }
                if at!=self.writeSize                {
                    throw RuntimeException::new("output size mismatch");
                }                
            }
}
