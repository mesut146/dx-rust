use crate::helper;
use crate::com::android::dx::cf::code::Frame;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::cf::code::LocalsArraySet;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::cf::code::OneLocalsArray;
use crate::com::android::dx::cf::code::ExecutionStack;
use crate::com::android::dx::cf::code::LocalsArray;
use crate::com::android::dx::rop::type::Type;

struct Frame{
    pub locals: LocalsArray,
    pub stack: ExecutionStack,
    pub subroutines: IntList,
}
impl Frame{
    pub fn new(&self, locals: &LocalsArray, stack: &ExecutionStack)    {
        this(locals,stack,IntList.EMPTY);

    }
    pub fn new(&self, locals: &LocalsArray, stack: &ExecutionStack, subroutines: &IntList)    {
        if locals==None        {
            throw NullPointerException::new("locals == null");
        }        
        if stack==None        {
            throw NullPointerException::new("stack == null");
        }        
        subroutines.throwIfMutable();
        self->locals=locals;
        self->stack=stack;
        self->subroutines=subroutines;
    }
    pub fn new(&self, maxLocals: i32, maxStack: i32)    {
        this(new OneLocalsArray(maxLocals),new ExecutionStack(maxStack));

    }
    pub fn copy(&self) -> Frame    {
        return Frame::new(self.locals.copy(), self.stack.copy(), &self.subroutines);
    }
    pub fn setImmutable(&self)    {
        self.locals.setImmutable();
        self.stack.setImmutable();
    }
    pub fn makeInitialized(&self, type: &Type)    {
        self.locals.makeInitialized(&type);
        self.stack.makeInitialized(&type);
    }
    pub fn getLocals(&self) -> LocalsArray    {
        return self.locals;
    }
    pub fn getStack(&self) -> ExecutionStack    {
        return self.stack;
    }
    pub fn getSubroutines(&self) -> IntList    {
        return self.subroutines;
    }
    pub fn initializeWithParameters(&self, params: &StdTypeList)    {
        let at: i32 = 0;
        let sz: i32 = params.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Type = params.get(i);
            self.locals.set_int_TypeBearer(at, &one);
            at+=one.getCategory();
        }
    }
    pub fn subFrameForLabel(&self, startLabel: i32, subLabel: i32) -> Frame    {
        let subLocals: LocalsArray = None;
        if //locals instanceof LocalsArraySet        {
            subLocals=((LocalsArraySet*)self.locals).subArrayForLabel(subLabel);
        }        
        let newSubroutines: IntList;
        try        {
            newSubroutines=self.subroutines.mutableCopy();
            if newSubroutines.pop()!=startLabel            {
                throw RuntimeException::new("returning from invalid subroutine");
            }            
            newSubroutines.setImmutable();
        }        catch(        let ex: IndexOutOfBoundsException)        {
            throw RuntimeException::new("returning from invalid subroutine");
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("can't return from non-subroutine");
        }
        return if (subLocals==None) { None } else { Frame::new(&subLocals, &self.stack, &newSubroutines) };
            }
            pub fn mergeWith(&self, other: &Frame) -> Frame            {
                let resultLocals: LocalsArray;
                let resultStack: ExecutionStack;
                let resultSubroutines: IntList;
                resultLocals=getLocals().merge(other.getLocals());
                resultStack=getStack().merge(other.getStack());
                resultSubroutines=mergeSubroutineLists(&);
                resultLocals=Frame::adjustLocalsForSubroutines(&resultLocals, &resultSubroutines);
                if (resultLocals==getLocals())&&(resultStack==getStack())&&self.subroutines==resultSubroutines                {
                    return self;
                }                
                return Frame::new(&resultLocals, &resultStack, &resultSubroutines);
            }
            pub fn mergeSubroutineLists(&self, otherSubroutines: &IntList) -> IntList            {
                if self.subroutines.equals(&otherSubroutines)                {
                    return self.subroutines;
                }                
                let resultSubroutines: IntList = IntList::new();
                let szSubroutines: i32 = self.subroutines.size();
                let szOthers: i32 = otherSubroutines.size();
                for(                let i: i32 = 0;;i<szSubroutines&&i<szOthers&&(self.subroutines.get(i)==otherSubroutines.get(i))i += 1)                {
                    resultSubroutines.add(i);
                }
                resultSubroutines.setImmutable();
                return resultSubroutines;
            }
            pub fn adjustLocalsForSubroutines(locals: &LocalsArray, subroutines: &IntList) -> LocalsArray            {
                if !(//locals instanceof LocalsArraySet)                {
                    return locals;
                }                
                let laSet: LocalsArraySet = (LocalsArraySet*)locals;
                if subroutines.size()==0                {
                    return laSet.getPrimary();
                }                
                return laSet;
            }
            pub fn mergeWithSubroutineCaller(&self, other: &Frame, subLabel: i32, predLabel: i32) -> Frame            {
                let resultLocals: LocalsArray;
                let resultStack: ExecutionStack;
                resultLocals=getLocals().mergeWithSubroutineCaller(other.getLocals(), predLabel);
                resultStack=getStack().merge(other.getStack());
                let newOtherSubroutines: IntList = .mutableCopy();
                newOtherSubroutines.add(subLabel);
                newOtherSubroutines.setImmutable();
                if (resultLocals==getLocals())&&(resultStack==getStack())&&self.subroutines.equals(&newOtherSubroutines)                {
                    return self;
                }                
                let resultSubroutines: IntList;
                if self.subroutines.equals(&newOtherSubroutines)                {
                    resultSubroutines=self.subroutines;
                }                else                 {
                    let nonResultSubroutines: IntList;
                    if self.subroutines.size()>newOtherSubroutines.size()                    {
                        resultSubroutines=self.subroutines;
                        nonResultSubroutines=newOtherSubroutines;
                    }                    else                     {
                        resultSubroutines=newOtherSubroutines;
                        nonResultSubroutines=self.subroutines;
                    }
                    let szResult: i32 = resultSubroutines.size();
                    let szNonResult: i32 = nonResultSubroutines.size();
                    for(                    let i: i32 = szNonResult-1;;i>=0i -= 1)                    {
                        if nonResultSubroutines.get(i)!=resultSubroutines.get(i+(szResult-szNonResult))                        {
                            throw RuntimeException::new("Incompatible merged subroutines");
                        }                        
                    }
                }
                return Frame::new(&resultLocals, &resultStack, &resultSubroutines);
            }
            pub fn makeNewSubroutineStartFrame(&self, subLabel: i32, callerLabel: i32) -> Frame            {
                let newSubroutines: IntList = self.subroutines.mutableCopy();
                newSubroutines.add(subLabel);
                let newFrame: Frame = Frame::new(self.locals.getPrimary(), &self.stack, IntList::makeImmutable_int(subLabel));
                return newFrame.mergeWithSubroutineCaller(self, subLabel, callerLabel);
            }
            pub fn makeExceptionHandlerStartFrame(&self, exceptionClass: &CstType) -> Frame            {
                let newStack: ExecutionStack = getStack().copy();
                newStack.clear();
                newStack.push(&exceptionClass);
                return Frame::new(getLocals(), &newStack, &self.subroutines);
            }
            pub fn annotate(&self, ex: &ExceptionWithContext)            {
                self.locals.annotate(&ex);
                self.stack.annotate(&ex);
            }
}
