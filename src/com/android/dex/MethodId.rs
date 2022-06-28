use crate::helper;
use crate::com::android::dex::ProtoId;
use crate::com::android::dex::Dex;
use crate::com::android::dex::MethodId;
use crate::com::android::dex::util::Unsigned;
use crate::com::android::dex::Dex::Section;

struct MethodId{
    pub dex: Dex,
    pub declaringClassIndex: i32,
    pub protoIndex: i32,
    pub nameIndex: i32,
}
impl MethodId{
    pub fn new(&self, dex: &Dex, declaringClassIndex: i32, protoIndex: i32, nameIndex: i32)    {
        self->dex=dex;
        self->declaringClassIndex=declaringClassIndex;
        self->protoIndex=protoIndex;
        self->nameIndex=nameIndex;
    }
    pub fn getDeclaringClassIndex(&self) -> i32    {
        return self.declaringClassIndex;
    }
    pub fn getProtoIndex(&self) -> i32    {
        return self.protoIndex;
    }
    pub fn getNameIndex(&self) -> i32    {
        return self.nameIndex;
    }
    pub fn compareTo(&self, other: &MethodId) -> i32    {
        if self.declaringClassIndex!=        {
            return Unsigned::compare_int_int(self.declaringClassIndex, );
        }        
        if self.nameIndex!=        {
            return Unsigned::compare_int_int(self.nameIndex, );
        }        
        return Unsigned::compare_int_int(self.protoIndex, );
    }
    pub fn writeTo(&self, out: &Dex.Section)    {
        out.writeUnsignedShort(self.declaringClassIndex);
        out.writeUnsignedShort(self.protoIndex);
        out.writeInt(self.nameIndex);
    }
    pub fn toString(&self) -> String    {
        if self.dex==None        {
            return self.declaringClassIndex+" "+self.protoIndex+" "+self.nameIndex;
        }        
        return self.dex.typeNames().get(self.declaringClassIndex)+"."+self.dex.strings().get(self.nameIndex)+self.dex.readTypeList(self.dex.protoIds().get(self.protoIndex).getParametersOffset());
    }
}
