use crate::helper;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::Insn::Visitor;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::code::PlainInsn;

struct PlainInsn{
}
impl PlainInsn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, result: &RegisterSpec, sources: &RegisterSpecList)    {
        super(opcode,position,result,sources);

        match opcode.getBranchingness(){Rop::BRANCH_SWITCH => Rop::BRANCH_THROW =>             {
                throw IllegalArgumentException::new("opcode with invalid branchingness: "+opcode.getBranchingness());
            }        }
        if result!=None&&opcode.getBranchingness()!=Rop::BRANCH_NONE        {
            throw IllegalArgumentException::new("can't mix branchingness with result");
        }        
    }
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, result: &RegisterSpec, source: &RegisterSpec)    {
        this(opcode,position,result,RegisterSpecList.make(source));

    }
    pub fn getCatches(&self) -> TypeList    {
        return StdTypeList::EMPTY;
    }
    pub fn accept(&self, visitor: &Visitor)    {
        visitor.visitPlainInsn(self);
    }
    pub fn withAddedCatch(&self, type: &Type) -> Insn    {
        throw UnsupportedOperationException::new("unsupported");
    }
    pub fn withRegisterOffset(&self, delta: i32) -> Insn    {
        return PlainInsn::new(getOpcode(), getPosition(), getResult().withOffset(delta), getSources().withOffset(delta));
    }
    pub fn withSourceLiteral(&self) -> Insn    {
        let sources: RegisterSpecList = getSources();
        let szSources: i32 = sources.size();
        if szSources==0        {
            return self;
        }        
        let lastType: TypeBearer = sources.get(szSources-1).getTypeBearer();
        if !lastType.isConstant()        {
            let firstType: TypeBearer = sources.get(0).getTypeBearer();
            if szSources==2&&firstType.isConstant()            {
                let cst: Constant = (Constant*)firstType;
                let newSources: RegisterSpecList = sources.withoutFirst();
                let newRop: Rop = Rops::ropFor(getOpcode().getOpcode(), getResult(), &newSources, &cst);
                return PlainCstInsn::new(&newRop, getPosition(), getResult(), &newSources, &cst);
            }            
            return self;
        }        else         {
            let cst: Constant = (Constant*)lastType;
            let newSources: RegisterSpecList = sources.withoutLast();
            let newRop: Rop;
            try            {
                let opcode: i32 = getOpcode().getOpcode();
                if opcode==RegOps::SUB&&//cst instanceof CstInteger                {
                    opcode=RegOps::ADD;
                    cst=CstInteger::make(-((CstInteger*)cst).getValue());
                }                
                newRop=Rops::ropFor(opcode, getResult(), &newSources, &cst);
            }            catch(            let ex: IllegalArgumentException)            {
                return self;
            }
            return PlainCstInsn::new(&newRop, getPosition(), getResult(), &newSources, &cst);
        }
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn    {
        return PlainInsn::new(getOpcode(), getPosition(), &result, &sources);
    }
}
