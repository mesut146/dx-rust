use crate::helper;
use crate::;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::OffsettedItem;

struct UniformListItem{
    pub itemType: ItemType,
    pub items: List<T>,
}
impl UniformListItem{
    pub const HEADER_SIZE: i32 = 4;
    pub fn new(&self, itemType: &ItemType, items: &List<T>)    {
        super(getAlignment(items),writeSize(items));

        if itemType==None        {
            throw NullPointerException::new("itemType == null");
        }        
        self->items=items;
        self->itemType=itemType;
    }
    pub fn getAlignment(items: &List<? extends OffsettedItem>) -> i32    {
        try        {
            return Math::max_int_int(UniformListItem<T>::HEADER_SIZE, items.get(0).getAlignment());
        }        catch(        let ex: IndexOutOfBoundsException)        {
            throw IllegalArgumentException::new("items.size() == 0");
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("items == null");
        }
    }
    pub fn writeSize(items: &List<? extends OffsettedItem>) -> i32    {
        let first: OffsettedItem = items.get(0);
        return (items.size()*first.writeSize())+UniformListItem<T>::getAlignment(&items);
    }
    pub fn itemType(&self) -> ItemType    {
        return itemType;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String(getClass().getName());
        sb.append_Object(&items);
        return sb.toString();
    }
    pub fn addContents(&self, file: &DexFile)    {
        for i in items        {
            i.addContents(&file);
        }
    }
    pub fn toHuman(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        let first: boolean = true;
        sb.append_String("{");
        for i in items        {
            if first            {
                first=false;
            }            else             {
                sb.append_String(", ");
            }
            sb.append_String(i.toHuman());
        }
        sb.append_String("}");
        return sb.toString();
    }
    pub fn getItems(&self) -> List<T>    {
        return items;
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        offset+=headerSize();
        let first: boolean = true;
        let theSize: i32 = -1;
        let theAlignment: i32 = -1;
        for i in items        {
            let size: i32 = i.writeSize();
            if first            {
                theSize=size;
                theAlignment=i.getAlignment();
                first=false;
            }            else             {
                if size!=theSize                {
                    throw UnsupportedOperationException::new("item size mismatch");
                }                
                if i.getAlignment()!=theAlignment                {
                    throw UnsupportedOperationException::new("item alignment mismatch");
                }                
            }
            offset=i.place(&addedTo, offset)+size;
        }
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let size: i32 = items.size();
        if out.annotates()        {
            out.annotate_int_String(0, offsetString()+" "+typeName());
            out.annotate_int_String(4, "  size: "+Hex::u4(size));
        }        
        out.writeInt(size);
        for i in items        {
            i.writeTo(&file, &out);
        }
    }
    pub fn headerSize(&self) -> i32    {
        return getAlignment();
    }
}
