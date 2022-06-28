use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::cf::code::ByteOps;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::cf::code::SwitchList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstDouble;

struct CodeObserver{
    pub bytes: ByteArray,
    pub observer: ParseObserver,
}
impl CodeObserver{
    pub fn new(&self, bytes: &ByteArray, observer: &ParseObserver)    {
        if bytes==None        {
            throw NullPointerException::new("bytes == null");
        }        
        if observer==None        {
            throw NullPointerException::new("observer == null");
        }        
        self->bytes=bytes;
        self->observer=observer;
    }
    pub fn visitInvalid(&self, opcode: i32, offset: i32, length: i32)    {
        self.observer.parsed(&self.bytes, offset, length, header(offset));
    }
    pub fn visitNoArgs(&self, opcode: i32, offset: i32, length: i32, type: &Type)    {
        self.observer.parsed(&self.bytes, offset, length, header(offset));
    }
    pub fn visitLocal(&self, opcode: i32, offset: i32, length: i32, idx: i32, type: &Type, value: i32)    {
        let idxStr: String = if (length<=3) { Hex::u1(idx) } else { Hex::u2(idx) };
                let argComment: boolean = (length==1);
                let valueStr: String = "";
                if opcode==ByteOps::IINC                {
                    valueStr=", #"+(if (length<=3) { Hex::s1(value) } else { Hex::s2(value) });
                        }                        
                        let catStr: String = "";
                        if type.isCategory2()                        {
                            catStr=(if argComment { "," } else { " //" })+" category-2";
                                }                                
                                self.observer.parsed(&self.bytes, offset, length, header(offset)+(if argComment { " // " } else { " " })+idxStr+valueStr+catStr);
                                    }
                                    pub fn visitConstant(&self, opcode: i32, offset: i32, length: i32, cst: &Constant, value: i32)                                    {
                                        if //cst instanceof CstKnownNull                                        {
                                            visitNoArgs(opcode, offset, length, None);
                                            return;
                                        }                                        
                                        if //cst instanceof CstInteger                                        {
                                            visitLiteralInt(opcode, offset, length, value);
                                            return;
                                        }                                        
                                        if //cst instanceof CstLong                                        {
                                            visitLiteralLong(opcode, offset, length, ((CstLong*)cst).getValue());
                                            return;
                                        }                                        
                                        if //cst instanceof CstFloat                                        {
                                            visitLiteralFloat(opcode, offset, length, ((CstFloat*)cst).getIntBits());
                                            return;
                                        }                                        
                                        if //cst instanceof CstDouble                                        {
                                            visitLiteralDouble(opcode, offset, length, ((CstDouble*)cst).getLongBits());
                                            return;
                                        }                                        
                                        let valueStr: String = "";
                                        if value!=0                                        {
                                            valueStr=", ";
                                            if opcode==ByteOps::MULTIANEWARRAY                                            {
                                                valueStr+=Hex::u1(value);
                                            }                                            else                                             {
                                                valueStr+=Hex::u2(value);
                                            }
                                        }                                        
                                        self.observer.parsed(&self.bytes, offset, length, header(offset)+" "+cst+valueStr);
                                    }
                                    pub fn visitBranch(&self, opcode: i32, offset: i32, length: i32, target: i32)                                    {
                                        let targetStr: String = if (length<=3) { Hex::u2(target) } else { Hex::u4(target) };
                                                self.observer.parsed(&self.bytes, offset, length, header(offset)+" "+targetStr);
                                            }
                                            pub fn visitSwitch(&self, opcode: i32, offset: i32, length: i32, cases: &SwitchList, padding: i32)                                            {
                                                let sz: i32 = cases.size();
                                                let sb: StringBuilder = StringBuilder::new(sz*20+100);
                                                sb.append_String(header(offset));
                                                if padding!=0                                                {
                                                    sb.append_String(" // padding: "+Hex::u4(padding));
                                                }                                                
                                                sb.append_char('\n');
                                                for(                                                let i: i32 = 0;;i<szi += 1)                                                {
                                                    sb.append_String("  ");
                                                    sb.append_String(Hex::s4(cases.getValue(i)));
                                                    sb.append_String(": ");
                                                    sb.append_String(Hex::u2(cases.getTarget(i)));
                                                    sb.append_char('\n');
                                                }
                                                sb.append_String("  default: ");
                                                sb.append_String(Hex::u2(cases.getDefaultTarget()));
                                                self.observer.parsed(&self.bytes, offset, length, sb.toString());
                                            }
                                            pub fn visitNewarray(&self, offset: i32, length: i32, cst: &CstType, intVals: &ArrayList<Constant>)                                            {
                                                let commentOrSpace: String = if (length==1) { " // " } else { " " };
                                                        let typeName: String = cst.getClassType().getComponentType().toHuman();
                                                        self.observer.parsed(&self.bytes, offset, length, header(offset)+commentOrSpace+typeName);
                                                    }
                                                    pub fn setPreviousOffset(&self, offset: i32)                                                    {
                                                    }
                                                    pub fn getPreviousOffset(&self) -> i32                                                    {
                                                        return -1;
                                                    }
                                                    pub fn header(&self, offset: i32) -> String                                                    {
                                                        let opcode: i32 = self.bytes.getUnsignedByte(offset);
                                                        let name: String = ByteOps::opName(opcode);
                                                        if opcode==ByteOps::WIDE                                                        {
                                                            opcode=self.bytes.getUnsignedByte(offset+1);
                                                            name+=" "+ByteOps::opName(opcode);
                                                        }                                                        
                                                        return Hex::u2(offset)+": "+name;
                                                    }
                                                    pub fn visitLiteralInt(&self, opcode: i32, offset: i32, length: i32, value: i32)                                                    {
                                                        let commentOrSpace: String = if (length==1) { " // " } else { " " };
                                                                let valueStr: String;
                                                                opcode=self.bytes.getUnsignedByte(offset);
                                                                if (length==1)||(opcode==ByteOps::BIPUSH)                                                                {
                                                                    valueStr="#"+Hex::s1(value);
                                                                }                                                                else                                                                 if opcode==ByteOps::SIPUSH                                                                {
                                                                    valueStr="#"+Hex::s2(value);
                                                                }                                                                else                                                                 {
                                                                    valueStr="#"+Hex::s4(value);
                                                                }
                                                                self.observer.parsed(&self.bytes, offset, length, header(offset)+commentOrSpace+valueStr);
                                                            }
                                                            pub fn visitLiteralLong(&self, opcode: i32, offset: i32, length: i32, value: i64)                                                            {
                                                                let commentOrLit: String = if (length==1) { " // " } else { " #" };
                                                                        let valueStr: String;
                                                                        if length==1                                                                        {
                                                                            valueStr=Hex::s1(value as i32);
                                                                        }                                                                        else                                                                         {
                                                                            valueStr=Hex::s8(value);
                                                                        }
                                                                        self.observer.parsed(&self.bytes, offset, length, header(offset)+commentOrLit+valueStr);
                                                                    }
                                                                    pub fn visitLiteralFloat(&self, opcode: i32, offset: i32, length: i32, bits: i32)                                                                    {
                                                                        let optArg: String = if (length!=1) { " #"+Hex::u4(bits) } else { "" };
                                                                                self.observer.parsed(&self.bytes, offset, length, header(offset)+optArg+" // "+Float::intBitsToFloat(bits));
                                                                            }
                                                                            pub fn visitLiteralDouble(&self, opcode: i32, offset: i32, length: i32, bits: i64)                                                                            {
                                                                                let optArg: String = if (length!=1) { " #"+Hex::u8(bits) } else { "" };
                                                                                        self.observer.parsed(&self.bytes, offset, length, header(offset)+optArg+" // "+Double::longBitsToDouble(bits));
                                                                                    }
}
