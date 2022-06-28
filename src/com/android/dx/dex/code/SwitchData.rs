use crate::helper;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::code::SwitchData;
use crate::com::android::dx::dex::code::InsnFormat;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::dex::code::Dops;

struct SwitchData{
    pub user: CodeAddress,
    pub cases: IntList,
    pub targets: Vec<CodeAddress>,
    pub packed: boolean,
}
impl SwitchData{
    pub fn new(&self, position: &SourcePosition, user: &CodeAddress, cases: &IntList, targets: &Vec<CodeAddress>)    {
        super(position,RegisterSpecList.EMPTY);

        if user==None        {
            throw NullPointerException::new("user == null");
        }        
        if cases==None        {
            throw NullPointerException::new("cases == null");
        }        
        if targets==None        {
            throw NullPointerException::new("targets == null");
        }        
        let sz: i32 = cases.size();
        if sz!=targets.len()        {
            throw IllegalArgumentException::new("cases / targets mismatch");
        }        
        if sz>65535        {
            throw IllegalArgumentException::new("too many cases");
        }        
        self->user=user;
        self->cases=cases;
        self->targets=targets;
        self->packed=SwitchData::shouldPack(&cases);
    }
    pub fn codeSize(&self) -> i32    {
        return if self.packed { SwitchData::packedCodeSize(&self.cases) as i32 } else { SwitchData::sparseCodeSize(&self.cases) as i32 };
            }
            pub fn writeTo(&self, out: &AnnotatedOutput)            {
                let baseAddress: i32 = self.user.getAddress();
                let defaultTarget: i32 = Dops::PACKED_SWITCH.getFormat().codeSize();
                let sz: i32 = self.targets.len();
                if self.packed                {
                    let firstCase: i32 = if (sz==0) { 0 } else { self.cases.get(0) };
                            let lastCase: i32 = if (sz==0) { 0 } else { self.cases.get(sz-1) };
                                    let outSz: i32 = lastCase-firstCase+1;
                                    out.writeShort(Opcodes::PACKED_SWITCH_PAYLOAD);
                                    out.writeShort(outSz);
                                    out.writeInt(firstCase);
                                    let caseAt: i32 = 0;
                                    for(                                    let i: i32 = 0;;i<outSzi += 1)                                    {
                                        let outCase: i32 = firstCase+i;
                                        let oneCase: i32 = self.cases.get(caseAt);
                                        let relTarget: i32;
                                        if oneCase>outCase                                        {
                                            relTarget=defaultTarget;
                                        }                                        else                                         {
                                            relTarget=self.targets[caseAt].getAddress()-baseAddress;
                                            caseAt += 1;
                                        }
                                        out.writeInt(relTarget);
                                    }
                                }                                else                                 {
                                    out.writeShort(Opcodes::SPARSE_SWITCH_PAYLOAD);
                                    out.writeShort(sz);
                                    for(                                    let i: i32 = 0;;i<szi += 1)                                    {
                                        out.writeInt(self.cases.get(i));
                                    }
                                    for(                                    let i: i32 = 0;;i<szi += 1)                                    {
                                        let relTarget: i32 = self.targets[i].getAddress()-baseAddress;
                                        out.writeInt(relTarget);
                                    }
                                }
                            }
                            pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn                            {
                                return SwitchData::new(getPosition(), &self.user, &self.cases, &self.targets);
                            }
                            pub fn isPacked(&self) -> boolean                            {
                                return self.packed;
                            }
                            pub fn argString(&self) -> String                            {
                                let sb: StringBuilder = StringBuilder::new(100);
                                let sz: i32 = self.targets.len();
                                for(                                let i: i32 = 0;;i<szi += 1)                                {
                                    sb.append_String("\n    ");
                                    sb.append_int(self.cases.get(i));
                                    sb.append_String(": ");
                                    sb.append_Object(self.targets[i]);
                                }
                                return sb.toString();
                            }
                            pub fn listingString0(&self, noteIndices: boolean) -> String                            {
                                let baseAddress: i32 = self.user.getAddress();
                                let sb: StringBuilder = StringBuilder::new(100);
                                let sz: i32 = self.targets.len();
                                sb.append_String(if self.packed { "packed" } else { "sparse" });
                                        sb.append_String("-switch-payload // for switch @ ");
                                        sb.append_String(Hex::u2(baseAddress));
                                        for(                                        let i: i32 = 0;;i<szi += 1)                                        {
                                            let absTarget: i32 = self.targets[i].getAddress();
                                            let relTarget: i32 = absTarget-baseAddress;
                                            sb.append_String("\n  ");
                                            sb.append_int(self.cases.get(i));
                                            sb.append_String(": ");
                                            sb.append_String(Hex::u4(absTarget));
                                            sb.append_String(" // ");
                                            sb.append_String(Hex::s4(relTarget));
                                        }
                                        return sb.toString();
                                    }
                                    pub fn packedCodeSize(cases: &IntList) -> i64                                    {
                                        let sz: i32 = cases.size();
                                        let low: i64 = cases.get(0);
                                        let high: i64 = cases.get(sz-1);
                                        let result: i64 = ((high-low+1))*2+4;
                                        return if (result<=0x7fffffff) { result } else { -1 };
                                            }
                                            pub fn sparseCodeSize(cases: &IntList) -> i64                                            {
                                                let sz: i32 = cases.size();
                                                return (sz*4L)+2;
                                            }
                                            pub fn shouldPack(cases: &IntList) -> boolean                                            {
                                                let sz: i32 = cases.size();
                                                if sz<2                                                {
                                                    return true;
                                                }                                                
                                                let packedSize: i64 = SwitchData::packedCodeSize(&cases);
                                                let sparseSize: i64 = SwitchData::sparseCodeSize(&cases);
                                                return (packedSize>=0)&&(packedSize<=((sparseSize*5)/4));
                                            }
}
