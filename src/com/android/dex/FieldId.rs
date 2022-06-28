use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dex::FieldId;
use crate::com::android::dex::util::Unsigned;
use crate::com::android::dex::Dex::Section;

struct FieldId{
    pub dex: Dex,
    pub declaringClassIndex: i32,
    pub typeIndex: i32,
    pub nameIndex: i32,
}
impl FieldId{
    pub fn new(&self, dex: &Dex, declaringClassIndex: i32, typeIndex: i32, nameIndex: i32)    {
        self->dex=dex;
        self->declaringClassIndex=declaringClassIndex;
        self->typeIndex=typeIndex;
        self->nameIndex=nameIndex;
    }
    pub fn getDeclaringClassIndex(&self) -> i32    {
        return self.declaringClassIndex;
    }
    pub fn getTypeIndex(&self) -> i32    {
        return self.typeIndex;
    }
    pub fn getNameIndex(&self) -> i32    {
        return self.nameIndex;
    }
    pub fn compareTo(&self, other: &FieldId) -> i32    {
        if self.declaringClassIndex!=        {
            return Unsigned::compare_int_int(self.declaringClassIndex, );
        }        
        if self.nameIndex!=        {
            return Unsigned::compare_int_int(self.nameIndex, );
        }        
        return Unsigned::compare_int_int(self.typeIndex, );
    }
    pub fn writeTo(&self, out: &Dex.Section)    {
        out.writeUnsignedShort(self.declaringClassIndex);
        out.writeUnsignedShort(self.typeIndex);
        out.writeInt(self.nameIndex);
    }
    pub fn toString(&self) -> String    {
        if self.dex==None        {
            return self.declaringClassIndex+" "+self.typeIndex+" "+self.nameIndex;
        }        
        return self.dex.typeNames().get(self.typeIndex)+"."+self.dex.strings().get(self.nameIndex);
    }
}
