use crate::helper;
use crate::com::android::dx::io::instructions::DecodedInstruction;
use crate::com::android::dx::io::OpcodeInfo;
use crate::com::android::dx::io::CodeReader::Visitor;

struct CodeReader{
    pub fallbackVisitor: Visitor,
    pub stringVisitor: Visitor,
    pub typeVisitor: Visitor,
    pub fieldVisitor: Visitor,
    pub methodVisitor: Visitor,
    pub methodAndProtoVisitor: Visitor,
    pub callSiteVisitor: Visitor,
}
impl CodeReader{
    pub fn setAllVisitors(&self, visitor: &Visitor)    {
        self.fallbackVisitor=visitor;
        self.stringVisitor=visitor;
        self.typeVisitor=visitor;
        self.fieldVisitor=visitor;
        self.methodVisitor=visitor;
        self.methodAndProtoVisitor=visitor;
        self.callSiteVisitor=visitor;
    }
    pub fn setFallbackVisitor(&self, visitor: &Visitor)    {
        self.fallbackVisitor=visitor;
    }
    pub fn setStringVisitor(&self, visitor: &Visitor)    {
        self.stringVisitor=visitor;
    }
    pub fn setTypeVisitor(&self, visitor: &Visitor)    {
        self.typeVisitor=visitor;
    }
    pub fn setFieldVisitor(&self, visitor: &Visitor)    {
        self.fieldVisitor=visitor;
    }
    pub fn setMethodVisitor(&self, visitor: &Visitor)    {
        self.methodVisitor=visitor;
    }
    pub fn setMethodAndProtoVisitor(&self, visitor: &Visitor)    {
        self.methodAndProtoVisitor=visitor;
    }
    pub fn setCallSiteVisitor(&self, visitor: &Visitor)    {
        self.callSiteVisitor=visitor;
    }
    pub fn visitAll(&self, decodedInstructions: &Vec<DecodedInstruction>)    {
        let size: i32 = decodedInstructions.len();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let one: DecodedInstruction = decodedInstructions[i];
            if one==None            {
                continue;
            }            
            callVisit(&decodedInstructions, &one);
        }
    }
    pub fn visitAll(&self, encodedInstructions: &Vec<i16>)    {
        let decodedInstructions: Vec<DecodedInstruction> = DecodedInstruction::decodeAll(&encodedInstructions);
        visitAll_DecodedInstruction[](&decodedInstructions);
    }
    pub fn callVisit(&self, all: &Vec<DecodedInstruction>, one: &DecodedInstruction)    {
        let visitor: Visitor = None;
        match OpcodeInfo::getIndexType(one.getOpcode()){IndexType::STRING_REF =>             visitor=self.stringVisitor;            break;IndexType::TYPE_REF =>             visitor=self.typeVisitor;            break;IndexType::FIELD_REF =>             visitor=self.fieldVisitor;            break;IndexType::METHOD_REF =>             visitor=self.methodVisitor;            break;IndexType::METHOD_AND_PROTO_REF =>             visitor=self.methodAndProtoVisitor;            break;IndexType::CALL_SITE_REF =>             visitor=self.callSiteVisitor;            break;        }
        if visitor==None        {
            visitor=self.fallbackVisitor;
        }        
        if visitor!=None        {
            visitor.visit(&all, &one);
        }        
    }
}
