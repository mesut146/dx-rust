use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::DexTranslationAdvice;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::Rop;

let static THE_ONE: DexTranslationAdvice = DexTranslationAdvice::new();
let static NO_SOURCES_IN_ORDER: DexTranslationAdvice = DexTranslationAdvice::new(true);
struct DexTranslationAdvice{
    pub disableSourcesInOrder: boolean,
}
impl DexTranslationAdvice{
    pub const MIN_INVOKE_IN_ORDER: i32 = 6;
    pub fn new(&self)    {
        self.disableSourcesInOrder=false;
    }
    pub fn new(&self, disableInvokeRange: boolean)    {
        self->disableSourcesInOrder=disableInvokeRange;
    }
    pub fn hasConstantOperation(&self, opcode: &Rop, sourceA: &RegisterSpec, sourceB: &RegisterSpec) -> boolean    {
        if sourceA.getType()!=Type::INT        {
            return false;
        }        
        if !(//sourceB.getTypeBearer() instanceof CstInteger)        {
            if //sourceA.getTypeBearer() instanceof CstInteger&&opcode.getOpcode()==RegOps::SUB            {
                let cst: CstInteger = (CstInteger*)sourceA.getTypeBearer();
                return cst.fitsIn16Bits();
            }            else             {
                return false;
            }
        }        
        let cst: CstInteger = (CstInteger*)sourceB.getTypeBearer();
        match opcode.getOpcode(){RegOps::REM => RegOps::ADD => RegOps::MUL => RegOps::DIV => RegOps::AND => RegOps::OR => RegOps::XOR =>             return cst.fitsIn16Bits();RegOps::SHL => RegOps::SHR => RegOps::USHR =>             return cst.fitsIn8Bits();RegOps::SUB =>             let cst2: CstInteger = CstInteger::make(-cst.getValue());            return cst2.fitsIn16Bits();        _ => {}        return false;    }
}
pub fn requiresSourcesInOrder(&self, opcode: &Rop, sources: &RegisterSpecList) -> boolean{
    return !self.disableSourcesInOrder&&opcode.isCallLike()&&totalRopWidth(&sources)>=DexTranslationAdvice::MIN_INVOKE_IN_ORDER;
}
pub fn totalRopWidth(&self, sources: &RegisterSpecList) -> i32{
    let sz: i32 = sources.size();
    let total: i32 = 0;
    for(    let i: i32 = 0;;i<szi += 1)    {
        total+=sources.get(i).getCategory();
    }
    return total;
}
pub fn getMaxOptimalRegisterCount(&self) -> i32{
    return 16;
}
}
