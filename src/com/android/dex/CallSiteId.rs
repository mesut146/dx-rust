use crate::helper;
use crate::com::android::dex::CallSiteId;
use crate::com::android::dex::ProtoId;
use crate::com::android::dex::Dex;
use crate::com::android::dex::util::Unsigned;
use crate::com::android::dex::Dex::Section;

struct CallSiteId{
    pub dex: Dex,
    pub offset: i32,
}
impl CallSiteId{
    pub fn new(&self, dex: &Dex, offset: i32)    {
        self->dex=dex;
        self->offset=offset;
    }
    pub fn compareTo(&self, o: &CallSiteId) -> i32    {
        return Unsigned::compare_int_int(self.offset, );
    }
    pub fn getCallSiteOffset(&self) -> i32    {
        return self.offset;
    }
    pub fn writeTo(&self, out: &Section)    {
        out.writeInt(self.offset);
    }
    pub fn toString(&self) -> String    {
        if self.dex==None        {
            return String::valueOf_int(self.offset);
        }        
        return self.dex.protoIds().get(self.offset).toString();
    }
}
