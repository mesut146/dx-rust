use crate::helper;
use crate::com::android::dx::dex::code::CatchTable;
use crate::com::android::dx::dex::code::OutputFinisher;
use crate::com::android::dx::dex::code::CatchBuilder;
use crate::com::android::dx::dex::code::LocalList;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::dex::code::PositionList;

struct DalvCode{
    pub positionInfo: i32,
    pub unprocessedInsns: OutputFinisher,
    pub unprocessedCatches: CatchBuilder,
    pub catches: CatchTable,
    pub positions: PositionList,
    pub locals: LocalList,
    pub insns: DalvInsnList,
}
impl DalvCode{
    pub fn new(&self, positionInfo: i32, unprocessedInsns: &OutputFinisher, unprocessedCatches: &CatchBuilder)    {
        if unprocessedInsns==None        {
            throw NullPointerException::new("unprocessedInsns == null");
        }        
        if unprocessedCatches==None        {
            throw NullPointerException::new("unprocessedCatches == null");
        }        
        self->positionInfo=positionInfo;
        self->unprocessedInsns=unprocessedInsns;
        self->unprocessedCatches=unprocessedCatches;
        self->catches=None;
        self->positions=None;
        self->locals=None;
        self->insns=None;
    }
    pub fn finishProcessingIfNecessary(&self)    {
        if self.insns!=None        {
            return;
        }        
        self.insns=self.unprocessedInsns.finishProcessingAndGetList();
        self.positions=PositionList::make(&self.insns, self.positionInfo);
        self.locals=LocalList::make(&self.insns);
        self.catches=self.unprocessedCatches.build();
        self.unprocessedInsns=None;
        self.unprocessedCatches=None;
    }
    pub fn assignIndices(&self, callback: &AssignIndicesCallback)    {
        self.unprocessedInsns.assignIndices_AssignIndicesCallback(&callback);
    }
    pub fn hasPositions(&self) -> boolean    {
        return (self.positionInfo!=PositionList::NONE)&&self.unprocessedInsns.hasAnyPositionInfo();
    }
    pub fn hasLocals(&self) -> boolean    {
        return self.unprocessedInsns.hasAnyLocalInfo();
    }
    pub fn hasAnyCatches(&self) -> boolean    {
        return self.unprocessedCatches.hasAnyCatches();
    }
    pub fn getCatchTypes(&self) -> HashSet<Type>    {
        return self.unprocessedCatches.getCatchTypes();
    }
    pub fn getInsnConstants(&self) -> HashSet<Constant>    {
        return self.unprocessedInsns.getAllConstants();
    }
    pub fn getInsns(&self) -> DalvInsnList    {
        finishProcessingIfNecessary();
        return self.insns;
    }
    pub fn getCatches(&self) -> CatchTable    {
        finishProcessingIfNecessary();
        return self.catches;
    }
    pub fn getPositions(&self) -> PositionList    {
        finishProcessingIfNecessary();
        return self.positions;
    }
    pub fn getLocals(&self) -> LocalList    {
        finishProcessingIfNecessary();
        return self.locals;
    }
}
