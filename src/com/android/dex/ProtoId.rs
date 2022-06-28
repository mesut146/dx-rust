use crate::helper;
use crate::com::android::dex::ProtoId;
use crate::com::android::dex::Dex;
use crate::com::android::dex::util::Unsigned;
use crate::com::android::dex::Dex::Section;

struct ProtoId{
    pub dex: Dex,
    pub shortyIndex: i32,
    pub returnTypeIndex: i32,
    pub parametersOffset: i32,
}
impl ProtoId{
    pub fn new(&self, dex: &Dex, shortyIndex: i32, returnTypeIndex: i32, parametersOffset: i32)    {
        self->dex=dex;
        self->shortyIndex=shortyIndex;
        self->returnTypeIndex=returnTypeIndex;
        self->parametersOffset=parametersOffset;
    }
    pub fn compareTo(&self, other: &ProtoId) -> i32    {
        if self.returnTypeIndex!=        {
            return Unsigned::compare_int_int(self.returnTypeIndex, );
        }        
        return Unsigned::compare_int_int(self.parametersOffset, );
    }
    pub fn getShortyIndex(&self) -> i32    {
        return self.shortyIndex;
    }
    pub fn getReturnTypeIndex(&self) -> i32    {
        return self.returnTypeIndex;
    }
    pub fn getParametersOffset(&self) -> i32    {
        return self.parametersOffset;
    }
    pub fn writeTo(&self, out: &Dex.Section)    {
        out.writeInt(self.shortyIndex);
        out.writeInt(self.returnTypeIndex);
        out.writeInt(self.parametersOffset);
    }
    pub fn toString(&self) -> String    {
        if self.dex==None        {
            return self.shortyIndex+" "+self.returnTypeIndex+" "+self.parametersOffset;
        }        
        return self.dex.strings().get(self.shortyIndex)+": "+self.dex.typeNames().get(self.returnTypeIndex)+" "+self.dex.readTypeList(self.parametersOffset);
    }
}
