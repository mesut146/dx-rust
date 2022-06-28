use crate::helper;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::Item;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::MapItem;
use crate::com::android::dx::dex::file::UniformListItem<com::android::dx::dex::file::MapItem>;

struct MapItem{
    pub type: ItemType,
    pub section: Section,
    pub firstItem: Item,
    pub lastItem: Item,
    pub itemCount: i32,
}
impl MapItem{
    pub const ALIGNMENT: i32 = 4;
    pub const WRITE_SIZE: i32 = (4*3);
    pub fn addMap(sections: &Vec<Section>, mapSection: &MixedItemSection)    {
        if sections==None        {
            throw NullPointerException::new("sections == null");
        }        
        if mapSection.items().size()!=0        {
            throw IllegalArgumentException::new("mapSection.items().size() != 0");
        }        
        let items: ArrayList<MapItem> = ArrayList<MapItem>::new(50);
        for section in sections        {
            let currentType: ItemType = None;
            let firstItem: Item = None;
            let lastItem: Item = None;
            let count: i32 = 0;
            for item in section.items()            {
                let type: ItemType = item.itemType();
                if type_renamed!=currentType                {
                    if count!=0                    {
                        items.add_MapItem(MapItem::new(&currentType, &section, &firstItem, &lastItem, count));
                    }                    
                    currentType=type_renamed;
                    firstItem=item;
                    count=0;
                }                
                lastItem=item;
                count += 1;
            }
            if count!=0            {
                items.add_MapItem(MapItem::new(&currentType, &section, &firstItem, &lastItem, count));
            }            else             if section==mapSection            {
                items.add_MapItem(MapItem::new(&mapSection));
            }            
        }
        mapSection.add(UniformListItem<MapItem>::new(&ItemType::TYPE_MAP_LIST, &items));
    }
    pub fn new(&self, type: &ItemType, section: &Section, firstItem: &Item, lastItem: &Item, itemCount: i32)    {
        super(ALIGNMENT,WRITE_SIZE);

        if type==None        {
            throw NullPointerException::new("type == null");
        }        
        if section==None        {
            throw NullPointerException::new("section == null");
        }        
        if firstItem==None        {
            throw NullPointerException::new("firstItem == null");
        }        
        if lastItem==None        {
            throw NullPointerException::new("lastItem == null");
        }        
        if itemCount<=0        {
            throw IllegalArgumentException::new("itemCount <= 0");
        }        
        self->type=type;
        self->section=section;
        self->firstItem=firstItem;
        self->lastItem=lastItem;
        self->itemCount=itemCount;
    }
    pub fn new(&self, section: &Section)    {
        super(ALIGNMENT,WRITE_SIZE);

        if section==None        {
            throw NullPointerException::new("section == null");
        }        
        self->type=ItemType::TYPE_MAP_LIST;
        self->section=section;
        self->firstItem=None;
        self->lastItem=None;
        self->itemCount=1;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_MAP_ITEM;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_String(getClass().getName());
        sb.append_char('{');
        sb.append_String(self.section.toString());
        sb.append_char(' ');
        sb.append_String(self.type_renamed.toHuman());
        sb.append_char('}');
        return sb.toString();
    }
    pub fn addContents(&self, file: &DexFile)    {
    }
    pub fn toHuman(&self) -> String    {
        return toString();
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let value: i32 = self.type_renamed.getMapValue();
        let offset: i32;
        if self.firstItem==None        {
            offset=self.section.getFileOffset();
        }        else         {
            offset=self.section.getAbsoluteItemOffset(&self.firstItem);
        }
        if out.annotates()        {
            out.annotate_int_String(0, offsetString()+' '+self.type_renamed.getTypeName()+" map");
            out.annotate_int_String(2, "  type:   "+Hex::u2(value)+" // "+self.type_renamed.toString());
            out.annotate_int_String(2, "  unused: 0");
            out.annotate_int_String(4, "  size:   "+Hex::u4(self.itemCount));
            out.annotate_int_String(4, "  offset: "+Hex::u4(offset));
        }        
        out.writeShort(value);
        out.writeShort(0);
        out.writeInt(self.itemCount);
        out.writeInt(offset);
    }
}
