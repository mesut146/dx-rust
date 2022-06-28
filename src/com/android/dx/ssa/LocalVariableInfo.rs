use crate::helper;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::ssa::SsaBasicBlock;

struct LocalVariableInfo{
    pub regCount: i32,
    pub emptySet: RegisterSpecSet,
    pub blockStarts: Vec<RegisterSpecSet>,
    pub insnAssignments: HashMap<SsaInsn,RegisterSpec>,
}
impl LocalVariableInfo{
    pub fn new(&self, method: &SsaMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        let blocks: List<SsaBasicBlock> = method.getBlocks();
        self->regCount=method.getRegCount();
        self->emptySet=RegisterSpecSet::new(self.regCount);
        self->blockStarts=new RegisterSpecSet[blocks.size()];
        self->insnAssignments=HashMap<SsaInsn,RegisterSpec>::new();
        self.emptySet.setImmutable();
    }
    pub fn setStarts(&self, index: i32, specs: &RegisterSpecSet)    {
        throwIfImmutable();
        if specs==None        {
            throw NullPointerException::new("specs == null");
        }        
        try        {
            self.blockStarts[index]=specs;
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            throw IllegalArgumentException::new("bogus index");
        }
    }
    pub fn mergeStarts(&self, index: i32, specs: &RegisterSpecSet) -> boolean    {
        let start: RegisterSpecSet = getStarts0(index);
        let changed: boolean = false;
        if start==None        {
            setStarts(index, &specs);
            return true;
        }        
        let newStart: RegisterSpecSet = start.mutableCopy();
        newStart.intersect(&specs, true);
        if start.equals(&newStart)        {
            return false;
        }        
        newStart.setImmutable();
        setStarts(index, &newStart);
        return true;
    }
    pub fn getStarts(&self, index: i32) -> RegisterSpecSet    {
        let result: RegisterSpecSet = getStarts0(index);
        return if (result!=None) { result } else { self.emptySet };
            }
            pub fn getStarts(&self, block: &SsaBasicBlock) -> RegisterSpecSet            {
                return getStarts_int(block.getIndex());
            }
            pub fn mutableCopyOfStarts(&self, index: i32) -> RegisterSpecSet            {
                let result: RegisterSpecSet = getStarts0(index);
                return if (result!=None) { result.mutableCopy() } else { RegisterSpecSet::new(self.regCount) };
                    }
                    pub fn addAssignment(&self, insn: &SsaInsn, spec: &RegisterSpec)                    {
                        throwIfImmutable();
                        if insn==None                        {
                            throw NullPointerException::new("insn == null");
                        }                        
                        if spec==None                        {
                            throw NullPointerException::new("spec == null");
                        }                        
                        self.insnAssignments.put(&insn, &spec);
                    }
                    pub fn getAssignment(&self, insn: &SsaInsn) -> RegisterSpec                    {
                        return self.insnAssignments.get(&insn);
                    }
                    pub fn getAssignmentCount(&self) -> i32                    {
                        return self.insnAssignments.size();
                    }
                    pub fn debugDump(&self)                    {
                        for(                        let index: i32 = 0;;index<self.blockStarts.len()index += 1)                        {
                            if self.blockStarts[index]==None                            {
                                continue;
                            }                            
                            if self.blockStarts[index]==self.emptySet                            {
                                System::out.printf_String_Object[]("%04x: empty set\n", index);
                            }                            else                             {
                                System::out.printf_String_Object[]("%04x: %s\n", index, self.blockStarts[index]);
                            }
                        }
                    }
                    pub fn getStarts0(&self, index: i32) -> RegisterSpecSet                    {
                        try                        {
                            return self.blockStarts[index];
                        }                        catch(                        let ex: ArrayIndexOutOfBoundsException)                        {
                            throw IllegalArgumentException::new("bogus index");
                        }
                    }
}
