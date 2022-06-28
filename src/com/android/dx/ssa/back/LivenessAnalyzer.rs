use crate::helper;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::ssa::back::LivenessAnalyzer::NextFunction;
use crate::com::android::dx::ssa::back::LivenessAnalyzer;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::ssa::back::InterferenceGraph;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;

struct LivenessAnalyzer{
    pub visitedBlocks: BitSet,
    pub liveOutBlocks: BitSet,
    pub regV: i32,
    pub ssaMeth: SsaMethod,
    pub interference: InterferenceGraph,
    pub blockN: SsaBasicBlock,
    pub statementIndex: i32,
    pub nextFunction: NextFunction,
}
impl LivenessAnalyzer{
    pub fn constructInterferenceGraph(ssaMeth: &SsaMethod) -> InterferenceGraph    {
        let szRegs: i32 = ssaMeth.getRegCount();
        let interference: InterferenceGraph = InterferenceGraph::new(szRegs);
        for(        let i: i32 = 0;;i<szRegsi += 1)        {
            LivenessAnalyzer::new(&ssaMeth, i, &interference).run();
        }
        LivenessAnalyzer::coInterferePhis(&ssaMeth, &interference);
        return interference;
    }
    pub fn new(&self, ssaMeth: &SsaMethod, reg: i32, interference: &InterferenceGraph)    {
        let blocksSz: i32 = ssaMeth.getBlocks().size();
        self->ssaMeth=ssaMeth;
        self->regV=reg;
        self.visitedBlocks=BitSet::new(blocksSz);
        self.liveOutBlocks=BitSet::new(blocksSz);
        self->interference=interference;
    }
    pub fn handleTailRecursion(&self)    {
        while self.nextFunction!=NextFunction::DONE        {
            match self.nextFunction{NextFunction::LIVE_IN_AT_STATEMENT =>                 self.nextFunction=NextFunction::DONE;                liveInAtStatement();                break;NextFunction::LIVE_OUT_AT_STATEMENT =>                 self.nextFunction=NextFunction::DONE;                liveOutAtStatement();                break;NextFunction::LIVE_OUT_AT_BLOCK =>                 self.nextFunction=NextFunction::DONE;                liveOutAtBlock();                break;            _ => {}        }
    }
}
pub fn run(&self){
    let useList: List<SsaInsn> = self.ssaMeth.getUseListForRegister(self.regV);
    for insn in useList    {
        self.nextFunction=NextFunction::DONE;
        if //insn instanceof PhiInsn        {
            let phi: PhiInsn = (PhiInsn*)insn;
            for pred in phi.predBlocksForReg(self.regV, &self.ssaMeth)            {
                self.blockN=pred;
                self.nextFunction=NextFunction::LIVE_OUT_AT_BLOCK;
                handleTailRecursion();
            }
        }        else         {
            self.blockN=insn.getBlock();
            self.statementIndex=self.blockN.getInsns().indexOf(&insn);
            if self.statementIndex<0            {
                throw RuntimeException::new("insn not found in it's own block");
            }            
            self.nextFunction=NextFunction::LIVE_IN_AT_STATEMENT;
            handleTailRecursion();
        }
    }
    let nextLiveOutBlock: i32;
    while (nextLiveOutBlock=self.liveOutBlocks.nextSetBit_int(0))>=0    {
        self.blockN=self.ssaMeth.getBlocks().get(nextLiveOutBlock);
        self.liveOutBlocks.clear_int(nextLiveOutBlock);
        self.nextFunction=NextFunction::LIVE_OUT_AT_BLOCK;
        handleTailRecursion();
    }
}
pub fn liveOutAtBlock(&self){
    if !self.visitedBlocks.get_int(self.blockN.getIndex())    {
        self.visitedBlocks.set_int(self.blockN.getIndex());
        self.blockN.addLiveOut(self.regV);
        let insns: ArrayList<SsaInsn>;
        insns=self.blockN.getInsns();
        self.statementIndex=insns.size()-1;
        self.nextFunction=NextFunction::LIVE_OUT_AT_STATEMENT;
    }    
}
pub fn liveInAtStatement(&self){
    if self.statementIndex==0    {
        self.blockN.addLiveIn(self.regV);
        let preds: BitSet = self.blockN.getPredecessors();
        self.liveOutBlocks.or_BitSet(&preds);
    }    else     {
        self.statementIndex-=1;
        self.nextFunction=NextFunction::LIVE_OUT_AT_STATEMENT;
    }
}
pub fn liveOutAtStatement(&self){
    let statement: SsaInsn = self.blockN.getInsns().get(self.statementIndex);
    let rs: RegisterSpec = statement.getResult();
    if !statement.isResultReg(self.regV)    {
        if rs!=None        {
            self.interference.add(self.regV, rs.getReg());
        }        
        self.nextFunction=NextFunction::LIVE_IN_AT_STATEMENT;
    }    
}
pub fn coInterferePhis(ssaMeth: &SsaMethod, interference: &InterferenceGraph){
    for b in ssaMeth.getBlocks()    {
        let phis: List<SsaInsn> = b.getPhiInsns();
        let szPhis: i32 = phis.size();
        for(        let i: i32 = 0;;i<szPhisi += 1)        {
            for(            let j: i32 = 0;;j<szPhisj += 1)            {
                if i==j                {
                    continue;
                }                
                let first: SsaInsn = phis.get(i);
                let second: SsaInsn = phis.get(j);
                LivenessAnalyzer::coInterferePhiRegisters(&interference, first.getResult(), second.getSources());
                LivenessAnalyzer::coInterferePhiRegisters(&interference, second.getResult(), first.getSources());
                interference.add(first.getResult().getReg(), second.getResult().getReg());
            }
        }
    }
}
pub fn coInterferePhiRegisters(interference: &InterferenceGraph, result: &RegisterSpec, sources: &RegisterSpecList){
    let resultReg: i32 = result.getReg();
    for(    let i: i32 = 0;;i<sources.size()++i)    {
        interference.add(resultReg, sources.get(i).getReg());
    }
}
}
