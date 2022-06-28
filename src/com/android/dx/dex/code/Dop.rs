use crate::helper;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::InsnFormat;
use crate::com::android::dx::io::OpcodeInfo;
use crate::com::android::dx::dex::code::Dops;

struct Dop{
    pub opcode: i32,
    pub family: i32,
    pub nextOpcode: i32,
    pub format: InsnFormat,
    pub hasResult: boolean,
}
impl Dop{
    pub fn new(&self, opcode: i32, family: i32, nextOpcode: i32, format: &InsnFormat, hasResult: boolean)    {
        if !Opcodes::isValidShape(opcode)        {
            throw IllegalArgumentException::new("bogus opcode");
        }        
        if !Opcodes::isValidShape(family)        {
            throw IllegalArgumentException::new("bogus family");
        }        
        if !Opcodes::isValidShape(nextOpcode)        {
            throw IllegalArgumentException::new("bogus nextOpcode");
        }        
        if format==None        {
            throw NullPointerException::new("format == null");
        }        
        self->opcode=opcode;
        self->family=family;
        self->nextOpcode=nextOpcode;
        self->format=format;
        self->hasResult=hasResult;
    }
    pub fn toString(&self) -> String    {
        return getName();
    }
    pub fn getOpcode(&self) -> i32    {
        return self.opcode;
    }
    pub fn getFamily(&self) -> i32    {
        return self.family;
    }
    pub fn getFormat(&self) -> InsnFormat    {
        return self.format;
    }
    pub fn hasResult(&self) -> boolean    {
        return self.hasResult;
    }
    pub fn getName(&self) -> String    {
        return OpcodeInfo::getName(self.opcode);
    }
    pub fn getNextOpcode(&self) -> i32    {
        return self.nextOpcode;
    }
    pub fn getOppositeTest(&self) -> Dop    {
        match self.opcode{Opcodes::IF_EQ =>             return Dops::IF_NE;Opcodes::IF_NE =>             return Dops::IF_EQ;Opcodes::IF_LT =>             return Dops::IF_GE;Opcodes::IF_GE =>             return Dops::IF_LT;Opcodes::IF_GT =>             return Dops::IF_LE;Opcodes::IF_LE =>             return Dops::IF_GT;Opcodes::IF_EQZ =>             return Dops::IF_NEZ;Opcodes::IF_NEZ =>             return Dops::IF_EQZ;Opcodes::IF_LTZ =>             return Dops::IF_GEZ;Opcodes::IF_GEZ =>             return Dops::IF_LTZ;Opcodes::IF_GTZ =>             return Dops::IF_LEZ;Opcodes::IF_LEZ =>             return Dops::IF_GTZ;        }
        throw IllegalArgumentException::new("bogus opcode: "+self);
    }
}
