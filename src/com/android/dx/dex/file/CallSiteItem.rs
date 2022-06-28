use crate::helper;
use crate::com::android::dx::dex::file::ValueEncoder;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::rop::cst::CstCallSite;

struct CallSiteItem{
    pub value: CstCallSite,
    pub encodedForm: Vec<i8>,
}
impl CallSiteItem{
    pub fn new(&self, value: &CstCallSite)    {
        super(1,writeSize(value));

        self->value=value;
    }
    pub fn writeSize(value: &CstCallSite) -> i32    {
        return -1;
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new();
        let encoder: ValueEncoder = ValueEncoder::new(addedTo.getFile(), &out);
        encoder.writeArray(&self.value, true);
        self.encodedForm=out.toByteArray();
        setWriteSize(self.encodedForm.len());
    }
    pub fn toHuman(&self) -> String    {
        return self.value.toHuman();
    }
    pub fn toString(&self) -> String    {
        return self.value.toString();
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        if out.annotates()        {
            out.annotate_int_String(0, offsetString()+" call site");
            let encoder: ValueEncoder = ValueEncoder::new(&file, &out);
            encoder.writeArray(&self.value, true);
        }        else         {
            out.write_byte[](&self.encodedForm);
        }
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_ENCODED_ARRAY_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        ValueEncoder::addContents_DexFile_Constant(&file, &self.value);
    }
}
