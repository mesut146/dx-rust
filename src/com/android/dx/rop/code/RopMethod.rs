use crate::helper;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::code::BasicBlock;

struct RopMethod{
    pub blocks: BasicBlockList,
    pub firstLabel: i32,
    pub predecessors: Vec<IntList>,
    pub exitPredecessors: IntList,
}
impl RopMethod{
    pub fn new(&self, blocks: &BasicBlockList, firstLabel: i32)    {
        if blocks==None        {
            throw NullPointerException::new("blocks == null");
        }        
        if firstLabel<0        {
            throw IllegalArgumentException::new("firstLabel < 0");
        }        
        self->blocks=blocks;
        self->firstLabel=firstLabel;
        self->predecessors=None;
        self->exitPredecessors=None;
    }
    pub fn getBlocks(&self) -> BasicBlockList    {
        return self.blocks;
    }
    pub fn getFirstLabel(&self) -> i32    {
        return self.firstLabel;
    }
    pub fn labelToPredecessors(&self, label: i32) -> IntList    {
        if self.exitPredecessors==None        {
            calcPredecessors();
        }        
        let result: IntList = self.predecessors[label];
        if result==None        {
            throw RuntimeException::new("no such block: "+Hex::u2(label));
        }        
        return result;
    }
    pub fn getExitPredecessors(&self) -> IntList    {
        if self.exitPredecessors==None        {
            calcPredecessors();
        }        
        return self.exitPredecessors;
    }
    pub fn withRegisterOffset(&self, delta: i32) -> RopMethod    {
        let result: RopMethod = RopMethod::new(self.blocks.withRegisterOffset(delta), self.firstLabel);
        if self.exitPredecessors!=None        {
            =self.exitPredecessors;
            =self.predecessors;
        }        
        return result;
    }
    pub fn calcPredecessors(&self)    {
        let maxLabel: i32 = self.blocks.getMaxLabel();
        let predecessors: Vec<IntList> = new IntList[maxLabel];
        let exitPredecessors: IntList = IntList::new(10);
        let sz: i32 = self.blocks.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: BasicBlock = self.blocks.get(i);
            let label: i32 = one.getLabel();
            let successors: IntList = one.getSuccessors();
            let ssz: i32 = successors.size();
            if ssz==0            {
                exitPredecessors.add(label);
            }            else             {
                for(                let j: i32 = 0;;j<sszj += 1)                {
                    let succLabel: i32 = successors.get(j);
                    let succPreds: IntList = predecessors[succLabel];
                    if succPreds==None                    {
                        succPreds=IntList::new(10);
                        predecessors[succLabel]=succPreds;
                    }                    
                    succPreds.add(label);
                }
            }
        }
        for(        let i: i32 = 0;;i<maxLabeli += 1)        {
            let preds: IntList = predecessors[i];
            if preds!=None            {
                preds.sort();
                preds.setImmutable();
            }            
        }
        exitPredecessors.sort();
        exitPredecessors.setImmutable();
        if predecessors[self.firstLabel]==None        {
            predecessors[self.firstLabel]=IntList::EMPTY;
        }        
        self->predecessors=predecessors;
        self->exitPredecessors=exitPredecessors;
    }
}
