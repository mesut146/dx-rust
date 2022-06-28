use crate::helper;
use crate::com::android::dx::rop::cst::CstArray;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::ValueEncoder;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::dex::file::EncodedArrayItem;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::util::AnnotatedOutput;

struct EncodedArrayItem{
    pub array: CstArray,
    pub encodedForm: Vec<i8>,
}
impl EncodedArrayItem{
    pub const ALIGNMENT: i32 = 1;
    pub fn new(&self, array: &CstArray)    {
        super(ALIGNMENT,-1);

        if array==None        {
            throw NullPointerException::new("array == null");
        }        
        self->array=array;
        self->encodedForm=None;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_ENCODED_ARRAY_ITEM;
    }
    pub fn hashCode(&self) -> i32    {
        return self.array.hashCode();
    }
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        let otherArray: EncodedArrayItem = (EncodedArrayItem*)other;
        return self.array.compareTo(&);
    }
    pub fn toHuman(&self) -> String    {
        return self.array.toHuman();
    }
    pub fn addContents(&self, file: &DexFile)    {
        ValueEncoder::addContents_DexFile_Constant(&file, &self.array);
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new();
        let encoder: ValueEncoder = ValueEncoder::new(addedTo.getFile(), &out);
        encoder.writeArray(&self.array, false);
        self.encodedForm=out.toByteArray();
        setWriteSize(self.encodedForm.len());
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        if annotates        {
            out.annotate_int_String(0, offsetString()+" encoded array");
            let encoder: ValueEncoder = ValueEncoder::new(&file, &out);
            encoder.writeArray(&self.array, true);
        }        else         {
            out.write_byte[](&self.encodedForm);
        }
    }
}
