use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::util::Hex;

struct CstInsn{
    pub constant: Constant,
    pub index: i32,
    pub classIndex: i32,
}
impl CstInsn{
    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList, constant: &Constant)    {
        super(opcode,position,registers);

        if constant==None        {
            throw NullPointerException::new("constant == null");
        }        
        self->constant=constant;
        self->index=-1;
        self->classIndex=-1;
    }
    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn    {
        let result: CstInsn = CstInsn::new(&opcode, getPosition(), getRegisters(), &self.constant);
        if self.index>=0        {
            result.setIndex(self.index);
        }        
        if self.classIndex>=0        {
            result.setClassIndex(self.classIndex);
        }        
        return result;
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        let result: CstInsn = CstInsn::new(getOpcode(), getPosition(), &registers, &self.constant);
        if self.index>=0        {
            result.setIndex(self.index);
        }        
        if self.classIndex>=0        {
            result.setClassIndex(self.classIndex);
        }        
        return result;
    }
    pub fn getConstant(&self) -> Constant    {
        return self.constant;
    }
    pub fn getIndex(&self) -> i32    {
        if self.index<0        {
            throw IllegalStateException::new("index not yet set for "+self.constant);
        }        
        return self.index;
    }
    pub fn hasIndex(&self) -> boolean    {
        return (self.index>=0);
    }
    pub fn setIndex(&self, index: i32)    {
        if index<0        {
            throw IllegalArgumentException::new("index < 0");
        }        
        if self->index>=0        {
            throw IllegalStateException::new("index already set");
        }        
        self->index=index;
    }
    pub fn getClassIndex(&self) -> i32    {
        if self.classIndex<0        {
            throw IllegalStateException::new("class index not yet set");
        }        
        return self.classIndex;
    }
    pub fn hasClassIndex(&self) -> boolean    {
        return (self.classIndex>=0);
    }
    pub fn setClassIndex(&self, index: i32)    {
        if index<0        {
            throw IllegalArgumentException::new("index < 0");
        }        
        if self->classIndex>=0        {
            throw IllegalStateException::new("class index already set");
        }        
        self->classIndex=index;
    }
    pub fn argString(&self) -> String    {
        return self.constant.toHuman();
    }
    pub fn cstString(&self) -> String    {
        if //constant instanceof CstString        {
            return ((CstString*)self.constant).toQuoted();
        }        
        return self.constant.toHuman();
    }
    pub fn cstComment(&self) -> String    {
        if !hasIndex()        {
            return "";
        }        
        let sb: StringBuilder = StringBuilder::new(20);
        sb.append_String(getConstant().typeName());
        sb.append_char('@');
        if self.index<65536        {
            sb.append_String(Hex::u2(self.index));
        }        else         {
            sb.append_String(Hex::u4(self.index));
        }
        return sb.toString();
    }
}
