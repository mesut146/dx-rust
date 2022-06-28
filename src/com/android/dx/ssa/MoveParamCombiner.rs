use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::CstInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::ssa::SsaInsn::Visitor;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::ssa::MoveParamCombiner;

struct MoveParamCombiner{
    pub ssaMeth: SsaMethod,
}
impl MoveParamCombiner{
    pub fn process(ssaMethod: &SsaMethod)    {
        MoveParamCombiner::new(&ssaMethod).run();
    }
    pub fn new(&self, ssaMeth: &SsaMethod)    {
        self->ssaMeth=ssaMeth;
    }
    pub fn run(&self)    {
        let paramSpecs: Vec<RegisterSpec> = new RegisterSpec[ssaMeth.getParamWidth()];
        let deletedInsns: HashSet<SsaInsn> = HashSet::new();
        self.ssaMeth.forEachInsn(/*new SsaInsn.Visitor(){
  @Override public void visitMoveInsn(  NormalSsaInsn insn){
  }
  @Override public void visitPhiInsn(  PhiInsn phi){
  }
  @Override public void visitNonMoveInsn(  NormalSsaInsn insn){
    if (insn.getOpcode().getOpcode() != RegOps.MOVE_PARAM) {
      return;
    }
    int param=getParamIndex(insn);
    if (paramSpecs[param] == null) {
      paramSpecs[param]=insn.getResult();
    }
 else {
      final RegisterSpec specA=paramSpecs[param];
      final RegisterSpec specB=insn.getResult();
      LocalItem localA=specA.getLocalItem();
      LocalItem localB=specB.getLocalItem();
      LocalItem newLocal;
      if (localA == null) {
        newLocal=localB;
      }
 else       if (localB == null) {
        newLocal=localA;
      }
 else       if (localA.equals(localB)) {
        newLocal=localA;
      }
 else {
        return;
      }
      ssaMeth.getDefinitionForRegister(specA.getReg()).setResultLocal(newLocal);
      RegisterMapper mapper=new RegisterMapper(){
        /** 
 * {@inheritDoc} 
 */
        @Override public int getNewRegisterCount(){
          return ssaMeth.getRegCount();
        }
        /** 
 * {@inheritDoc} 
 */
        @Override public RegisterSpec map(        RegisterSpec registerSpec){
          if (registerSpec.getReg() == specB.getReg()) {
            return specA;
          }
          return registerSpec;
        }
      }
;
      List<SsaInsn> uses=ssaMeth.getUseListForRegister(specB.getReg());
      for (int i=uses.size() - 1; i >= 0; i--) {
        SsaInsn use=uses.get(i);
        use.mapSourceRegisters(mapper);
      }
      deletedInsns.add(insn);
    }
  }
}
*/);
        self.ssaMeth.deleteInsns(&deletedInsns);
    }
    pub fn getParamIndex(&self, insn: &NormalSsaInsn) -> i32    {
        let cstInsn: CstInsn = (CstInsn*)(insn.getOriginalRopInsn());
        let param: i32 = ((CstInteger*)cstInsn.getConstant()).getValue();
        return param;
    }
}
