use crate::helper;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::file::TypeListItem;

struct TypeListItem{
    pub list: TypeList,
}
impl TypeListItem{
    pub const ALIGNMENT: i32 = 4;
    pub const ELEMENT_SIZE: i32 = 2;
    pub const HEADER_SIZE: i32 = 4;
    pub fn new(&self, list: &TypeList)    {
        super(ALIGNMENT,(list.size() * ELEMENT_SIZE) + HEADER_SIZE);

        self->list=list;
    }
    pub fn hashCode(&self) -> i32    {
        return StdTypeList::hashContents(&self.list);
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_TYPE_LIST;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let typeIds: TypeIdsSection = file.getTypeIds();
        let sz: i32 = self.list.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            typeIds.intern_Type(self.list.getType(i));
        }
    }
    pub fn toHuman(&self) -> String    {
        throw RuntimeException::new("unsupported");
    }
    pub fn getList(&self) -> TypeList    {
        return self.list;
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let typeIds: TypeIdsSection = file.getTypeIds();
        let sz: i32 = self.list.size();
        if out.annotates()        {
            out.annotate_int_String(0, offsetString()+" type_list");
            out.annotate_int_String(TypeListItem::HEADER_SIZE, "  size: "+Hex::u4(sz));
            for(            let i: i32 = 0;;i<szi += 1)            {
                let one: Type = self.list.getType(i);
                let idx: i32 = typeIds.indexOf_Type(&one);
                out.annotate_int_String(TypeListItem::ELEMENT_SIZE, "  "+Hex::u2(idx)+" // "+one.toHuman());
            }
        }        
        out.writeInt(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            out.writeShort(typeIds.indexOf_Type(self.list.getType(i)));
        }
    }
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        let thisList: TypeList = self->list;
        let otherList: TypeList = ((TypeListItem*)other)->list;
        return StdTypeList::compareContents(&thisList, &otherList);
    }
}
