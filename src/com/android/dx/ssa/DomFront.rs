use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::ssa::DomFront::DomInfo;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::ssa::SetFactory;
use crate::com::android::dx::ssa::Dominators;
use crate::com::android::dx::ssa::SsaBasicBlock;

struct DomFront{
    pub meth: SsaMethod,
    pub nodes: ArrayList<SsaBasicBlock>,
    pub domInfos: Vec<DomInfo>,
}
impl DomFront{
    pub const DEBUG: boolean = false;
    pub fn new(&self, meth: &SsaMethod)    {
        self->meth=meth;
        self.nodes=meth.getBlocks();
        let szNodes: i32 = self.nodes.size();
        self.domInfos=new DomInfo[szNodes];
        for(        let i: i32 = 0;;i<szNodesi += 1)        {
            self.domInfos[i]=DomInfo::new();
        }
    }
    pub fn run(&self) -> Vec<DomInfo>    {
        let szNodes: i32 = self.nodes.size();
        if DomFront::DEBUG        {
            for(            let i: i32 = 0;;i<szNodesi += 1)            {
                let node: SsaBasicBlock = self.nodes.get(i);
                System::out.println_String("pred["+i+"]: "+node.getPredecessors());
            }
        }        
        let methDom: Dominators = Dominators::make(&self.meth, &self.domInfos, false);
        if DomFront::DEBUG        {
            for(            let i: i32 = 0;;i<szNodesi += 1)            {
                let info: DomInfo = self.domInfos[i];
                System::out.println_String("idom["+i+"]: "+);
            }
        }        
        buildDomTree();
        if DomFront::DEBUG        {
            debugPrintDomChildren();
        }        
        for(        let i: i32 = 0;;i<szNodesi += 1)        {
            self.domInfos[i]->dominanceFrontiers=SetFactory::makeDomFrontSet(szNodes);
        }
        calcDomFronts();
        if DomFront::DEBUG        {
            for(            let i: i32 = 0;;i<szNodesi += 1)            {
                System::out.println_String("df["+i+"]: "+self.domInfos[i]->dominanceFrontiers);
            }
        }        
        return self.domInfos;
    }
    pub fn debugPrintDomChildren(&self)    {
        let szNodes: i32 = self.nodes.size();
        for(        let i: i32 = 0;;i<szNodesi += 1)        {
            let node: SsaBasicBlock = self.nodes.get(i);
            let sb: StringBuffer = StringBuffer::new();
            sb.append_char('{');
            let comma: boolean = false;
            for child in node.getDomChildren()            {
                if comma                {
                    sb.append_char(',');
                }                
                sb.append_Object(&child);
                comma=true;
            }
            sb.append_char('}');
            System::out.println_String("domChildren["+node+"]: "+sb);
        }
    }
    pub fn buildDomTree(&self)    {
        let szNodes: i32 = self.nodes.size();
        for(        let i: i32 = 0;;i<szNodesi += 1)        {
            let info: DomInfo = self.domInfos[i];
            if ==-1{                continue;            }            
            let domParent: SsaBasicBlock = self.nodes.get();
            domParent.addDomChild(self.nodes.get(i));
        }
    }
    pub fn calcDomFronts(&self)    {
        let szNodes: i32 = self.nodes.size();
        for(        let b: i32 = 0;;b<szNodesb += 1)        {
            let nb: SsaBasicBlock = self.nodes.get(b);
            let nbInfo: DomInfo = self.domInfos[b];
            let pred: BitSet = nb.getPredecessors();
            if pred.cardinality()>1            {
                for(                let i: i32 = pred.nextSetBit_int(0);;i>=0i=pred.nextSetBit_int(i+1))                {
                    for(                    let runnerIndex: i32 = i;;runnerIndex!=)                    {
                        if runnerIndex==-1{                            break;                        }                        
                        let runnerInfo: DomInfo = self.domInfos[runnerIndex];
                        if .has(b)                        {
                            break;
                        }                        
                        .add(b);
                        runnerIndex=;
                    }
                }
            }            
        }
    }
}
