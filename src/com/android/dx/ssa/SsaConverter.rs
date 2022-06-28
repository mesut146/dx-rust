use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::util::IntIterator;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::ssa::SsaRenamer;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::ssa::LocalVariableInfo;
use crate::com::android::dx::ssa::LocalVariableExtractor;
use crate::com::android::dx::ssa::DomFront;

struct SsaConverter{
}
impl SsaConverter{
    pub const DEBUG: boolean = false;
    pub fn convertToSsaMethod(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean) -> SsaMethod    {
        let result: SsaMethod = SsaMethod::newFromRopMethod(&rmeth, paramWidth, isStatic);
        SsaConverter::edgeSplit(&result);
        let localInfo: LocalVariableInfo = LocalVariableExtractor::extract(&result);
        SsaConverter::placePhiFunctions(&result, &localInfo, 0);
        SsaRenamer::new(&result).run();
        result.makeExitBlock();
        return result;
    }
    pub fn updateSsaMethod(ssaMeth: &SsaMethod, threshold: i32)    {
        let localInfo: LocalVariableInfo = LocalVariableExtractor::extract(&ssaMeth);
        SsaConverter::placePhiFunctions(&ssaMeth, &localInfo, threshold);
        SsaRenamer::new(&ssaMeth, threshold).run();
    }
    pub fn testEdgeSplit(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean) -> SsaMethod    {
        let result: SsaMethod;
        result=SsaMethod::newFromRopMethod(&rmeth, paramWidth, isStatic);
        SsaConverter::edgeSplit(&result);
        return result;
    }
    pub fn testPhiPlacement(rmeth: &RopMethod, paramWidth: i32, isStatic: boolean) -> SsaMethod    {
        let result: SsaMethod;
        result=SsaMethod::newFromRopMethod(&rmeth, paramWidth, isStatic);
        SsaConverter::edgeSplit(&result);
        let localInfo: LocalVariableInfo = LocalVariableExtractor::extract(&result);
        SsaConverter::placePhiFunctions(&result, &localInfo, 0);
        return result;
    }
    pub fn edgeSplit(result: &SsaMethod)    {
        SsaConverter::edgeSplitPredecessors(&result);
        SsaConverter::edgeSplitMoveExceptionsAndResults(&result);
        SsaConverter::edgeSplitSuccessors(&result);
    }
    pub fn edgeSplitPredecessors(result: &SsaMethod)    {
        let blocks: ArrayList<SsaBasicBlock> = result.getBlocks();
        for(        let i: i32 = blocks.size()-1;;i>=0i -= 1)        {
            let block: SsaBasicBlock = blocks.get(i);
            if SsaConverter::nodeNeedsUniquePredecessor(&block)            {
                block.insertNewPredecessor();
            }            
        }
    }
    pub fn nodeNeedsUniquePredecessor(block: &SsaBasicBlock) -> boolean    {
        let countPredecessors: i32 = block.getPredecessors().cardinality();
        let countSuccessors: i32 = block.getSuccessors().cardinality();
        return (countPredecessors>1&&countSuccessors>1);
    }
    pub fn edgeSplitMoveExceptionsAndResults(ssaMeth: &SsaMethod)    {
        let blocks: ArrayList<SsaBasicBlock> = ssaMeth.getBlocks();
        for(        let i: i32 = blocks.size()-1;;i>=0i -= 1)        {
            let block: SsaBasicBlock = blocks.get(i);
            if !block.isExitBlock()&&block.getPredecessors().cardinality()>1&&block.getInsns().get(0).isMoveException()            {
                let preds: BitSet = (BitSet*)block.getPredecessors().clone();
                for(                let j: i32 = preds.nextSetBit_int(0);;j>=0j=preds.nextSetBit_int(j+1))                {
                    let predecessor: SsaBasicBlock = blocks.get(j);
                    let zNode: SsaBasicBlock = predecessor.insertNewSuccessor(&block);
                    zNode.getInsns().add_int_SsaInsn(0, block.getInsns().get(0).clone());
                }
                block.getInsns().remove_int(0);
            }            
        }
    }
    pub fn edgeSplitSuccessors(result: &SsaMethod)    {
        let blocks: ArrayList<SsaBasicBlock> = result.getBlocks();
        for(        let i: i32 = blocks.size()-1;;i>=0i -= 1)        {
            let block: SsaBasicBlock = blocks.get(i);
            let successors: BitSet = (BitSet*)block.getSuccessors().clone();
            for(            let j: i32 = successors.nextSetBit_int(0);;j>=0j=successors.nextSetBit_int(j+1))            {
                let succ: SsaBasicBlock = blocks.get(j);
                if SsaConverter::needsNewSuccessor(&block, &succ)                {
                    block.insertNewSuccessor(&succ);
                }                
            }
        }
    }
    pub fn needsNewSuccessor(block: &SsaBasicBlock, succ: &SsaBasicBlock) -> boolean    {
        let insns: ArrayList<SsaInsn> = block.getInsns();
        let lastInsn: SsaInsn = insns.get(insns.size()-1);
        if block.getSuccessors().cardinality()>1&&succ.getPredecessors().cardinality()>1        {
            return true;
        }        
        return ((lastInsn.getResult()!=None)||(lastInsn.getSources().size()>0))&&succ.getPredecessors().cardinality()>1;
    }
    pub fn placePhiFunctions(ssaMeth: &SsaMethod, localInfo: &LocalVariableInfo, threshold: i32)    {
        let ssaBlocks: ArrayList<SsaBasicBlock>;
        let regCount: i32;
        let blockCount: i32;
        ssaBlocks=ssaMeth.getBlocks();
        blockCount=ssaBlocks.size();
        regCount=ssaMeth.getRegCount()-threshold;
        let df: DomFront = DomFront::new(&ssaMeth);
        let domInfos: Vec<DomInfo> = df.run();
        let defsites: Vec<BitSet> = new BitSet[regCount];
        let phisites: Vec<BitSet> = new BitSet[regCount];
        for(        let i: i32 = 0;;i<regCounti += 1)        {
            defsites[i]=BitSet::new(blockCount);
            phisites[i]=BitSet::new(blockCount);
        }
        for(        let bi: i32 = 0;        let s: i32 = ssaBlocks.size();;bi<sbi += 1)        {
            let b: SsaBasicBlock = ssaBlocks.get(bi);
            for insn in b.getInsns()            {
                let rs: RegisterSpec = insn.getResult();
                if rs!=None&&rs.getReg()-threshold>=0                {
                    defsites[rs.getReg()-threshold].set_int(bi);
                }                
            }
        }
        if SsaConverter::DEBUG        {
            System::out.println_String("defsites");
            for(            let i: i32 = 0;;i<regCounti += 1)            {
                let sb: StringBuilder = StringBuilder::new();
                sb.append_char('v').append_int(i).append_String(": ");
                sb.append_String(defsites[i].toString());
                System::out.println_Object(&sb);
            }
        }        
        let worklist: BitSet;
        for(        let reg: i32 = 0;        let s: i32 = regCount;;reg<sreg += 1)        {
            let workBlockIndex: i32;
            worklist=(BitSet*)(defsites[reg].clone());
            while 0<=(workBlockIndex=worklist.nextSetBit_int(0))            {
                worklist.clear_int(workBlockIndex);
                let dfIterator: IntIterator = domInfos[workBlockIndex]->dominanceFrontiers.iterator();
                while dfIterator.hasNext()                {
                    let dfBlockIndex: i32 = dfIterator.next();
                    if !phisites[reg].get_int(dfBlockIndex)                    {
                        phisites[reg].set_int(dfBlockIndex);
                        let tReg: i32 = reg+threshold;
                        let rs: RegisterSpec = localInfo.getStarts_int(dfBlockIndex).get_int(tReg);
                        if rs==None                        {
                            ssaBlocks.get(dfBlockIndex).addPhiInsnForReg_int(tReg);
                        }                        else                         {
                            ssaBlocks.get(dfBlockIndex).addPhiInsnForReg_RegisterSpec(&rs);
                        }
                        if !defsites[reg].get_int(dfBlockIndex)                        {
                            worklist.set_int(dfBlockIndex);
                        }                        
                    }                    
                }
            }
        }
        if SsaConverter::DEBUG        {
            System::out.println_String("phisites");
            for(            let i: i32 = 0;;i<regCounti += 1)            {
                let sb: StringBuilder = StringBuilder::new();
                sb.append_char('v').append_int(i).append_String(": ");
                sb.append_String(phisites[i].toString());
                System::out.println_Object(&sb);
            }
        }        
    }
}
