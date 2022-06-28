use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dex::TypeList;

struct ClassDef{
    pub buffer: Dex,
    pub offset: i32,
    pub typeIndex: i32,
    pub accessFlags: i32,
    pub supertypeIndex: i32,
    pub interfacesOffset: i32,
    pub sourceFileIndex: i32,
    pub annotationsOffset: i32,
    pub classDataOffset: i32,
    pub staticValuesOffset: i32,
}
impl ClassDef{
    pub const NO_INDEX: i32 = -1;
    pub fn new(&self, buffer: &Dex, offset: i32, typeIndex: i32, accessFlags: i32, supertypeIndex: i32, interfacesOffset: i32, sourceFileIndex: i32, annotationsOffset: i32, classDataOffset: i32, staticValuesOffset: i32)    {
        self->buffer=buffer;
        self->offset=offset;
        self->typeIndex=typeIndex;
        self->accessFlags=accessFlags;
        self->supertypeIndex=supertypeIndex;
        self->interfacesOffset=interfacesOffset;
        self->sourceFileIndex=sourceFileIndex;
        self->annotationsOffset=annotationsOffset;
        self->classDataOffset=classDataOffset;
        self->staticValuesOffset=staticValuesOffset;
    }
    pub fn getOffset(&self) -> i32    {
        return self.offset;
    }
    pub fn getTypeIndex(&self) -> i32    {
        return self.typeIndex;
    }
    pub fn getSupertypeIndex(&self) -> i32    {
        return self.supertypeIndex;
    }
    pub fn getInterfacesOffset(&self) -> i32    {
        return self.interfacesOffset;
    }
    pub fn getInterfaces(&self) -> Vec<i16>    {
        return self.buffer.readTypeList(self.interfacesOffset).getTypes();
    }
    pub fn getAccessFlags(&self) -> i32    {
        return self.accessFlags;
    }
    pub fn getSourceFileIndex(&self) -> i32    {
        return self.sourceFileIndex;
    }
    pub fn getAnnotationsOffset(&self) -> i32    {
        return self.annotationsOffset;
    }
    pub fn getClassDataOffset(&self) -> i32    {
        return self.classDataOffset;
    }
    pub fn getStaticValuesOffset(&self) -> i32    {
        return self.staticValuesOffset;
    }
    pub fn toString(&self) -> String    {
        if self.buffer==None        {
            return self.typeIndex+" "+self.supertypeIndex;
        }        
        let result: StringBuilder = StringBuilder::new();
        result.append_String(self.buffer.typeNames().get(self.typeIndex));
        if self.supertypeIndex!=ClassDef::NO_INDEX        {
            result.append_String(" extends ").append_String(self.buffer.typeNames().get(self.supertypeIndex));
        }        
        return result.toString();
    }
}
