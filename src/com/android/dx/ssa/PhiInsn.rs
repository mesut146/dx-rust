use crate::helper;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::ssa::PhiInsn::Operand;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::type::Type;

struct PhiInsn{
    pub ropResultReg: i32,
    pub operands: ArrayList<Operand>,
    pub sources: RegisterSpecList,
}
impl PhiInsn{
    pub fn new(&self, resultReg: &RegisterSpec, block: &SsaBasicBlock)    {
        super(resultReg,block);

        self.ropResultReg=resultReg.getReg();
    }
    pub fn new(&self, resultReg: i32, block: &SsaBasicBlock)    {
        super(RegisterSpec.make(resultReg,Type.VOID),block);

        self.ropResultReg=resultReg;
    }
    pub fn clone(&self) -> PhiInsn    {
        throw UnsupportedOperationException::new("can't clone phi");
    }
    pub fn updateSourcesToDefinitions(&self, ssaMeth: &SsaMethod)    {
        for o in self.operands        {
            let def: RegisterSpec = ssaMeth.getDefinitionForRegister(.getReg()).getResult();
            =.withType(def.getType());
        }
        self.sources=None;
    }
    pub fn changeResultType(&self, type: &TypeBearer, local: &LocalItem)    {
        setResult(RegisterSpec::makeLocalOptional(getResult().getReg(), &type, &local));
    }
    pub fn getRopResultReg(&self) -> i32    {
        return self.ropResultReg;
    }
    pub fn addPhiOperand(&self, registerSpec: &RegisterSpec, predBlock: &SsaBasicBlock)    {
        self.operands.add_Operand(Operand::new(&registerSpec, predBlock.getIndex(), predBlock.getRopLabel()));
        self.sources=None;
    }
    pub fn removePhiRegister(&self, registerSpec: &RegisterSpec)    {
        let operandsToRemove: ArrayList<Operand> = ArrayList<Operand>::new();
        for o in self.operands        {
            if .getReg()==registerSpec.getReg()            {
                operandsToRemove.add_Operand(&o);
            }            
        }
        self.operands.removeAll(&operandsToRemove);
        self.sources=None;
    }
    pub fn predBlockIndexForSourcesIndex(&self, sourcesIndex: i32) -> i32    {
        return self.operands.get(sourcesIndex)->blockIndex;
    }
    pub fn getOpcode(&self) -> Rop    {
        return None;
    }
    pub fn getOriginalRopInsn(&self) -> Insn    {
        return None;
    }
    pub fn canThrow(&self) -> boolean    {
        return false;
    }
    pub fn getSources(&self) -> RegisterSpecList    {
        if self.sources!=None        {
            return self.sources;
        }        
        if self.operands.size()==0        {
            return RegisterSpecList::EMPTY;
        }        
        let szSources: i32 = self.operands.size();
        self.sources=RegisterSpecList::new(szSources);
        for(        let i: i32 = 0;;i<szSourcesi += 1)        {
            let o: Operand = self.operands.get(i);
            self.sources.set_int_RegisterSpec(i, &);
        }
        self.sources.setImmutable();
        return self.sources;
    }
    pub fn isRegASource(&self, reg: i32) -> boolean    {
        for o in self.operands        {
            if .getReg()==reg            {
                return true;
            }            
        }
        return false;
    }
    pub fn areAllOperandsEqual(&self) -> boolean    {
        if self.operands.size()==0        {
            return true;
        }        
        let firstReg: i32 = self.operands.get(0)->regSpec.getReg();
        for o in self.operands        {
            if firstReg!=.getReg()            {
                return false;
            }            
        }
        return true;
    }
    pub fn mapSourceRegisters(&self, mapper: &RegisterMapper)    {
        for o in self.operands        {
            let old: RegisterSpec = ;
            =mapper.map_RegisterSpec(&old);
            if old!=            {
                getBlock().getParent().onSourceChanged(self, &old, &);
            }            
        }
        self.sources=None;
    }
    pub fn toRopInsn(&self) -> Insn    {
        throw IllegalArgumentException::new("Cannot convert phi insns to rop form");
    }
    pub fn predBlocksForReg(&self, reg: i32, ssaMeth: &SsaMethod) -> List<SsaBasicBlock>    {
        let ret: ArrayList<SsaBasicBlock> = ArrayList<SsaBasicBlock>::new();
        for o in self.operands        {
            if .getReg()==reg            {
                ret.add_SsaBasicBlock(ssaMeth.getBlocks().get());
            }            
        }
        return ret;
    }
    pub fn isPhiOrMove(&self) -> boolean    {
        return true;
    }
    pub fn hasSideEffect(&self) -> boolean    {
        return Optimizer::getPreserveLocals()&&getLocalAssignment()!=None;
    }
    pub fn accept(&self, v: &SsaInsn.Visitor)    {
        v.visitPhiInsn(self);
    }
    pub fn toHuman(&self) -> String    {
        return toHumanWithInline(None);
    }
    pub fn toHumanWithInline(&self, extra: &String) -> String    {
        let sb: StringBuilder = StringBuilder::new(80);
        sb.append_Object(&SourcePosition::NO_INFO);
        sb.append_String(": phi");
        if extra!=None        {
            sb.append_String("(");
            sb.append_String(&extra);
            sb.append_String(")");
        }        
        let result: RegisterSpec = getResult();
        if result==None        {
            sb.append_String(" .");
        }        else         {
            sb.append_String(" ");
            sb.append_String(result.toHuman());
        }
        sb.append_String(" <-");
        let sz: i32 = getSources().size();
        if sz==0        {
            sb.append_String(" .");
        }        else         {
            for(            let i: i32 = 0;;i<szi += 1)            {
                sb.append_String(" ");
                sb.append_String(self.sources.get(i).toHuman()+"[b="+Hex::u2(self.operands.get(i)->ropLabel)+"]");
            }
        }
        return sb.toString();
    }
}
