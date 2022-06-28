use crate::helper;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::DexFile;

struct IdItem{
    pub type: CstType,
}
impl IdItem{
    pub fn new(&self, type: &CstType)    {
        if type==None        {
            throw NullPointerException::new("type == null");
        }        
        self->type=type;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let typeIds: TypeIdsSection = file.getTypeIds();
        typeIds.intern_CstType(&self.type_renamed);
    }
    pub fn getDefiningClass(&self) -> CstType    {
        return self.type_renamed;
    }
}
