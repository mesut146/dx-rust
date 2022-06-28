use crate::helper;
use crate::com::android::dx::cf::code::ByteBlockList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::cf::code::ByteOps;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::cf::code::ByteCatchList;
use crate::com::android::dx::cf::code::ByteBlock;
use crate::com::android::dx::cf::code::ByteCatchList::Item;
use crate::com::android::dx::rop::cst::CstMemberRef;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::cf::code::BytecodeArray;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::cf::code::SwitchList;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::util::Bits;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::cf::code::BasicBlocker;
use crate::com::android::dx::rop::type::Type;

struct BasicBlocker{
    pub method: ConcreteMethod,
    pub workSet: Vec<i32>,
    pub liveSet: Vec<i32>,
    pub blockSet: Vec<i32>,
    pub targetLists: Vec<IntList>,
    pub catchLists: Vec<ByteCatchList>,
    pub previousOffset: i32,
}
impl BasicBlocker{
    pub fn identifyBlocks(method: &ConcreteMethod) -> ByteBlockList    {
        let bb: BasicBlocker = BasicBlocker::new(&method);
        bb.doit();
        return bb.getBlockList();
    }
    pub fn new(&self, method: &ConcreteMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        self->method=method;
        let sz: i32 = method.getCode().size()+1;
        self.workSet=Bits::makeBitSet(sz);
        self.liveSet=Bits::makeBitSet(sz);
        self.blockSet=Bits::makeBitSet(sz);
        self.targetLists=new IntList[sz];
        self.catchLists=new ByteCatchList[sz];
        self.previousOffset=-1;
    }
    pub fn visitInvalid(&self, opcode: i32, offset: i32, length: i32)    {
        visitCommon(offset, length, true);
    }
    pub fn visitNoArgs(&self, opcode: i32, offset: i32, length: i32, type: &Type)    {
        match opcode{ByteOps::IRETURN => ByteOps::RETURN =>             {
                visitCommon(offset, length, false);
                self.targetLists[offset]=IntList::EMPTY;
                break;
            }ByteOps::ATHROW =>             {
                visitCommon(offset, length, false);
                visitThrowing(offset, length, false);
                break;
            }ByteOps::IALOAD => ByteOps::LALOAD => ByteOps::FALOAD => ByteOps::DALOAD => ByteOps::AALOAD => ByteOps::BALOAD => ByteOps::CALOAD => ByteOps::SALOAD => ByteOps::IASTORE => ByteOps::LASTORE => ByteOps::FASTORE => ByteOps::DASTORE => ByteOps::AASTORE => ByteOps::BASTORE => ByteOps::CASTORE => ByteOps::SASTORE => ByteOps::ARRAYLENGTH => ByteOps::MONITORENTER => ByteOps::MONITOREXIT =>             {
                visitCommon(offset, length, true);
                visitThrowing(offset, length, true);
                break;
            }ByteOps::IDIV => ByteOps::IREM =>             {
                visitCommon(offset, length, true);
                if (type==Type::INT)||(type==Type::LONG)                {
                    visitThrowing(offset, length, true);
                }                
                break;
            }        _ => {}        {
            visitCommon(offset, length, true);
            break;
        }    }
}
pub fn visitLocal(&self, opcode: i32, offset: i32, length: i32, idx: i32, type: &Type, value: i32){
    if opcode==ByteOps::RET    {
        visitCommon(offset, length, false);
        self.targetLists[offset]=IntList::EMPTY;
    }    else     {
        visitCommon(offset, length, true);
    }
}
pub fn visitConstant(&self, opcode: i32, offset: i32, length: i32, cst: &Constant, value: i32){
    visitCommon(offset, length, true);
    if //cst instanceof CstMemberRef||//cst instanceof CstType||//cst instanceof CstString||//cst instanceof CstInvokeDynamic||//cst instanceof CstMethodHandle||//cst instanceof CstProtoRef    {
        visitThrowing(offset, length, true);
    }    
}
pub fn visitBranch(&self, opcode: i32, offset: i32, length: i32, target: i32){
    match opcode{ByteOps::GOTO =>         {
            visitCommon(offset, length, false);
            self.targetLists[offset]=IntList::makeImmutable_int(target);
            break;
        }ByteOps::JSR =>         {
            addWorkIfNecessary(offset, true);
        }    _ => {}    {
        let next: i32 = offset+length;
        visitCommon(offset, length, true);
        addWorkIfNecessary(next, true);
        self.targetLists[offset]=IntList::makeImmutable_int_int(next, target);
        break;
    }}
addWorkIfNecessary(target, true);
}
pub fn visitSwitch(&self, opcode: i32, offset: i32, length: i32, cases: &SwitchList, padding: i32){
visitCommon(offset, length, false);
addWorkIfNecessary(cases.getDefaultTarget(), true);
let sz: i32 = cases.size();
for(let i: i32 = 0;;i<szi += 1){
    addWorkIfNecessary(cases.getTarget(i), true);
}
self.targetLists[offset]=cases.getTargets();
}
pub fn visitNewarray(&self, offset: i32, length: i32, type: &CstType, intVals: &ArrayList<Constant>){
visitCommon(offset, length, true);
visitThrowing(offset, length, true);
}
pub fn getBlockList(&self) -> ByteBlockList{
let bytes: BytecodeArray = self.method.getCode();
let bbs: Vec<ByteBlock> = new ByteBlock[bytes.size()];
let count: i32 = 0;
for(let at: i32 = 0;let next: i32;;at=next){
    next=Bits::findFirst_int[]_int(&self.blockSet, at+1);
    if next<0    {
        break;
    }    
    if Bits::get(&self.liveSet, at)    {
        let targets: IntList = None;
        let targetsAt: i32 = -1;
        let blockCatches: ByteCatchList;
        for(        let i: i32 = next-1;;i>=ati -= 1)        {
            targets=self.targetLists[i];
            if targets!=None            {
                targetsAt=i;
                break;
            }            
        }
        if targets==None        {
            targets=IntList::makeImmutable_int(next);
            blockCatches=ByteCatchList::EMPTY;
        }        else         {
            blockCatches=self.catchLists[targetsAt];
            if blockCatches==None            {
                blockCatches=ByteCatchList::EMPTY;
            }            
        }
        bbs[count]=ByteBlock::new(at, at, next, &targets, &blockCatches);
        count += 1;
    }    
}
let result: ByteBlockList = ByteBlockList::new(count);
for(let i: i32 = 0;;i<counti += 1){
    result.set(i, bbs[i]);
}
return result;
}
pub fn doit(&self){
let bytes: BytecodeArray = self.method.getCode();
let catches: ByteCatchList = self.method.getCatches();
let catchSz: i32 = catches.size();
Bits::set_int[]_int(&self.workSet, 0);
Bits::set_int[]_int(&self.blockSet, 0);
while !Bits::isEmpty(&self.workSet){
    try    {
        bytes.processWorkSet(&self.workSet, self);
    }    catch(    let ex: IllegalArgumentException)    {
        throw SimException::new("flow of control falls off "+"end of method", &ex);
    }
    for(    let i: i32 = 0;;i<catchSzi += 1)    {
        let item: Item = catches.get(i);
        let start: i32 = item.getStartPc();
        let end: i32 = item.getEndPc();
        if Bits::anyInRange(&self.liveSet, start, end)        {
            Bits::set_int[]_int(&self.blockSet, start);
            Bits::set_int[]_int(&self.blockSet, end);
            addWorkIfNecessary(item.getHandlerPc(), true);
        }        
    }
}
}
pub fn addWorkIfNecessary(&self, offset: i32, blockStart: boolean){
if !Bits::get(&self.liveSet, offset){
    Bits::set_int[]_int(&self.workSet, offset);
}
if blockStart{
    Bits::set_int[]_int(&self.blockSet, offset);
}
}
pub fn visitCommon(&self, offset: i32, length: i32, nextIsLive: boolean){
Bits::set_int[]_int(&self.liveSet, offset);
if nextIsLive{
    addWorkIfNecessary(offset+length, false);
}else {
    Bits::set_int[]_int(&self.blockSet, offset+length);
}
}
pub fn visitThrowing(&self, offset: i32, length: i32, nextIsLive: boolean){
let next: i32 = offset+length;
if nextIsLive{
    addWorkIfNecessary(next, true);
}
let catches: ByteCatchList = self.method.getCatches().listFor(offset);
self.catchLists[offset]=catches;
self.targetLists[offset]=catches.toTargetList(if nextIsLive { next } else { -1 });
    }
    pub fn setPreviousOffset(&self, offset: i32)    {
        self.previousOffset=offset;
    }
    pub fn getPreviousOffset(&self) -> i32    {
        return self.previousOffset;
    }
}
