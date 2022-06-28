use crate::helper;
use crate::com::android::dx::ssa::SsaRenamer::BlockRenamer;
use crate::com::android::dx::ssa::PhiInsn;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::ssa::PhiInsn::Visitor;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::ssa::NormalSsaInsn;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::ssa::SsaBasicBlock::Visitor;
use crate::com::android::dx::ssa::SsaRenamer::BlockRenamer::RenamingMapper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::rop::code::PlainInsn;

struct SsaRenamer{
    pub ssaMeth: SsaMethod,
    pub nextSsaReg: i32,
    pub ropRegCount: i32,
    pub threshold: i32,
    pub startsForBlocks: Vec<Vec<RegisterSpec>>,
    pub ssaRegToLocalItems: ArrayList<LocalItem>,
    pub ssaRegToRopReg: IntList,
}
impl SsaRenamer{
    pub const DEBUG: boolean = false;
    pub fn new(&self, ssaMeth: &SsaMethod)    {
        self.ropRegCount=ssaMeth.getRegCount();
        self->ssaMeth=ssaMeth;
        self.nextSsaReg=self.ropRegCount;
        self.threshold=0;
        self.startsForBlocks=new RegisterSpec[ssaMeth.getBlocks().size()][];
        self.ssaRegToLocalItems=ArrayList<LocalItem>::new();
        if SsaRenamer::DEBUG        {
            self.ssaRegToRopReg=IntList::new(self.ropRegCount);
        }        
        let initialRegMapping: Vec<RegisterSpec> = new RegisterSpec[ropRegCount];
        for(        let i: i32 = 0;;i<self.ropRegCounti += 1)        {
            initialRegMapping[i]=RegisterSpec::make_int_TypeBearer(i, &Type::VOID);
            if SsaRenamer::DEBUG            {
                self.ssaRegToRopReg.add(i);
            }            
        }
        self.startsForBlocks[ssaMeth.getEntryBlockIndex()]=initialRegMapping;
    }
    pub fn new(&self, ssaMeth: &SsaMethod, thresh: i32)    {
        this(ssaMeth);

        self.threshold=thresh;
    }
    pub fn run(&self)    {
        self.ssaMeth.forEachBlockDepthFirstDom(/*new SsaBasicBlock.Visitor(){
  @Override public void visitBlock(  SsaBasicBlock block,  SsaBasicBlock unused){
    new BlockRenamer(block).process();
  }
}
*/);
        self.ssaMeth.setNewRegCount(self.nextSsaReg);
        self.ssaMeth.onInsnsChanged();
        if SsaRenamer::DEBUG        {
            System::out.println_String("SSA\tRop");
            let versions: Vec<i32> = new int[ropRegCount];
            let sz: i32 = self.ssaRegToRopReg.size();
            for(            let i: i32 = 0;;i<szi += 1)            {
                let ropReg: i32 = self.ssaRegToRopReg.get(i);
                System::out.println_String(i+"\t"+ropReg+"["+versions[ropReg]+"]");
                versions[ropReg] += 1;
            }
        }        
    }
    pub fn dupArray(orig: &Vec<RegisterSpec>) -> Vec<RegisterSpec>    {
        let copy: Vec<RegisterSpec> = new RegisterSpec[orig.length];
        System::arraycopy(&orig, 0, &copy, 0, orig.len());
        return copy;
    }
    pub fn getLocalForNewReg(&self, ssaReg: i32) -> LocalItem    {
        if ssaReg<self.ssaRegToLocalItems.size()        {
            return self.ssaRegToLocalItems.get(ssaReg);
        }        else         {
            return None;
        }
    }
    pub fn setNameForSsaReg(&self, ssaReg: &RegisterSpec)    {
        let reg: i32 = ssaReg.getReg();
        let local: LocalItem = ssaReg.getLocalItem();
        self.ssaRegToLocalItems.ensureCapacity(reg+1);
        while self.ssaRegToLocalItems.size()<=reg        {
            self.ssaRegToLocalItems.add_LocalItem(None);
        }
        self.ssaRegToLocalItems.set(reg, &local);
    }
    pub fn isBelowThresholdRegister(&self, ssaReg: i32) -> boolean    {
        return ssaReg<self.threshold;
    }
    pub fn isVersionZeroRegister(&self, ssaReg: i32) -> boolean    {
        return ssaReg<self.ropRegCount;
    }
    pub fn equalsHandlesNulls(a: &Object, b: &Object) -> boolean    {
        return a==b||(a!=None&&a.equals(&b));
    }
}
