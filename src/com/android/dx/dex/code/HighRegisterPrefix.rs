use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::HighRegisterPrefix;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::dex::code::SimpleInsn;

struct HighRegisterPrefix{
    pub insns: Vec<SimpleInsn>,
}
impl HighRegisterPrefix{
    pub fn new(&self, position: &SourcePosition, registers: &RegisterSpecList)    {
        super(position,registers);

        if registers.size()==0        {
            throw IllegalArgumentException::new("registers.size() == 0");
        }        
        self.insns=None;
    }
    pub fn codeSize(&self) -> i32    {
        let result: i32 = 0;
        calculateInsnsIfNecessary();
        for insn in self.insns        {
            result+=insn.codeSize();
        }
        return result;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
        calculateInsnsIfNecessary();
        for insn in self.insns        {
            insn.writeTo(&out);
        }
    }
    pub fn calculateInsnsIfNecessary(&self)    {
        if self.insns!=None        {
            return;
        }        
        let registers: RegisterSpecList = getRegisters();
        let sz: i32 = registers.size();
        self.insns=new SimpleInsn[sz];
        for(        let i: i32 = 0;        let outAt: i32 = 0;;i<szi += 1)        {
            let src: RegisterSpec = registers.get(i);
            self.insns[i]=HighRegisterPrefix::moveInsnFor(&src, outAt);
            outAt+=src.getCategory();
        }
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return HighRegisterPrefix::new(getPosition(), &registers);
    }
    pub fn argString(&self) -> String    {
        return None;
    }
    pub fn listingString0(&self, noteIndices: boolean) -> String    {
        let registers: RegisterSpecList = getRegisters();
        let sz: i32 = registers.size();
        let sb: StringBuilder = StringBuilder::new(100);
        for(        let i: i32 = 0;        let outAt: i32 = 0;;i<szi += 1)        {
            let src: RegisterSpec = registers.get(i);
            let insn: SimpleInsn = HighRegisterPrefix::moveInsnFor(&src, outAt);
            if i!=0            {
                sb.append_char('\n');
            }            
            sb.append_String(insn.listingString0(noteIndices));
            outAt+=src.getCategory();
        }
        return sb.toString();
    }
    pub fn moveInsnFor(src: &RegisterSpec, destIndex: i32) -> SimpleInsn    {
        return DalvInsn::makeMove(&SourcePosition::NO_INFO, RegisterSpec::make_int_TypeBearer(destIndex, src.getType()), &src);
    }
}
