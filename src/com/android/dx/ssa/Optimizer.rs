use crate::helper;
use crate::com::android::dx::ssa::LiteralOpUpgrader;
use crate::com::android::dx::ssa::back::SsaToRop;
use crate::com::android::dx::ssa::PhiTypeResolver;
use crate::com::android::dx::ssa::SsaConverter;
use crate::com::android::dx::ssa::back::LivenessAnalyzer;
use crate::com::android::dx::ssa::DeadCodeRemover;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::ssa::SCCP;
use crate::com::android::dx::ssa::EscapeAnalysis;
use crate::com::android::dx::ssa::ConstCollector;
use crate::com::android::dx::rop::code::TranslationAdvice;
use crate::com::android::dx::ssa::Optimizer::OptionalStep;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::ssa::MoveParamCombiner;

let static preserveLocals: boolean = true;
let static advice: Option<TranslationAdvice> = Node;
struct Optimizer{
}
impl Optimizer{
    pub fn getPreserveLocals() -> boolean    {
        return Optimizer::preserveLocals;
    }
    pub fn getAdvice() -> TranslationAdvice    {
        return Optimizer::advice;
    }
    pub fn optimize(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice) -> RopMethod    {
        return Optimizer::optimize_RopMethod_int_boolean_boolean_TranslationAdvice_EnumSet<OptionalStep>(&rmeth, paramWidth, isStatic, inPreserveLocals, &inAdvice, EnumSet::allOf(OptionalStep.classOptionalStep));
    }
    pub fn optimize(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice, steps: &EnumSet<OptionalStep>) -> RopMethod    {
        let ssaMeth: SsaMethod = None;
        Optimizer::preserveLocals=inPreserveLocals;
        Optimizer::advice=inAdvice;
        ssaMeth=SsaConverter::convertToSsaMethod(&rmeth, paramWidth, isStatic);
        Optimizer::runSsaFormSteps(&ssaMeth, &steps);
        let resultMeth: RopMethod = SsaToRop::convertToRopMethod(&ssaMeth, false);
        if resultMeth.getBlocks().getRegCount()>Optimizer::advice.getMaxOptimalRegisterCount()        {
            resultMeth=Optimizer::optimizeMinimizeRegisters(&rmeth, paramWidth, isStatic, &steps);
        }        
        return resultMeth;
    }
    pub fn optimizeMinimizeRegisters(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, steps: &EnumSet<OptionalStep>) -> RopMethod    {
        let ssaMeth: SsaMethod;
        let resultMeth: RopMethod;
        ssaMeth=SsaConverter::convertToSsaMethod(&rmeth, paramWidth, isStatic);
        let newSteps: EnumSet<OptionalStep> = steps.clone();
        newSteps.remove(&OptionalStep::CONST_COLLECTOR);
        Optimizer::runSsaFormSteps(&ssaMeth, &newSteps);
        resultMeth=SsaToRop::convertToRopMethod(&ssaMeth, true);
        return resultMeth;
    }
    pub fn runSsaFormSteps(ssaMeth: &SsaMethod, steps: &EnumSet<OptionalStep>)    {
        let needsDeadCodeRemover: boolean = true;
        if steps.contains(&OptionalStep::MOVE_PARAM_COMBINER)        {
            MoveParamCombiner::process(&ssaMeth);
        }        
        if steps.contains(&OptionalStep::SCCP)        {
            SCCP::process(&ssaMeth);
            DeadCodeRemover::process(&ssaMeth);
            needsDeadCodeRemover=false;
        }        
        if steps.contains(&OptionalStep::LITERAL_UPGRADE)        {
            LiteralOpUpgrader::process(&ssaMeth);
            DeadCodeRemover::process(&ssaMeth);
            needsDeadCodeRemover=false;
        }        
        steps.remove(&OptionalStep::ESCAPE_ANALYSIS);
        if steps.contains(&OptionalStep::ESCAPE_ANALYSIS)        {
            EscapeAnalysis::process(&ssaMeth);
            DeadCodeRemover::process(&ssaMeth);
            needsDeadCodeRemover=false;
        }        
        if steps.contains(&OptionalStep::CONST_COLLECTOR)        {
            ConstCollector::process(&ssaMeth);
            DeadCodeRemover::process(&ssaMeth);
            needsDeadCodeRemover=false;
        }        
        if needsDeadCodeRemover        {
            DeadCodeRemover::process(&ssaMeth);
        }        
        PhiTypeResolver::process(&ssaMeth);
    }
    pub fn debugEdgeSplit(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice) -> SsaMethod    {
        Optimizer::preserveLocals=inPreserveLocals;
        Optimizer::advice=inAdvice;
        return SsaConverter::testEdgeSplit(&rmeth, paramWidth, isStatic);
    }
    pub fn debugPhiPlacement(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice) -> SsaMethod    {
        Optimizer::preserveLocals=inPreserveLocals;
        Optimizer::advice=inAdvice;
        return SsaConverter::testPhiPlacement(&rmeth, paramWidth, isStatic);
    }
    pub fn debugRenaming(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice) -> SsaMethod    {
        Optimizer::preserveLocals=inPreserveLocals;
        Optimizer::advice=inAdvice;
        return SsaConverter::convertToSsaMethod(&rmeth, paramWidth, isStatic);
    }
    pub fn debugDeadCodeRemover(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice) -> SsaMethod    {
        let ssaMeth: SsaMethod;
        Optimizer::preserveLocals=inPreserveLocals;
        Optimizer::advice=inAdvice;
        ssaMeth=SsaConverter::convertToSsaMethod(&rmeth, paramWidth, isStatic);
        DeadCodeRemover::process(&ssaMeth);
        return ssaMeth;
    }
    pub fn debugNoRegisterAllocation(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean, inPreserveLocals: boolean, inAdvice: &TranslationAdvice, steps: &EnumSet<OptionalStep>) -> SsaMethod    {
        let ssaMeth: SsaMethod;
        Optimizer::preserveLocals=inPreserveLocals;
        Optimizer::advice=inAdvice;
        ssaMeth=SsaConverter::convertToSsaMethod(&rmeth, paramWidth, isStatic);
        Optimizer::runSsaFormSteps(&ssaMeth, &steps);
        LivenessAnalyzer::constructInterferenceGraph(&ssaMeth);
        return ssaMeth;
    }
}
