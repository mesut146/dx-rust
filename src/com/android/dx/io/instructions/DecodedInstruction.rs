use crate::helper;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::io::IndexType;
use crate::com::android::dx::io::instructions::CodeInput;
use crate::com::android::dx::io::instructions::ShortArrayCodeInput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::io::instructions::DecodedInstruction;
use crate::com::android::dx::io::OpcodeInfo;
use crate::com::android::dex::DexException;
use crate::com::android::dx::io::instructions::InstructionCodec;

struct DecodedInstruction{
    pub format: InstructionCodec,
    pub opcode: i32,
    pub index: i32,
    pub indexType: IndexType,
    pub target: i32,
    pub literal: i64,
}
impl DecodedInstruction{
    pub fn decode(in: &CodeInput) -> DecodedInstruction    {
        let opcodeUnit: i32 = in.read();
        let opcode: i32 = Opcodes::extractOpcodeFromUnit(opcodeUnit);
        let format: InstructionCodec = OpcodeInfo::getFormat(opcode);
        return format.decode(opcodeUnit, &in);
    }
    pub fn decodeAll(encodedInstructions: &Vec<i16>) -> Vec<DecodedInstruction>    {
        let size: i32 = encodedInstructions.len();
        let decoded: Vec<DecodedInstruction> = new DecodedInstruction[size];
        let in: ShortArrayCodeInput = ShortArrayCodeInput::new(&encodedInstructions);
        try        {
            while in_renamed.hasMore()            {
                decoded[in_renamed.cursor()]=DecodedInstruction::decode(&in_renamed);
            }
        }        catch(        let ex: EOFException)        {
            throw DexException::new(&ex);
        }
        return decoded;
    }
    pub fn new(&self, format: &InstructionCodec, opcode: i32, index: i32, indexType: &IndexType, target: i32, literal: i64)    {
        if format==None        {
            throw NullPointerException::new("format == null");
        }        
        if !Opcodes::isValidShape(opcode)        {
            throw IllegalArgumentException::new("invalid opcode");
        }        
        self->format=format;
        self->opcode=opcode;
        self->index=index;
        self->indexType=indexType;
        self->target=target;
        self->literal=literal;
    }
    pub fn getFormat(&self) -> InstructionCodec    {
        return self.format;
    }
    pub fn getOpcode(&self) -> i32    {
        return self.opcode;
    }
    pub fn getOpcodeUnit(&self) -> i16    {
        return self.opcode as i16;
    }
    pub fn getIndex(&self) -> i32    {
        return self.index;
    }
    pub fn getIndexUnit(&self) -> i16    {
        return self.index as i16;
    }
    pub fn getIndexType(&self) -> IndexType    {
        return self.indexType;
    }
    pub fn getTarget(&self) -> i32    {
        return self.target;
    }
    pub fn getTarget(&self, baseAddress: i32) -> i32    {
        return self.target-baseAddress;
    }
    pub fn getTargetUnit(&self, baseAddress: i32) -> i16    {
        let relativeTarget: i32 = getTarget_int(baseAddress);
        if relativeTarget!=relativeTarget as i16        {
            throw DexException::new("Target out of range: "+Hex::s4(relativeTarget));
        }        
        return relativeTarget as i16;
    }
    pub fn getTargetByte(&self, baseAddress: i32) -> i32    {
        let relativeTarget: i32 = getTarget_int(baseAddress);
        if relativeTarget!=relativeTarget as i8        {
            throw DexException::new("Target out of range: "+Hex::s4(relativeTarget));
        }        
        return relativeTarget&0xff;
    }
    pub fn getLiteral(&self) -> i64    {
        return self.literal;
    }
    pub fn getLiteralInt(&self) -> i32    {
        if self.literal!=self.literal as i32        {
            throw DexException::new("Literal out of range: "+Hex::u8(self.literal));
        }        
        return self.literal as i32;
    }
    pub fn getLiteralUnit(&self) -> i16    {
        if self.literal!=self.literal as i16        {
            throw DexException::new("Literal out of range: "+Hex::u8(self.literal));
        }        
        return self.literal as i16;
    }
    pub fn getLiteralByte(&self) -> i32    {
        if self.literal!=self.literal as i8        {
            throw DexException::new("Literal out of range: "+Hex::u8(self.literal));
        }        
        return self.literal as i32&0xff;
    }
    pub fn getLiteralNibble(&self) -> i32    {
        if (self.literal<-8)||(self.literal>7)        {
            throw DexException::new("Literal out of range: "+Hex::u8(self.literal));
        }        
        return self.literal as i32&0xf;
    }
    pub fn getRegisterCount(&self) -> i32;
    pub fn getA(&self) -> i32    {
        return 0;
    }
    pub fn getB(&self) -> i32    {
        return 0;
    }
    pub fn getC(&self) -> i32    {
        return 0;
    }
    pub fn getD(&self) -> i32    {
        return 0;
    }
    pub fn getE(&self) -> i32    {
        return 0;
    }
    pub fn getRegisterCountUnit(&self) -> i16    {
        let registerCount: i32 = getRegisterCount();
        if (registerCount&~0xffff)!=0        {
            throw DexException::new("Register count out of range: "+Hex::u8(registerCount));
        }        
        return registerCount as i16;
    }
    pub fn getAUnit(&self) -> i16    {
        let a: i32 = getA();
        if (a&~0xffff)!=0        {
            throw DexException::new("Register A out of range: "+Hex::u8(a));
        }        
        return a as i16;
    }
    pub fn getAByte(&self) -> i16    {
        let a: i32 = getA();
        if (a&~0xff)!=0        {
            throw DexException::new("Register A out of range: "+Hex::u8(a));
        }        
        return a as i16;
    }
    pub fn getANibble(&self) -> i16    {
        let a: i32 = getA();
        if (a&~0xf)!=0        {
            throw DexException::new("Register A out of range: "+Hex::u8(a));
        }        
        return a as i16;
    }
    pub fn getBUnit(&self) -> i16    {
        let b: i32 = getB();
        if (b&~0xffff)!=0        {
            throw DexException::new("Register B out of range: "+Hex::u8(b));
        }        
        return b as i16;
    }
    pub fn getBByte(&self) -> i16    {
        let b: i32 = getB();
        if (b&~0xff)!=0        {
            throw DexException::new("Register B out of range: "+Hex::u8(b));
        }        
        return b as i16;
    }
    pub fn getBNibble(&self) -> i16    {
        let b: i32 = getB();
        if (b&~0xf)!=0        {
            throw DexException::new("Register B out of range: "+Hex::u8(b));
        }        
        return b as i16;
    }
    pub fn getCUnit(&self) -> i16    {
        let c: i32 = getC();
        if (c&~0xffff)!=0        {
            throw DexException::new("Register C out of range: "+Hex::u8(c));
        }        
        return c as i16;
    }
    pub fn getCByte(&self) -> i16    {
        let c: i32 = getC();
        if (c&~0xff)!=0        {
            throw DexException::new("Register C out of range: "+Hex::u8(c));
        }        
        return c as i16;
    }
    pub fn getCNibble(&self) -> i16    {
        let c: i32 = getC();
        if (c&~0xf)!=0        {
            throw DexException::new("Register C out of range: "+Hex::u8(c));
        }        
        return c as i16;
    }
    pub fn getDUnit(&self) -> i16    {
        let d: i32 = getD();
        if (d&~0xffff)!=0        {
            throw DexException::new("Register D out of range: "+Hex::u8(d));
        }        
        return d as i16;
    }
    pub fn getDByte(&self) -> i16    {
        let d: i32 = getD();
        if (d&~0xff)!=0        {
            throw DexException::new("Register D out of range: "+Hex::u8(d));
        }        
        return d as i16;
    }
    pub fn getDNibble(&self) -> i16    {
        let d: i32 = getD();
        if (d&~0xf)!=0        {
            throw DexException::new("Register D out of range: "+Hex::u8(d));
        }        
        return d as i16;
    }
    pub fn getENibble(&self) -> i16    {
        let e: i32 = getE();
        if (e&~0xf)!=0        {
            throw DexException::new("Register E out of range: "+Hex::u8(e));
        }        
        return e as i16;
    }
    pub fn encode(&self, out: &CodeOutput)    {
        self.format.encode(self, &out);
    }
    pub fn withIndex(&self, newIndex: i32) -> DecodedInstruction;
    pub fn withProtoIndex(&self, newIndex: i32, newProtoIndex: i32) -> DecodedInstruction    {
        throw IllegalStateException::new(getClass().toString());
    }
    pub fn getProtoIndex(&self) -> i16    {
        throw IllegalStateException::new(getClass().toString());
    }
}
