use crate::helper;
use crate::com::android::dx::dex::file::DebugInfoEncoder;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::dex::file::DebugInfoDecoder;

struct DebugInfoItem{
    pub code: DalvCode,
    pub encoded: Vec<i8>,
    pub isStatic: boolean,
    pub ref: CstMethodRef,
}
impl DebugInfoItem{
    pub const ALIGNMENT: i32 = 1;
    pub const ENABLE_ENCODER_SELF_CHECK: boolean = false;
    pub fn new(&self, code: &DalvCode, isStatic: boolean, ref: &CstMethodRef)    {
        super(ALIGNMENT,-1);

        if code==None        {
            throw NullPointerException::new("code == null");
        }        
        self->code=code;
        self->isStatic=isStatic;
        self->ref=ref;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_DEBUG_INFO_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        try        {
            self.encoded=encode(addedTo.getFile(), None, None, None, false);
            setWriteSize(self.encoded.len());
        }        catch(        let ex: RuntimeException)        {
            throw ExceptionWithContext::withContext(&ex, "...while placing debug info for "+self.ref_renamed.toHuman());
        }
    }
    pub fn toHuman(&self) -> String    {
        throw RuntimeException::new("unsupported");
    }
    pub fn annotateTo(&self, file: &DexFile, out: &AnnotatedOutput, prefix: &String)    {
        encode(&file, &prefix, None, &out, false);
    }
    pub fn debugPrint(&self, out: &PrintWriter, prefix: &String)    {
        encode(None, &prefix, &out, None, false);
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        if out.annotates()        {
            out.annotate_String(offsetString()+" debug info");
            encode(&file, None, None, &out, true);
        }        
        out.write_byte[](&self.encoded);
    }
    pub fn encode(&self, file: &DexFile, prefix: &String, debugPrint: &PrintWriter, out: &AnnotatedOutput, consume: boolean) -> Vec<i8>    {
        let result: Vec<i8> = encode0(&file, &prefix, &debugPrint, &out, consume);
        if DebugInfoItem::ENABLE_ENCODER_SELF_CHECK&&(file!=None)        {
            try            {
                DebugInfoDecoder::validateEncode(&result, &file, &self.ref_renamed, &self.code, self.isStatic);
            }            catch(            let ex: RuntimeException)            {
                encode0(&file, "", PrintWriter::new(&System::err, true), None, false);
                throw ex;
            }
        }        
        return result;
    }
    pub fn encode0(&self, file: &DexFile, prefix: &String, debugPrint: &PrintWriter, out: &AnnotatedOutput, consume: boolean) -> Vec<i8>    {
        let positions: PositionList = self.code.getPositions();
        let locals: LocalList = self.code.getLocals();
        let insns: DalvInsnList = self.code.getInsns();
        let codeSize: i32 = insns.codeSize();
        let regSize: i32 = insns.getRegistersSize();
        let encoder: DebugInfoEncoder = DebugInfoEncoder::new(&positions, &locals, &file, codeSize, regSize, self.isStatic, &self.ref_renamed);
        let result: Vec<i8>;
        if (debugPrint==None)&&(out==None)        {
            result=encoder.convert();
        }        else         {
            result=encoder.convertAndAnnotate(&prefix, &debugPrint, &out, consume);
        }
        return result;
    }
}
