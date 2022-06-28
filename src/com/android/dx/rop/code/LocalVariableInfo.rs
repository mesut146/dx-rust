use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::rop::code::BasicBlock;

struct LocalVariableInfo{
    pub regCount: i32,
    pub emptySet: RegisterSpecSet,
    pub blockStarts: Vec<RegisterSpecSet>,
    pub insnAssignments: HashMap<Insn,RegisterSpec>,
}
impl LocalVariableInfo{
    pub fn new(&self, method: &RopMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        let blocks: BasicBlockList = method.getBlocks();
        let maxLabel: i32 = blocks.getMaxLabel();
        self->regCount=blocks.getRegCount();
        self->emptySet=RegisterSpecSet::new(self.regCount);
        self->blockStarts=new RegisterSpecSet[maxLabel];
        self->insnAssignments=HashMap<Insn,RegisterSpec>::new(blocks.getInstructionCount());
        self.emptySet.setImmutable();
    }
    pub fn setStarts(&self, label: i32, specs: &RegisterSpecSet)    {
        throwIfImmutable();
        if specs==None        {
            throw NullPointerException::new("specs == null");
        }        
        try        {
            self.blockStarts[label]=specs;
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            throw IllegalArgumentException::new("bogus label");
        }
    }
    pub fn mergeStarts(&self, label: i32, specs: &RegisterSpecSet) -> boolean    {
        let start: RegisterSpecSet = getStarts0(label);
        let changed: boolean = false;
        if start==None        {
            setStarts(label, &specs);
            return true;
        }        
        let newStart: RegisterSpecSet = start.mutableCopy();
        if start.size()!=0        {
            newStart.intersect(&specs, true);
        }        else         {
            newStart=specs.mutableCopy();
        }
        if start.equals(&newStart)        {
            return false;
        }        
        newStart.setImmutable();
        setStarts(label, &newStart);
        return true;
    }
    pub fn getStarts(&self, label: i32) -> RegisterSpecSet    {
        let result: RegisterSpecSet = getStarts0(label);
        return if (result!=None) { result } else { self.emptySet };
            }
            pub fn getStarts(&self, block: &BasicBlock) -> RegisterSpecSet            {
                return getStarts_int(block.getLabel());
            }
            pub fn mutableCopyOfStarts(&self, label: i32) -> RegisterSpecSet            {
                let result: RegisterSpecSet = getStarts0(label);
                return if (result!=None) { result.mutableCopy() } else { RegisterSpecSet::new(self.regCount) };
                    }
                    pub fn addAssignment(&self, insn: &Insn, spec: &RegisterSpec)                    {
                        throwIfImmutable();
                        if insn==None                        {
                            throw NullPointerException::new("insn == null");
                        }                        
                        if spec==None                        {
                            throw NullPointerException::new("spec == null");
                        }                        
                        self.insnAssignments.put(&insn, &spec);
                    }
                    pub fn getAssignment(&self, insn: &Insn) -> RegisterSpec                    {
                        return self.insnAssignments.get(&insn);
                    }
                    pub fn getAssignmentCount(&self) -> i32                    {
                        return self.insnAssignments.size();
                    }
                    pub fn debugDump(&self)                    {
                        for(                        let label: i32 = 0;;label<self.blockStarts.len()label += 1)                        {
                            if self.blockStarts[label]==None                            {
                                continue;
                            }                            
                            if self.blockStarts[label]==self.emptySet                            {
                                System::out.printf_String_Object[]("%04x: empty set\n", label);
                            }                            else                             {
                                System::out.printf_String_Object[]("%04x: %s\n", label, self.blockStarts[label]);
                            }
                        }
                    }
                    pub fn getStarts0(&self, label: i32) -> RegisterSpecSet                    {
                        try                        {
                            return self.blockStarts[label];
                        }                        catch(                        let ex: ArrayIndexOutOfBoundsException)                        {
                            throw IllegalArgumentException::new("bogus label");
                        }
                    }
}
