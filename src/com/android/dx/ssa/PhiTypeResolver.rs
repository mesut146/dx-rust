use crate::helper;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::ssa::PhiTypeResolver;
use crate::com::android::dx::cf::code::Merger;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::type::Type;

struct PhiTypeResolver{
    pub ssaMeth: SsaMethod,
    pub worklist: BitSet,
}
impl PhiTypeResolver{
    pub fn process(ssaMeth: &SsaMethod)    {
        PhiTypeResolver::new(&ssaMeth).run();
    }
    pub fn new(&self, ssaMeth: &SsaMethod)    {
        self->ssaMeth=ssaMeth;
        self.worklist=BitSet::new(ssaMeth.getRegCount());
    }
    pub fn run(&self)    {
        let regCount: i32 = self.ssaMeth.getRegCount();
        for(        let reg: i32 = 0;;reg<regCountreg += 1)        {
            let definsn: SsaInsn = self.ssaMeth.getDefinitionForRegister(reg);
            if definsn!=None&&(definsn.getResult().getBasicType()==Type::BT_VOID)            {
                self.worklist.set_int(reg);
            }            
        }
        let reg: i32;
        while 0<=(reg=self.worklist.nextSetBit_int(0))        {
            self.worklist.clear_int(reg);
            let definsn: PhiInsn = (PhiInsn*)self.ssaMeth.getDefinitionForRegister(reg);
            if resolveResultType(&definsn)            {
                let useList: List<SsaInsn> = self.ssaMeth.getUseListForRegister(reg);
                let sz: i32 = useList.size();
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let useInsn: SsaInsn = useList.get(i);
                    let resultReg: RegisterSpec = useInsn.getResult();
                    if resultReg!=None&&//useInsn instanceof PhiInsn                    {
                        self.worklist.set_int(resultReg.getReg());
                    }                    
                }
            }            
        }
    }
    pub fn equalsHandlesNulls(a: &LocalItem, b: &LocalItem) -> boolean    {
        return (a==b)||((a!=None)&&a.equals(&b));
    }
    pub fn resolveResultType(&self, insn: &PhiInsn) -> boolean    {
        insn.updateSourcesToDefinitions(&self.ssaMeth);
        let sources: RegisterSpecList = insn.getSources();
        let first: RegisterSpec = None;
        let firstIndex: i32 = -1;
        let szSources: i32 = sources.size();
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            let rs: RegisterSpec = sources.get(i);
            if rs.getBasicType()!=Type::BT_VOID            {
                first=rs;
                firstIndex=i;
            }            
        }
        if first==None        {
            return false;
        }        
        let firstLocal: LocalItem = first.getLocalItem();
        let mergedType: TypeBearer = first.getType();
        let sameLocals: boolean = true;
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            if i==firstIndex            {
                continue;
            }            
            let rs: RegisterSpec = sources.get(i);
            if rs.getBasicType()==Type::BT_VOID            {
                continue;
            }            
            sameLocals=sameLocals&&PhiTypeResolver::equalsHandlesNulls(&firstLocal, rs.getLocalItem());
            mergedType=Merger::mergeType(&mergedType, rs.getType());
        }
        let newResultType: TypeBearer;
        if mergedType!=None        {
            newResultType=mergedType;
        }        else         {
            let sb: StringBuilder = StringBuilder::new();
            for(            let i: i32 = 0;;i<szSourcesi += 1)            {
                sb.append_String(sources.get(i).toString());
                sb.append_char(' ');
            }
            throw RuntimeException::new("Couldn't map types in phi insn:"+sb);
        }
        let newLocal: LocalItem = if sameLocals { firstLocal } else { None };
                let result: RegisterSpec = insn.getResult();
                if (result.getTypeBearer()==newResultType)&&PhiTypeResolver::equalsHandlesNulls(&newLocal, result.getLocalItem())                {
                    return false;
                }                
                insn.changeResultType(&newResultType, &newLocal);
                return true;
            }
}
