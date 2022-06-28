use crate::helper;
use crate::com::android::dex::MethodHandle::MethodHandleType;
use crate::com::android::dex::Dex;
use crate::com::android::dex::MethodHandle;
use crate::com::android::dex::util::Unsigned;
use crate::com::android::dex::Dex::Section;

struct MethodHandle{
    pub dex: Dex,
    pub methodHandleType: MethodHandleType,
    pub unused1: i32,
    pub fieldOrMethodId: i32,
    pub unused2: i32,
}
impl MethodHandle{
    pub fn new(&self, dex: &Dex, methodHandleType: &MethodHandleType, unused1: i32, fieldOrMethodId: i32, unused2: i32)    {
        self->dex=dex;
        self->methodHandleType=methodHandleType;
        self->unused1=unused1;
        self->fieldOrMethodId=fieldOrMethodId;
        self->unused2=unused2;
    }
    pub fn compareTo(&self, o: &MethodHandle) -> i32    {
        if self.methodHandleType!=        {
            return self.methodHandleType.compareTo(&);
        }        
        return Unsigned::compare_int_int(self.fieldOrMethodId, );
    }
    pub fn getMethodHandleType(&self) -> MethodHandleType    {
        return self.methodHandleType;
    }
    pub fn getUnused1(&self) -> i32    {
        return self.unused1;
    }
    pub fn getFieldOrMethodId(&self) -> i32    {
        return self.fieldOrMethodId;
    }
    pub fn getUnused2(&self) -> i32    {
        return self.unused2;
    }
    pub fn writeTo(&self, out: &Section)    {
        out.writeUnsignedShort();
        out.writeUnsignedShort(self.unused1);
        out.writeUnsignedShort(self.fieldOrMethodId);
        out.writeUnsignedShort(self.unused2);
    }
    pub fn toString(&self) -> String    {
        if self.dex==None        {
            return self.methodHandleType+" "+self.fieldOrMethodId;
        }        
        return self.methodHandleType+" "+(if self.methodHandleType.isField() { self.dex.fieldIds().get(self.fieldOrMethodId) } else { self.dex.methodIds().get(self.fieldOrMethodId) });
            }
}
