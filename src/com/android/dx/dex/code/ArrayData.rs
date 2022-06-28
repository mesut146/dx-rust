use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::rop::cst::CstLiteral32;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstLiteral64;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::code::ArrayData;

struct ArrayData{
    pub user: CodeAddress,
    pub values: ArrayList<Constant>,
    pub arrayType: Constant,
    pub elemWidth: i32,
    pub initLength: i32,
}
impl ArrayData{
    pub fn new(&self, position: &SourcePosition, user: &CodeAddress, values: &ArrayList<Constant>, arrayType: &Constant)    {
        super(position,RegisterSpecList.EMPTY);

        if user==None        {
            throw NullPointerException::new("user == null");
        }        
        if values==None        {
            throw NullPointerException::new("values == null");
        }        
        let sz: i32 = values.size();
        if sz<=0        {
            throw IllegalArgumentException::new("Illegal number of init values");
        }        
        self->arrayType=arrayType;
        if arrayType==CstType::BYTE_ARRAY||arrayType==CstType::BOOLEAN_ARRAY        {
            self.elemWidth=1;
        }        else         if arrayType==CstType::SHORT_ARRAY||arrayType==CstType::CHAR_ARRAY        {
            self.elemWidth=2;
        }        else         if arrayType==CstType::INT_ARRAY||arrayType==CstType::FLOAT_ARRAY        {
            self.elemWidth=4;
        }        else         if arrayType==CstType::LONG_ARRAY||arrayType==CstType::DOUBLE_ARRAY        {
            self.elemWidth=8;
        }        else         {
            throw IllegalArgumentException::new("Unexpected constant type");
        }
        self->user=user;
        self->values=values;
        self.initLength=values.size();
    }
    pub fn codeSize(&self) -> i32    {
        let sz: i32 = self.initLength;
        return 4+((sz*self.elemWidth)+1)/2;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
        let sz: i32 = self.values.size();
        out.writeShort(Opcodes::FILL_ARRAY_DATA_PAYLOAD);
        out.writeShort(self.elemWidth);
        out.writeInt(self.initLength);
        match self.elemWidth{1 =>             {
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let cst: Constant = self.values.get(i);
                    out.writeByte(((CstLiteral32*)cst).getIntBits() as i8);
                }
                break;
            }2 =>             {
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let cst: Constant = self.values.get(i);
                    out.writeShort(((CstLiteral32*)cst).getIntBits() as i16);
                }
                break;
            }4 =>             {
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let cst: Constant = self.values.get(i);
                    out.writeInt(((CstLiteral32*)cst).getIntBits());
                }
                break;
            }8 =>             {
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let cst: Constant = self.values.get(i);
                    out.writeLong(((CstLiteral64*)cst).getLongBits());
                }
                break;
            }        _ => {}        break;    }
    if self.elemWidth==1&&(sz%2!=0)    {
        out.writeByte(0x00);
    }    
}
pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn{
    return ArrayData::new(getPosition(), &self.user, &self.values, &self.arrayType);
}
pub fn argString(&self) -> String{
    let sb: StringBuilder = StringBuilder::new(100);
    let sz: i32 = self.values.size();
    for(    let i: i32 = 0;;i<szi += 1)    {
        sb.append_String("\n    ");
        sb.append_int(i);
        sb.append_String(": ");
        sb.append_String(self.values.get(i).toHuman());
    }
    return sb.toString();
}
pub fn listingString0(&self, noteIndices: boolean) -> String{
    let baseAddress: i32 = self.user.getAddress();
    let sb: StringBuilder = StringBuilder::new(100);
    let sz: i32 = self.values.size();
    sb.append_String("fill-array-data-payload // for fill-array-data @ ");
    sb.append_String(Hex::u2(baseAddress));
    for(    let i: i32 = 0;;i<szi += 1)    {
        sb.append_String("\n  ");
        sb.append_int(i);
        sb.append_String(": ");
        sb.append_String(self.values.get(i).toHuman());
    }
    return sb.toString();
}
}
