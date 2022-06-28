use crate::helper;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::CallSiteIdItem;
use crate::com::android::dx::dex::file::CallSiteIdsSection;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::dex::file::CallSiteItem;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;

struct CallSiteIdItem{
    pub invokeDynamicRef: CstCallSiteRef,
    pub data: CallSiteItem,
}
impl CallSiteIdItem{
    pub const ITEM_SIZE: i32 = 4;
    pub fn new(&self, invokeDynamicRef: &CstCallSiteRef)    {
        self->invokeDynamicRef=invokeDynamicRef;
        self->data=None;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_CALL_SITE_ID_ITEM;
    }
    pub fn writeSize(&self) -> i32    {
        return CallSiteIdItem::ITEM_SIZE;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let callSite: CstCallSite = self.invokeDynamicRef.getCallSite();
        let callSiteIds: CallSiteIdsSection = file.getCallSiteIds();
        let callSiteItem: CallSiteItem = callSiteIds.getCallSiteItem(&callSite);
        if callSiteItem==None        {
            let byteData: MixedItemSection = file.getByteData();
            callSiteItem=CallSiteItem::new(&callSite);
            byteData.add(&callSiteItem);
            callSiteIds.addCallSiteItem(&callSite, &callSiteItem);
        }        
        self->data=callSiteItem;
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let offset: i32 = self.data.getAbsoluteOffset();
        if out.annotates()        {
            out.annotate_int_String(0, indexString()+' '+self.invokeDynamicRef.toString());
            out.annotate_int_String(4, "call_site_off: "+Hex::u4(offset));
        }        
        out.writeInt(offset);
    }
    pub fn compareTo(&self, o: &Object) -> i32    {
        let other: CallSiteIdItem = (CallSiteIdItem*)o;
        return self.invokeDynamicRef.compareTo(&);
    }
}
