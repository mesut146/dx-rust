use crate::helper;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::util::Hex;

let static NO_INFO: SourcePosition = SourcePosition::new(None, -1, -1);
struct SourcePosition{
    pub sourceFile: CstString,
    pub address: i32,
    pub line: i32,
}
impl SourcePosition{
    pub fn new(&self, sourceFile: &CstString, address: i32, line: i32)    {
        if address<-1        {
            throw IllegalArgumentException::new("address < -1");
        }        
        if line<-1        {
            throw IllegalArgumentException::new("line < -1");
        }        
        self->sourceFile=sourceFile;
        self->address=address;
        self->line=line;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(50);
        if self.sourceFile!=None        {
            sb.append_String(self.sourceFile.toHuman());
            sb.append_String(":");
        }        
        if self.line>=0        {
            sb.append_int(self.line);
        }        
        sb.append_char('@');
        if self.address<0        {
            sb.append_String("????");
        }        else         {
            sb.append_String(Hex::u2(self.address));
        }
        return sb.toString();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof SourcePosition)        {
            return false;
        }        
        if self==other        {
            return true;
        }        
        let pos: SourcePosition = (SourcePosition*)other;
        return (self.address==)&&sameLineAndFile(&pos);
    }
    pub fn hashCode(&self) -> i32    {
        return self.sourceFile.hashCode()+self.address+self.line;
    }
    pub fn sameLine(&self, other: &SourcePosition) -> boolean    {
        return (self.line==);
    }
    pub fn sameLineAndFile(&self, other: &SourcePosition) -> boolean    {
        return (self.line==)&&((self.sourceFile==)||((self.sourceFile!=None)&&self.sourceFile.equals(&)));
    }
    pub fn getSourceFile(&self) -> CstString    {
        return self.sourceFile;
    }
    pub fn getAddress(&self) -> i32    {
        return self.address;
    }
    pub fn getLine(&self) -> i32    {
        return self.line;
    }
}
