use crate::helper;
use crate::com::android::dx::merge::InstructionTransformer::CallSiteVisitor;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::merge::InstructionTransformer::MethodVisitor;
use crate::com::android::dx::io::instructions::ShortArrayCodeOutput;
use crate::com::android::dx::io::CodeReader;
use crate::com::android::dx::merge::IndexMap;
use crate::com::android::dx::merge::InstructionTransformer::TypeVisitor;
use crate::com::android::dx::merge::InstructionTransformer::StringVisitor;
use crate::com::android::dx::io::instructions::DecodedInstruction;
use crate::com::android::dex::DexIndexOverflowException;
use crate::com::android::dx::merge::InstructionTransformer::MethodAndProtoVisitor;
use crate::com::android::dx::merge::InstructionTransformer::GenericVisitor;
use crate::com::android::dx::merge::InstructionTransformer::FieldVisitor;

struct InstructionTransformer{
    pub reader: CodeReader,
    pub mappedInstructions: Vec<DecodedInstruction>,
    pub mappedAt: i32,
    pub indexMap: IndexMap,
}
impl InstructionTransformer{
    pub fn new(&self)    {
        self->reader=CodeReader::new();
        self->reader.setAllVisitors(GenericVisitor::new(, self));
        self->reader.setStringVisitor(StringVisitor::new(, self));
        self->reader.setTypeVisitor(TypeVisitor::new(, self));
        self->reader.setFieldVisitor(FieldVisitor::new(, self));
        self->reader.setMethodVisitor(MethodVisitor::new(, self));
        self->reader.setMethodAndProtoVisitor(MethodAndProtoVisitor::new(, self));
        self->reader.setCallSiteVisitor(CallSiteVisitor::new(, self));
    }
    pub fn transform(&self, indexMap: &IndexMap, encodedInstructions: &Vec<i16>) -> Vec<i16>    {
        let decodedInstructions: Vec<DecodedInstruction> = DecodedInstruction::decodeAll(&encodedInstructions);
        let size: i32 = decodedInstructions.len();
        self->indexMap=indexMap;
        self.mappedInstructions=new DecodedInstruction[size];
        self.mappedAt=0;
        self.reader.visitAll_DecodedInstruction[](&decodedInstructions);
        let out: ShortArrayCodeOutput = ShortArrayCodeOutput::new(size);
        for instruction in self.mappedInstructions        {
            if instruction!=None            {
                instruction.encode(&out);
            }            
        }
        self->indexMap=None;
        return out.getArray();
    }
    pub fn jumboCheck(isJumbo: boolean, newIndex: i32)    {
        if !isJumbo&&(newIndex>0xffff)        {
            throw DexIndexOverflowException::new("Cannot merge new index "+newIndex+" into a non-jumbo instruction!");
        }        
    }
}
