use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::dex::code::CatchHandlerList;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::dex::code::CatchTable::Entry;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::dex::code::CatchTable;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::code::BlockAddresses;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::rop::type::Type;

struct StdCatchBuilder{
    pub method: RopMethod,
    pub order: Vec<i32>,
    pub addresses: BlockAddresses,
}
impl StdCatchBuilder{
    pub const MAX_CATCH_RANGE: i32 = 65535;
    pub fn new(&self, method: &RopMethod, order: &Vec<i32>, addresses: &BlockAddresses)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        if order==None        {
            throw NullPointerException::new("order == null");
        }        
        if addresses==None        {
            throw NullPointerException::new("addresses == null");
        }        
        self->method=method;
        self->order=order;
        self->addresses=addresses;
    }
    pub fn build(&self) -> CatchTable    {
        return StdCatchBuilder::build_RopMethod_int[]_BlockAddresses(&self.method, &self.order, &self.addresses);
    }
    pub fn hasAnyCatches(&self) -> boolean    {
        let blocks: BasicBlockList = self.method.getBlocks();
        let size: i32 = blocks.size();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let block: BasicBlock = blocks.get(i);
            let catches: TypeList = block.getLastInsn().getCatches();
            if catches.size()!=0            {
                return true;
            }            
        }
        return false;
    }
    pub fn getCatchTypes(&self) -> HashSet<Type>    {
        let result: HashSet<Type> = HashSet<Type>::new(20);
        let blocks: BasicBlockList = self.method.getBlocks();
        let size: i32 = blocks.size();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let block: BasicBlock = blocks.get(i);
            let catches: TypeList = block.getLastInsn().getCatches();
            let catchSize: i32 = catches.size();
            for(            let j: i32 = 0;;j<catchSizej += 1)            {
                result.add(catches.getType(j));
            }
        }
        return result;
    }
    pub fn build(method: &RopMethod, order: &Vec<i32>, addresses: &BlockAddresses) -> CatchTable    {
        let len: i32 = order.len();
        let blocks: BasicBlockList = method.getBlocks();
        let resultList: ArrayList<Entry> = ArrayList<CatchTable.Entry>::new(len);
        let currentHandlers: CatchHandlerList = CatchHandlerList::EMPTY;
        let currentStartBlock: BasicBlock = None;
        let currentEndBlock: BasicBlock = None;
        for(        let i: i32 = 0;;i<leni += 1)        {
            let block: BasicBlock = blocks.labelToBlock(order[i]);
            if !block.canThrow()            {
                continue;
            }            
            let handlers: CatchHandlerList = StdCatchBuilder::handlersFor(&block, &addresses);
            if currentHandlers.size()==0            {
                currentStartBlock=block;
                currentEndBlock=block;
                currentHandlers=handlers;
                continue;
            }            
            if currentHandlers.equals(&handlers)&&StdCatchBuilder::rangeIsValid(&currentStartBlock, &block, &addresses)            {
                currentEndBlock=block;
                continue;
            }            
            if currentHandlers.size()!=0            {
                let entry: Entry = StdCatchBuilder::makeEntry(&currentStartBlock, &currentEndBlock, &currentHandlers, &addresses);
                resultList.add_Entry(&entry);
            }            
            currentStartBlock=block;
            currentEndBlock=block;
            currentHandlers=handlers;
        }
        if currentHandlers.size()!=0        {
            let entry: Entry = StdCatchBuilder::makeEntry(&currentStartBlock, &currentEndBlock, &currentHandlers, &addresses);
            resultList.add_Entry(&entry);
        }        
        let resultSz: i32 = resultList.size();
        if resultSz==0        {
            return CatchTable::EMPTY;
        }        
        let result: CatchTable = CatchTable::new(resultSz);
        for(        let i: i32 = 0;;i<resultSzi += 1)        {
            result.set(i, resultList.get(i));
        }
        result.setImmutable();
        return result;
    }
    pub fn handlersFor(block: &BasicBlock, addresses: &BlockAddresses) -> CatchHandlerList    {
        let successors: IntList = block.getSuccessors();
        let succSize: i32 = successors.size();
        let primary: i32 = block.getPrimarySuccessor();
        let catches: TypeList = block.getLastInsn().getCatches();
        let catchSize: i32 = catches.size();
        if catchSize==0        {
            return CatchHandlerList::EMPTY;
        }        
        if ((primary==-1)&&(succSize!=catchSize))||((primary!=-1)&&((succSize!=(catchSize+1))||(primary!=successors.get(catchSize))))        {
            throw RuntimeException::new("shouldn't happen: weird successors list");
        }        
        for(        let i: i32 = 0;;i<catchSizei += 1)        {
            let type: Type = catches.getType(i);
            if type_renamed.equals(&Type::OBJECT)            {
                catchSize=i+1;
                break;
            }            
        }
        let result: CatchHandlerList = CatchHandlerList::new(catchSize);
        for(        let i: i32 = 0;;i<catchSizei += 1)        {
            let oneType: CstType = CstType::new(catches.getType(i));
            let oneHandler: CodeAddress = addresses.getStart_int(successors.get(i));
            result.set_int_CstType_int(i, &oneType, oneHandler.getAddress());
        }
        result.setImmutable();
        return result;
    }
    pub fn makeEntry(start: &BasicBlock, end: &BasicBlock, handlers: &CatchHandlerList, addresses: &BlockAddresses) -> CatchTable.Entry    {
        let startAddress: CodeAddress = addresses.getLast_BasicBlock(&start);
        let endAddress: CodeAddress = addresses.getEnd_BasicBlock(&end);
        return CatchTable.Entry::new(startAddress.getAddress(), endAddress.getAddress(), &handlers);
    }
    pub fn rangeIsValid(start: &BasicBlock, end: &BasicBlock, addresses: &BlockAddresses) -> boolean    {
        if start==None        {
            throw NullPointerException::new("start == null");
        }        
        if end==None        {
            throw NullPointerException::new("end == null");
        }        
        let startAddress: i32 = addresses.getLast_BasicBlock(&start).getAddress();
        let endAddress: i32 = addresses.getEnd_BasicBlock(&end).getAddress();
        return (endAddress-startAddress)<=StdCatchBuilder::MAX_CATCH_RANGE;
    }
}
