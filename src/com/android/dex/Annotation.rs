use crate::helper;
use crate::com::android::dex::EncodedValue;
use crate::com::android::dex::Dex;
use crate::com::android::dex::Annotation;
use crate::com::android::dex::EncodedValueReader;
use crate::com::android::dex::Dex::Section;

struct Annotation{
    pub dex: Dex,
    pub visibility: i8,
    pub encodedAnnotation: EncodedValue,
}
impl Annotation{
    pub fn new(&self, dex: &Dex, visibility: i8, encodedAnnotation: &EncodedValue)    {
        self->dex=dex;
        self->visibility=visibility;
        self->encodedAnnotation=encodedAnnotation;
    }
    pub fn getVisibility(&self) -> i8    {
        return self.visibility;
    }
    pub fn getReader(&self) -> EncodedValueReader    {
        return EncodedValueReader::new(&self.encodedAnnotation, EncodedValueReader::ENCODED_ANNOTATION);
    }
    pub fn getTypeIndex(&self) -> i32    {
        let reader: EncodedValueReader = getReader();
        reader.readAnnotation();
        return reader.getAnnotationType();
    }
    pub fn writeTo(&self, out: &Dex.Section)    {
        out.writeByte(self.visibility);
        self.encodedAnnotation.writeTo(&out);
    }
    pub fn compareTo(&self, other: &Annotation) -> i32    {
        return self.encodedAnnotation.compareTo(&);
    }
    pub fn toString(&self) -> String    {
        return if self.dex==None { self.visibility+" "+getTypeIndex() } else { self.visibility+" "+self.dex.typeNames().get(getTypeIndex()) };
            }
}
