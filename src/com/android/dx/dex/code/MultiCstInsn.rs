use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::code::MultiCstInsn;

struct MultiCstInsn{
    pub constants: Vec<Constant>,
    pub index: Vec<i32>,
    pub classIndex: i32,
}
impl MultiCstInsn{
    pub const NOT_SET: i32 = -1;
    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList, constants: &Vec<Constant>)    {
        super(opcode,position,registers);

        if constants==None        {
            throw NullPointerException::new("constants == null");
        }        
        self->constants=constants;
        self->index=new int[constants.length];
        for(        let i: i32 = 0;;i<self->index->len()++i)        {
            if constants[i]==None            {
                throw NullPointerException::new("constants[i] == null");
            }            
            self->index[i]=MultiCstInsn::NOT_SET;
        }
        self->classIndex=MultiCstInsn::NOT_SET;
    }
    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList, constants: &Vec<Constant>, index: &Vec<i32>, classIndex: i32)    {
        super(opcode,position,registers);

        self->constants=constants;
        self->index=index;
        self->classIndex=classIndex;
    }
    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn    {
        return MultiCstInsn::new(&opcode, getPosition(), getRegisters(), self->constants, self->index, self->classIndex);
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return MultiCstInsn::new(getOpcode(), getPosition(), &registers, self->constants, self->index, self->classIndex);
    }
    pub fn getNumberOfConstants(&self) -> i32    {
        return self.constants.len();
    }
    pub fn getConstant(&self, position: i32) -> Constant    {
        return self.constants[position];
    }
    pub fn getIndex(&self, position: i32) -> i32    {
        if !hasIndex(position)        {
            throw IllegalStateException::new("index not yet set for constant "+position+" value = "+self.constants[position]);
        }        
        return self.index[position];
    }
    pub fn hasIndex(&self, position: i32) -> boolean    {
        return (self.index[position]!=MultiCstInsn::NOT_SET);
    }
    pub fn setIndex(&self, position: i32, index: i32)    {
        if index<0        {
            throw IllegalArgumentException::new("index < 0");
        }        
        if hasIndex(position)        {
            throw IllegalStateException::new("index already set");
        }        
        self->index[position]=index;
    }
    pub fn getClassIndex(&self) -> i32    {
        if !hasClassIndex()        {
            throw IllegalStateException::new("class index not yet set");
        }        
        return self.classIndex;
    }
    pub fn hasClassIndex(&self) -> boolean    {
        return (self.classIndex!=MultiCstInsn::NOT_SET);
    }
    pub fn setClassIndex(&self, index: i32)    {
        if index<0        {
            throw IllegalArgumentException::new("index < 0");
        }        
        if hasClassIndex()        {
            throw IllegalStateException::new("class index already set");
        }        
        self->classIndex=index;
    }
    pub fn argString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        for(        let i: i32 = 0;;i<self.constants.len()++i)        {
            if sb.length()>0{                sb.append_String(", ");            }            
            sb.append_String(self.constants[i].toHuman());
        }
        return sb.toString();
    }
    pub fn cstString(&self) -> String    {
        return argString();
    }
    pub fn cstComment(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        for(        let i: i32 = 0;;i<self.constants.len()++i)        {
            if !hasIndex(i)            {
                return "";
            }            
            if i>0            {
                sb.append_String(", ");
            }            
            sb.append_String(getConstant(i).typeName());
            sb.append_char('@');
            let currentIndex: i32 = getIndex(i);
            if currentIndex<65536            {
                sb.append_String(Hex::u2(currentIndex));
            }            else             {
                sb.append_String(Hex::u4(currentIndex));
            }
        }
        return sb.toString();
    }
}
