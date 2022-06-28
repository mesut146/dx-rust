use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::util::Hex;

struct BasicBlock{
    pub label: i32,
    pub insns: InsnList,
    pub successors: IntList,
    pub primarySuccessor: i32,
}
impl BasicBlock{
    pub fn new(&self, label: i32, insns: &InsnList, successors: &IntList, primarySuccessor: i32)    {
        if label<0        {
            throw IllegalArgumentException::new("label < 0");
        }        
        try        {
            insns.throwIfMutable();
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("insns == null");
        }
        let sz: i32 = insns.size();
        if sz==0        {
            throw IllegalArgumentException::new("insns.size() == 0");
        }        
        for(        let i: i32 = sz-2;;i>=0i -= 1)        {
            let one: Rop = insns.get(i).getOpcode();
            if one.getBranchingness()!=Rop::BRANCH_NONE            {
                throw IllegalArgumentException::new("insns["+i+"] is a "+"branch or can throw");
            }            
        }
        let lastInsn: Insn = insns.get(sz-1);
        if lastInsn.getOpcode().getBranchingness()==Rop::BRANCH_NONE        {
            throw IllegalArgumentException::new("insns does not end with "+"a branch or throwing "+"instruction");
        }        
        try        {
            successors.throwIfMutable();
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("successors == null");
        }
        if primarySuccessor<-1        {
            throw IllegalArgumentException::new("primarySuccessor < -1");
        }        
        if primarySuccessor>=0&&!successors.contains(primarySuccessor)        {
            throw IllegalArgumentException::new("primarySuccessor "+primarySuccessor+" not in successors "+successors);
        }        
        self->label=label;
        self->insns=insns;
        self->successors=successors;
        self->primarySuccessor=primarySuccessor;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        return (self==other);
    }
    pub fn hashCode(&self) -> i32    {
        return System::identityHashCode(self);
    }
    pub fn getLabel(&self) -> i32    {
        return self.label;
    }
    pub fn getInsns(&self) -> InsnList    {
        return self.insns;
    }
    pub fn getSuccessors(&self) -> IntList    {
        return self.successors;
    }
    pub fn getPrimarySuccessor(&self) -> i32    {
        return self.primarySuccessor;
    }
    pub fn getSecondarySuccessor(&self) -> i32    {
        if self.successors.size()!=2        {
            throw UnsupportedOperationException::new("block doesn't have exactly two successors");
        }        
        let succ: i32 = self.successors.get(0);
        if succ==self.primarySuccessor        {
            succ=self.successors.get(1);
        }        
        return succ;
    }
    pub fn getFirstInsn(&self) -> Insn    {
        return self.insns.get(0);
    }
    pub fn getLastInsn(&self) -> Insn    {
        return self.insns.getLast();
    }
    pub fn canThrow(&self) -> boolean    {
        return self.insns.getLast().canThrow();
    }
    pub fn hasExceptionHandlers(&self) -> boolean    {
        let lastInsn: Insn = self.insns.getLast();
        return lastInsn.getCatches().size()!=0;
    }
    pub fn getExceptionHandlerTypes(&self) -> TypeList    {
        let lastInsn: Insn = self.insns.getLast();
        return lastInsn.getCatches();
    }
    pub fn withRegisterOffset(&self, delta: i32) -> BasicBlock    {
        return BasicBlock::new(self.label, self.insns.withRegisterOffset(delta), &self.successors, self.primarySuccessor);
    }
    pub fn toString(&self) -> String    {
        return '{'+Hex::u2(self.label)+'}';
    }
}
