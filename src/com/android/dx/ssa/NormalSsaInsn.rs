use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;

struct NormalSsaInsn{
    pub insn: Insn,
}
impl NormalSsaInsn{
    pub fn new(&self, insn: &Insn, block: &SsaBasicBlock)    {
        super(insn.getResult(),block);

        self->insn=insn;
    }
    pub fn mapSourceRegisters(&self, mapper: &RegisterMapper)    {
        let oldSources: RegisterSpecList = self.insn.getSources();
        let newSources: RegisterSpecList = mapper.map_RegisterSpecList(&oldSources);
        if newSources!=oldSources        {
            self.insn=self.insn.withNewRegisters(getResult(), &newSources);
            getBlock().getParent().onSourcesChanged(self, &oldSources);
        }        
    }
    pub fn changeOneSource(&self, index: i32, newSpec: &RegisterSpec)    {
        let origSources: RegisterSpecList = self.insn.getSources();
        let sz: i32 = origSources.size();
        let newSources: RegisterSpecList = RegisterSpecList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            newSources.set_int_RegisterSpec(i, if i==index { newSpec } else { origSources.get(i) });
                }
                newSources.setImmutable();
                let origSpec: RegisterSpec = origSources.get(index);
                if origSpec.getReg()!=newSpec.getReg()                {
                    getBlock().getParent().onSourceChanged(self, &origSpec, &newSpec);
                }                
                self.insn=self.insn.withNewRegisters(getResult(), &newSources);
            }
            pub fn setNewSources(&self, newSources: &RegisterSpecList)            {
                let origSources: RegisterSpecList = self.insn.getSources();
                if origSources.size()!=newSources.size()                {
                    throw RuntimeException::new("Sources counts don't match");
                }                
                self.insn=self.insn.withNewRegisters(getResult(), &newSources);
            }
            pub fn clone(&self) -> NormalSsaInsn            {
                return (NormalSsaInsn*)super.clone();
            }
            pub fn getSources(&self) -> RegisterSpecList            {
                return self.insn.getSources();
            }
            pub fn toHuman(&self) -> String            {
                return toRopInsn().toHuman();
            }
            pub fn toRopInsn(&self) -> Insn            {
                return self.insn.withNewRegisters(getResult(), self.insn.getSources());
            }
            pub fn getOpcode(&self) -> Rop            {
                return self.insn.getOpcode();
            }
            pub fn getOriginalRopInsn(&self) -> Insn            {
                return self.insn;
            }
            pub fn getLocalAssignment(&self) -> RegisterSpec            {
                let assignment: RegisterSpec;
                if self.insn.getOpcode().getOpcode()==RegOps::MARK_LOCAL                {
                    assignment=self.insn.getSources().get(0);
                }                else                 {
                    assignment=getResult();
                }
                if assignment==None                {
                    return None;
                }                
                let local: LocalItem = assignment.getLocalItem();
                if local==None                {
                    return None;
                }                
                return assignment;
            }
            pub fn upgradeToLiteral(&self)            {
                let oldSources: RegisterSpecList = self.insn.getSources();
                self.insn=self.insn.withSourceLiteral();
                getBlock().getParent().onSourcesChanged(self, &oldSources);
            }
            pub fn isNormalMoveInsn(&self) -> boolean            {
                return self.insn.getOpcode().getOpcode()==RegOps::MOVE;
            }
            pub fn isMoveException(&self) -> boolean            {
                return self.insn.getOpcode().getOpcode()==RegOps::MOVE_EXCEPTION;
            }
            pub fn canThrow(&self) -> boolean            {
                return self.insn.canThrow();
            }
            pub fn accept(&self, v: &Visitor)            {
                if isNormalMoveInsn()                {
                    v.visitMoveInsn(self);
                }                else                 {
                    v.visitNonMoveInsn(self);
                }
            }
            pub fn isPhiOrMove(&self) -> boolean            {
                return isNormalMoveInsn();
            }
            pub fn hasSideEffect(&self) -> boolean            {
                let opcode: Rop = getOpcode();
                if opcode.getBranchingness()!=Rop::BRANCH_NONE                {
                    return true;
                }                
                let hasLocalSideEffect: boolean = Optimizer::getPreserveLocals()&&getLocalAssignment()!=None;
                match opcode.getOpcode(){RegOps::MOVE_RESULT => RegOps::MOVE => RegOps::CONST =>                     return hasLocalSideEffect;                _ => {}                return true;            }
        }
}
