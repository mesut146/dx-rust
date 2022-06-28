use crate::helper;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::dex::cf::CfOptions;
use crate::com::android::dx::ssa::Optimizer::OptionalStep;
use crate::com::android::dx::rop::code::RopMethod;

struct OptimizerOptions{
    pub optimizeList: HashSet<String>,
    pub dontOptimizeList: HashSet<String>,
    pub optimizeListsLoaded: boolean,
}
impl OptimizerOptions{
    pub fn loadOptimizeLists(&self, optimizeListFile: &String, dontOptimizeListFile: &String)    {
        if self.optimizeListsLoaded        {
            return;
        }        
        if optimizeListFile!=None&&dontOptimizeListFile!=None        {
            throw RuntimeException::new("optimize and don't optimize lists "+" are mutually exclusive.");
        }        
        if optimizeListFile!=None        {
            self.optimizeList=OptimizerOptions::loadStringsFromFile(&optimizeListFile);
        }        
        if dontOptimizeListFile!=None        {
            self.dontOptimizeList=OptimizerOptions::loadStringsFromFile(&dontOptimizeListFile);
        }        
        self.optimizeListsLoaded=true;
    }
    pub fn loadStringsFromFile(filename: &String) -> HashSet<String>    {
        let result: HashSet<String> = HashSet<String>::new();
        try        {
            let fr: FileReader = FileReader::new(&filename);
            let bfr: BufferedReader = BufferedReader::new(&fr);
            let line: String;
            while None!=(line=bfr.readLine())            {
                result.add(&line);
            }
            fr.close();
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new("Error with optimize list: "+filename, &ex);
        }
        return result;
    }
    pub fn compareOptimizerStep(&self, nonOptRmeth: &RopMethod, paramSize: i32, isStatic: boolean, args: &CfOptions, advice: &TranslationAdvice, rmeth: &RopMethod)    {
        let steps: EnumSet<OptionalStep>;
        steps=EnumSet::allOf(Optimizer.OptionalStep.classOptimizer::OptionalStep);
        steps.remove(&OptionalStep::CONST_COLLECTOR);
        let skipRopMethod: RopMethod = Optimizer::optimize_RopMethod_int_boolean_boolean_TranslationAdvice_EnumSet<OptionalStep>(&nonOptRmeth, paramSize, isStatic, , &advice, &steps);
        let normalInsns: i32 = rmeth.getBlocks().getEffectiveInstructionCount();
        let skipInsns: i32 = skipRopMethod.getBlocks().getEffectiveInstructionCount();
        System::err.printf_String_Object[]("optimize step regs:(%d/%d/%.2f%%)"+" insns:(%d/%d/%.2f%%)\n", rmeth.getBlocks().getRegCount(), skipRopMethod.getBlocks().getRegCount(), 100.0*((skipRopMethod.getBlocks().getRegCount()-rmeth.getBlocks().getRegCount())/skipRopMethod.getBlocks().getRegCount() as f32), normalInsns, skipInsns, 100.0*((skipInsns-normalInsns)/skipInsns as f32));
    }
    pub fn shouldOptimize(&self, canonicalMethodName: &String) -> boolean    {
        if self.optimizeList!=None        {
            return self.optimizeList.contains(&canonicalMethodName);
        }        
        if self.dontOptimizeList!=None        {
            return !self.dontOptimizeList.contains(&canonicalMethodName);
        }        
        return true;
    }
}
