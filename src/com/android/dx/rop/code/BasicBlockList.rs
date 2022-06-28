use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::BasicBlockList::RegCountVisitor;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::BasicBlock;

struct BasicBlockList{
    pub regCount: i32,
}
impl BasicBlockList{
    pub fn new(&self, size: i32)    {
        super(size);

        self.regCount=-1;
    }
    pub fn new(&self, old: &BasicBlockList)    {
        super(old);

        self.regCount=;
    }
    pub fn get(&self, n: i32) -> BasicBlock    {
        return (BasicBlock*)get0(n);
    }
    pub fn set(&self, n: i32, bb: &BasicBlock)    {
        super.set(n,bb);
        self.regCount=-1;
    }
    pub fn getRegCount(&self) -> i32    {
        if self.regCount==-1        {
            let visitor: RegCountVisitor = RegCountVisitor::new();
            forEachInsn(&visitor);
            self.regCount=visitor.getRegCount();
        }        
        return self.regCount;
    }
    pub fn getInstructionCount(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: BasicBlock = (BasicBlock*)getOrNull0(i);
            if one!=None            {
                result+=one.getInsns().size();
            }            
        }
        return result;
    }
    pub fn getEffectiveInstructionCount(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: BasicBlock = (BasicBlock*)getOrNull0(i);
            if one!=None            {
                let insns: InsnList = one.getInsns();
                let insnsSz: i32 = insns.size();
                for(                let j: i32 = 0;;j<insnsSzj += 1)                {
                    let insn: Insn = insns.get(j);
                    if insn.getOpcode().getOpcode()!=RegOps::MARK_LOCAL                    {
                        result += 1;
                    }                    
                }
            }            
        }
        return result;
    }
    pub fn labelToBlock(&self, label: i32) -> BasicBlock    {
        let idx: i32 = indexOfLabel(label);
        if idx<0        {
            throw IllegalArgumentException::new("no such label: "+Hex::u2(label));
        }        
        return get(idx);
    }
    pub fn forEachInsn(&self, visitor: &Insn.Visitor)    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: BasicBlock = get(i);
            let insns: InsnList = one.getInsns();
            insns.forEach(&visitor);
        }
    }
    pub fn withRegisterOffset(&self, delta: i32) -> BasicBlockList    {
        let sz: i32 = size();
        let result: BasicBlockList = BasicBlockList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: BasicBlock = (BasicBlock*)get0(i);
            if one!=None            {
                result.set_int_BasicBlock(i, one.withRegisterOffset(delta));
            }            
        }
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
    pub fn getMutableCopy(&self) -> BasicBlockList    {
        return BasicBlockList::new(self);
    }
    pub fn preferredSuccessorOf(&self, block: &BasicBlock) -> BasicBlock    {
        let primarySuccessor: i32 = block.getPrimarySuccessor();
        let successors: IntList = block.getSuccessors();
        let succSize: i32 = successors.size();
        match succSize{0 =>             {
                return None;
            }1 =>             {
                return labelToBlock(successors.get(0));
            }        }
        if primarySuccessor!=-1        {
            return labelToBlock(primarySuccessor);
        }        else         {
            return labelToBlock(successors.get(0));
        }
    }
    pub fn catchesEqual(&self, block1: &BasicBlock, block2: &BasicBlock) -> boolean    {
        let catches1: TypeList = block1.getExceptionHandlerTypes();
        let catches2: TypeList = block2.getExceptionHandlerTypes();
        if !StdTypeList::equalContents(&catches1, &catches2)        {
            return false;
        }        
        let succ1: IntList = block1.getSuccessors();
        let succ2: IntList = block2.getSuccessors();
        let size: i32 = succ1.size();
        let primary1: i32 = block1.getPrimarySuccessor();
        let primary2: i32 = block2.getPrimarySuccessor();
        if ((primary1==-1)||(primary2==-1))&&(primary1!=primary2)        {
            return false;
        }        
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let label1: i32 = succ1.get(i);
            let label2: i32 = succ2.get(i);
            if label1==primary1            {
                if label2!=primary2                {
                    return false;
                }                
                continue;
            }            
            if label1!=label2            {
                return false;
            }            
        }
        return true;
    }
}
