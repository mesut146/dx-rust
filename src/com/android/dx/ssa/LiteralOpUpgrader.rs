use crate::helper;
use crate::com::android::dx::ssa::LiteralOpUpgrader;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::code::TranslationAdvice;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::rop::type::Type;

struct LiteralOpUpgrader{
    pub ssaMeth: SsaMethod,
}
impl LiteralOpUpgrader{
    pub fn process(ssaMethod: &SsaMethod)    {
        let dc: LiteralOpUpgrader;
        dc=LiteralOpUpgrader::new(&ssaMethod);
        dc.run();
    }
    pub fn new(&self, ssaMethod: &SsaMethod)    {
        self->ssaMeth=ssaMethod;
    }
    pub fn isConstIntZeroOrKnownNull(spec: &RegisterSpec) -> boolean    {
        let tb: TypeBearer = spec.getTypeBearer();
        if //tb instanceof CstLiteralBits        {
            let clb: CstLiteralBits = (CstLiteralBits*)tb;
            return (clb.getLongBits()==0);
        }        
        return false;
    }
    pub fn run(&self)    {
        let advice: TranslationAdvice = Optimizer::getAdvice();
        self.ssaMeth.forEachInsn(/*new SsaInsn.Visitor(){
  @Override public void visitMoveInsn(  NormalSsaInsn insn){
  }
  @Override public void visitPhiInsn(  PhiInsn insn){
  }
  @Override public void visitNonMoveInsn(  NormalSsaInsn insn){
    Insn originalRopInsn=insn.getOriginalRopInsn();
    Rop opcode=originalRopInsn.getOpcode();
    RegisterSpecList sources=insn.getSources();
    if (tryReplacingWithConstant(insn))     return;
    if (sources.size() != 2) {
      return;
    }
    if (opcode.getBranchingness() == Rop.BRANCH_IF) {
      if (isConstIntZeroOrKnownNull(sources.get(0))) {
        replacePlainInsn(insn,sources.withoutFirst(),RegOps.flippedIfOpcode(opcode.getOpcode()),null);
      }
 else       if (isConstIntZeroOrKnownNull(sources.get(1))) {
        replacePlainInsn(insn,sources.withoutLast(),opcode.getOpcode(),null);
      }
    }
 else     if (advice.hasConstantOperation(opcode,sources.get(0),sources.get(1))) {
      insn.upgradeToLiteral();
    }
 else     if (opcode.isCommutative() && advice.hasConstantOperation(opcode,sources.get(1),sources.get(0))) {
      insn.setNewSources(RegisterSpecList.make(sources.get(1),sources.get(0)));
      insn.upgradeToLiteral();
    }
  }
}
*/);
    }
    pub fn tryReplacingWithConstant(&self, insn: &NormalSsaInsn) -> boolean    {
        let originalRopInsn: Insn = insn.getOriginalRopInsn();
        let opcode: Rop = originalRopInsn.getOpcode();
        let result: RegisterSpec = insn.getResult();
        if result!=None&&!self.ssaMeth.isRegALocal(&result)&&opcode.getOpcode()!=RegOps::CONST        {
            let type: TypeBearer = insn.getResult().getTypeBearer();
            if type_renamed.isConstant()&&type_renamed.getBasicType()==Type::BT_INT            {
                replacePlainInsn(&insn, &RegisterSpecList::EMPTY, RegOps::CONST, (Constant*)type_renamed);
                if opcode.getOpcode()==RegOps::MOVE_RESULT_PSEUDO                {
                    let pred: i32 = insn.getBlock().getPredecessors().nextSetBit_int(0);
                    let predInsns: ArrayList<SsaInsn> = self.ssaMeth.getBlocks().get(pred).getInsns();
                    let sourceInsn: NormalSsaInsn = (NormalSsaInsn*)predInsns.get(predInsns.size()-1);
                    replacePlainInsn(&sourceInsn, &RegisterSpecList::EMPTY, RegOps::GOTO, None);
                }                
                return true;
            }            
        }        
        return false;
    }
    pub fn replacePlainInsn(&self, insn: &NormalSsaInsn, newSources: &RegisterSpecList, newOpcode: i32, cst: &Constant)    {
        let originalRopInsn: Insn = insn.getOriginalRopInsn();
        let newRop: Rop = Rops::ropFor(newOpcode, insn.getResult(), &newSources, &cst);
        let newRopInsn: Insn;
        if cst==None        {
            newRopInsn=PlainInsn::new(&newRop, originalRopInsn.getPosition(), insn.getResult(), &newSources);
        }        else         {
            newRopInsn=PlainCstInsn::new(&newRop, originalRopInsn.getPosition(), insn.getResult(), &newSources, &cst);
        }
        let newInsn: NormalSsaInsn = NormalSsaInsn::new(&newRopInsn, insn.getBlock());
        let insns: List<SsaInsn> = insn.getBlock().getInsns();
        self.ssaMeth.onInsnRemoved(&insn);
        insns.set(insns.lastIndexOf(&insn), &newInsn);
        self.ssaMeth.onInsnAdded(&newInsn);
    }
}
