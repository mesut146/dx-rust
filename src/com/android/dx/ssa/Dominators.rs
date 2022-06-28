use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::ssa::Dominators::DFSInfo;
use crate::com::android::dx::ssa::Dominators::DfsWalker;
use crate::com::android::dx::ssa::Dominators;
use crate::com::android::dx::ssa::SsaBasicBlock;

struct Dominators{
    pub postdom: boolean,
    pub meth: SsaMethod,
    pub blocks: ArrayList<SsaBasicBlock>,
    pub info: Vec<DFSInfo>,
    pub vertex: ArrayList<SsaBasicBlock>,
    pub domInfos: DomFront.DomInfo,
}
impl Dominators{
    pub fn new(&self, meth: &SsaMethod, domInfos: &Vec<DomFront.DomInfo>, postdom: boolean)    {
        self->meth=meth;
        self->domInfos=domInfos;
        self->postdom=postdom;
        self->blocks=meth.getBlocks();
        self->info=new DFSInfo[blocks.size() + 2];
        self->vertex=ArrayList<SsaBasicBlock>::new();
    }
    pub fn make(meth: &SsaMethod, domInfos: &Vec<DomFront.DomInfo>, postdom: boolean) -> Dominators    {
        let result: Dominators = Dominators::new(&meth, &domInfos, postdom);
        result.run();
        return result;
    }
    pub fn getSuccs(&self, block: &SsaBasicBlock) -> BitSet    {
        if self.postdom        {
            return block.getPredecessors();
        }        else         {
            return block.getSuccessors();
        }
    }
    pub fn getPreds(&self, block: &SsaBasicBlock) -> BitSet    {
        if self.postdom        {
            return block.getSuccessors();
        }        else         {
            return block.getPredecessors();
        }
    }
    pub fn compress(&self, in: &SsaBasicBlock)    {
        let bbInfo: DFSInfo = self.info[in.getIndex()];
        let ancestorbbInfo: DFSInfo = self.info[.getIndex()];
        if !=None        {
            let worklist: ArrayList<SsaBasicBlock> = ArrayList<SsaBasicBlock>::new();
            let visited: HashSet<SsaBasicBlock> = HashSet<SsaBasicBlock>::new();
            worklist.add_SsaBasicBlock(&in);
            while !worklist.isEmpty()            {
                let wsize: i32 = worklist.size();
                let v: SsaBasicBlock = worklist.get(wsize-1);
                let vbbInfo: DFSInfo = self.info[v.getIndex()];
                let vAncestor: SsaBasicBlock = ;
                let vabbInfo: DFSInfo = self.info[vAncestor.getIndex()];
                if visited.add(&vAncestor)&&!=None                {
                    worklist.add_SsaBasicBlock(&vAncestor);
                    continue;
                }                
                worklist.remove_int(wsize-1);
                if ==None                {
                    continue;
                }                
                let vAncestorRep: SsaBasicBlock = ;
                let vRep: SsaBasicBlock = ;
                if self.info[vAncestorRep.getIndex()]->semidom<self.info[vRep.getIndex()]->semidom                {
                    =vAncestorRep;
                }                
                =;
            }
        }        
    }
    pub fn eval(&self, v: &SsaBasicBlock) -> SsaBasicBlock    {
        let bbInfo: DFSInfo = self.info[v.getIndex()];
        if ==None        {
            return v;
        }        
        compress(&v);
        return ;
    }
    pub fn run(&self)    {
        let root: SsaBasicBlock = if self.postdom { self.meth.getExitBlock() } else { self.meth.getEntryBlock() };
                if root!=None                {
                    self.vertex.add_SsaBasicBlock(&root);
                    self.domInfos[root.getIndex()]->idom=root.getIndex();
                }                
                let walker: DfsWalker = DfsWalker::new(, self);
                self.meth.forEachBlockDepthFirst(self.postdom, &walker);
                let dfsMax: i32 = self.vertex.size()-1;
                for(                let i: i32 = dfsMax;;i>=2--i)                {
                    let w: SsaBasicBlock = self.vertex.get(i);
                    let wInfo: DFSInfo = self.info[w.getIndex()];
                    let preds: BitSet = getPreds(&w);
                    for(                    let j: i32 = preds.nextSetBit_int(0);;j>=0j=preds.nextSetBit_int(j+1))                    {
                        let predBlock: SsaBasicBlock = self.blocks.get(j);
                        let predInfo: DFSInfo = self.info[predBlock.getIndex()];
                        if predInfo!=None                        {
                            let predSemidom: i32 = self.info[eval(&predBlock).getIndex()]->semidom;
                            if predSemidom<                            {
                                =predSemidom;
                            }                            
                        }                        
                    }
                    self.info[self.vertex.get().getIndex()]->bucket.add_SsaBasicBlock(&w);
                    =;
                    let wParentBucket: ArrayList<SsaBasicBlock>;
                    wParentBucket=self.info[.getIndex()]->bucket;
                    while !wParentBucket.isEmpty()                    {
                        let lastItem: i32 = wParentBucket.size()-1;
                        let last: SsaBasicBlock = wParentBucket.remove_int(lastItem);
                        let U: SsaBasicBlock = eval(&last);
                        if self.info[U.getIndex()]->semidom<self.info[last.getIndex()]->semidom                        {
                            self.domInfos[last.getIndex()]->idom=U.getIndex();
                        }                        else                         {
                            self.domInfos[last.getIndex()]->idom=.getIndex();
                        }
                    }
                }
                for(                let i: i32 = 2;;i<=dfsMax++i)                {
                    let w: SsaBasicBlock = self.vertex.get(i);
                    if self.domInfos[w.getIndex()]->idom!=self.vertex.get(self.info[w.getIndex()]->semidom).getIndex()                    {
                        self.domInfos[w.getIndex()]->idom=self.domInfos[self.domInfos[w.getIndex()]->idom]->idom;
                    }                    
                }
            }
}
