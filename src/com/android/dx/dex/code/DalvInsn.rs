use crate::helper;
use crate::com::android::dx::dex::code::SimpleInsn;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::util::TwoColumnOutput;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::dex::code::HighRegisterPrefix;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::code::Dops;

struct DalvInsn{
    pub address: i32,
    pub opcode: Dop,
    pub position: SourcePosition,
    pub registers: RegisterSpecList,
}
impl DalvInsn{
    pub fn makeMove(position: &SourcePosition, dest: &RegisterSpec, src: &RegisterSpec) -> SimpleInsn    {
        let category1: boolean = dest.getCategory()==1;
        let reference: boolean = dest.getType().isReference();
        let destReg: i32 = dest.getReg();
        let srcReg: i32 = src.getReg();
        let opcode: Dop;
        if (srcReg|destReg)<16        {
            opcode=if reference { Dops::MOVE_OBJECT } else { (if category1 { Dops::MOVE } else { Dops::MOVE_WIDE }) };
                        }                        else                         if destReg<256                        {
                            opcode=if reference { Dops::MOVE_OBJECT_FROM16 } else { (if category1 { Dops::MOVE_FROM16 } else { Dops::MOVE_WIDE_FROM16 }) };
                                        }                                        else                                         {
                                            opcode=if reference { Dops::MOVE_OBJECT_16 } else { (if category1 { Dops::MOVE_16 } else { Dops::MOVE_WIDE_16 }) };
                                                        }
                                                        return SimpleInsn::new(&opcode, &position, RegisterSpecList::make_RegisterSpec_RegisterSpec(&dest, &src));
                                                    }
                                                    pub fn new(&self, opcode: &Dop, position: &SourcePosition, registers: &RegisterSpecList)                                                    {
                                                        if opcode==None                                                        {
                                                            throw NullPointerException::new("opcode == null");
                                                        }                                                        
                                                        if position==None                                                        {
                                                            throw NullPointerException::new("position == null");
                                                        }                                                        
                                                        if registers==None                                                        {
                                                            throw NullPointerException::new("registers == null");
                                                        }                                                        
                                                        self->address=-1;
                                                        self->opcode=opcode;
                                                        self->position=position;
                                                        self->registers=registers;
                                                    }
                                                    pub fn toString(&self) -> String                                                    {
                                                        let sb: StringBuilder = StringBuilder::new(100);
                                                        sb.append_String(identifierString());
                                                        sb.append_char(' ');
                                                        sb.append_Object(&self.position);
                                                        sb.append_String(": ");
                                                        sb.append_String(self.opcode.getName());
                                                        let needComma: boolean = false;
                                                        if self.registers.size()!=0                                                        {
                                                            sb.append_String(self.registers.toHuman_String_String_String(" ", ", ", None));
                                                            needComma=true;
                                                        }                                                        
                                                        let extra: String = argString();
                                                        if extra!=None                                                        {
                                                            if needComma                                                            {
                                                                sb.append_char(',');
                                                            }                                                            
                                                            sb.append_char(' ');
                                                            sb.append_String(&extra);
                                                        }                                                        
                                                        return sb.toString();
                                                    }
                                                    pub fn hasAddress(&self) -> boolean                                                    {
                                                        return (self.address>=0);
                                                    }
                                                    pub fn getAddress(&self) -> i32                                                    {
                                                        if self.address<0                                                        {
                                                            throw RuntimeException::new("address not yet known");
                                                        }                                                        
                                                        return self.address;
                                                    }
                                                    pub fn getOpcode(&self) -> Dop                                                    {
                                                        return self.opcode;
                                                    }
                                                    pub fn getPosition(&self) -> SourcePosition                                                    {
                                                        return self.position;
                                                    }
                                                    pub fn getRegisters(&self) -> RegisterSpecList                                                    {
                                                        return self.registers;
                                                    }
                                                    pub fn hasResult(&self) -> boolean                                                    {
                                                        return self.opcode.hasResult();
                                                    }
                                                    pub fn getMinimumRegisterRequirement(&self, compatRegs: &BitSet) -> i32                                                    {
                                                        let hasResult: boolean = hasResult();
                                                        let regSz: i32 = self.registers.size();
                                                        let resultRequirement: i32 = 0;
                                                        let sourceRequirement: i32 = 0;
                                                        if hasResult&&!compatRegs.get_int(0)                                                        {
                                                            resultRequirement=self.registers.get(0).getCategory();
                                                        }                                                        
                                                        for(                                                        let i: i32 = if hasResult { 1 } else { 0 };;i<regSzi += 1)                                                                {
                                                                    if !compatRegs.get_int(i)                                                                    {
                                                                        sourceRequirement+=self.registers.get(i).getCategory();
                                                                    }                                                                    
                                                                }
                                                                return Math::max_int_int(sourceRequirement, resultRequirement);
                                                            }
                                                            pub fn getLowRegVersion(&self) -> DalvInsn                                                            {
                                                                let regs: RegisterSpecList = self.registers.withExpandedRegisters(0, hasResult(), None);
                                                                return withRegisters(&regs);
                                                            }
                                                            pub fn expandedPrefix(&self, compatRegs: &BitSet) -> DalvInsn                                                            {
                                                                let regs: RegisterSpecList = self.registers;
                                                                let firstBit: boolean = compatRegs.get_int(0);
                                                                if hasResult(){                                                                    compatRegs.set_int(0);                                                                }                                                                
                                                                regs=regs.subset(&compatRegs);
                                                                if hasResult(){                                                                    compatRegs.set_int_boolean(0, firstBit);                                                                }                                                                
                                                                if regs.size()==0{                                                                    return None;                                                                }                                                                
                                                                return HighRegisterPrefix::new(&self.position, &regs);
                                                            }
                                                            pub fn expandedSuffix(&self, compatRegs: &BitSet) -> DalvInsn                                                            {
                                                                if hasResult()&&!compatRegs.get_int(0)                                                                {
                                                                    let r: RegisterSpec = self.registers.get(0);
                                                                    return DalvInsn::makeMove(&self.position, &r, r.withReg(0));
                                                                }                                                                else                                                                 {
                                                                    return None;
                                                                }
                                                            }
                                                            pub fn expandedVersion(&self, compatRegs: &BitSet) -> DalvInsn                                                            {
                                                                let regs: RegisterSpecList = self.registers.withExpandedRegisters(0, hasResult(), &compatRegs);
                                                                return withRegisters(&regs);
                                                            }
                                                            pub fn identifierString(&self) -> String                                                            {
                                                                if self.address!=-1                                                                {
                                                                    return String::format_String_Object[]("%04x", self.address);
                                                                }                                                                
                                                                return Hex::u4(System::identityHashCode(self));
                                                            }
                                                            pub fn listingString(&self, prefix: &String, width: i32, noteIndices: boolean) -> String                                                            {
                                                                let insnPerSe: String = listingString0(noteIndices);
                                                                if insnPerSe==None                                                                {
                                                                    return None;
                                                                }                                                                
                                                                let addr: String = prefix+identifierString()+": ";
                                                                let w1: i32 = addr.length();
                                                                let w2: i32 = if (width==0) { insnPerSe.length() } else { (width-w1) };
                                                                        return TwoColumnOutput::toString(&addr, w1, "", &insnPerSe, w2);
                                                                    }
                                                                    pub fn setAddress(&self, address: i32)                                                                    {
                                                                        if address<0                                                                        {
                                                                            throw IllegalArgumentException::new("address < 0");
                                                                        }                                                                        
                                                                        self->address=address;
                                                                    }
                                                                    pub fn getNextAddress(&self) -> i32                                                                    {
                                                                        return getAddress()+codeSize();
                                                                    }
                                                                    pub fn withMapper(&self, mapper: &RegisterMapper) -> DalvInsn                                                                    {
                                                                        return withRegisters(mapper.map_RegisterSpecList(getRegisters()));
                                                                    }
                                                                    pub fn codeSize(&self) -> i32;
                                                                    pub fn writeTo(&self, out: &AnnotatedOutput);
                                                                    pub fn withOpcode(&self, opcode: &Dop) -> DalvInsn;
                                                                    pub fn withRegisterOffset(&self, delta: i32) -> DalvInsn;
                                                                    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn;
                                                                    pub fn argString(&self) -> String;
                                                                    pub fn listingString0(&self, noteIndices: boolean) -> String;
                                                                    pub fn cstString(&self) -> String                                                                    {
                                                                        throw UnsupportedOperationException::new("Not supported.");
                                                                    }
                                                                    pub fn cstComment(&self) -> String                                                                    {
                                                                        throw UnsupportedOperationException::new("Not supported.");
                                                                    }
}
