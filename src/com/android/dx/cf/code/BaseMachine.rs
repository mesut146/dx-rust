use crate::helper;
use crate::com::android::dx::cf::code::Frame;
use crate::com::android::dx::cf::code::ExecutionStack;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::cf::code::LocalsArray;
use crate::com::android::dx::cf::code::Merger;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::type::Type;

struct BaseMachine{
    pub prototype: Prototype,
    pub args: Vec<TypeBearer>,
    pub argCount: i32,
    pub auxType: Type,
    pub auxInt: i32,
    pub auxCst: Constant,
    pub auxTarget: i32,
    pub auxCases: SwitchList,
    pub auxInitValues: ArrayList<Constant>,
    pub localIndex: i32,
    pub localInfo: boolean,
    pub localTarget: RegisterSpec,
    pub results: Vec<TypeBearer>,
    pub resultCount: i32,
}
impl BaseMachine{
    pub fn new(&self, prototype: &Prototype)    {
        if prototype==None        {
            throw NullPointerException::new("prototype == null");
        }        
        self->prototype=prototype;
        self.args=new TypeBearer[10];
        self.results=new TypeBearer[6];
        clearArgs();
    }
    pub fn getPrototype(&self) -> Prototype    {
        return self.prototype;
    }
    pub fn clearArgs(&self)    {
        self.argCount=0;
        self.auxType=None;
        self.auxInt=0;
        self.auxCst=None;
        self.auxTarget=0;
        self.auxCases=None;
        self.auxInitValues=None;
        self.localIndex=-1;
        self.localInfo=false;
        self.localTarget=None;
        self.resultCount=-1;
    }
    pub fn popArgs(&self, frame: &Frame, count: i32)    {
        let stack: ExecutionStack = frame.getStack();
        clearArgs();
        if count>self.args.len()        {
            self.args=new TypeBearer[count + 10];
        }        
        for(        let i: i32 = count-1;;i>=0i -= 1)        {
            self.args[i]=stack.pop();
        }
        self.argCount=count;
    }
    pub fn popArgs(&self, frame: &Frame, prototype: &Prototype)    {
        let types: StdTypeList = prototype.getParameterTypes();
        let size: i32 = types.size();
        popArgs_Frame_int(&frame, size);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            if !Merger::isPossiblyAssignableFrom(types.getType(i), self.args[i])            {
                throw SimException::new("at stack depth "+(size-1-i)+", expected type "+types.getType(i).toHuman()+" but found "+self.args[i].getType().toHuman());
            }            
        }
    }
    pub fn popArgs(&self, frame: &Frame, type: &Type)    {
        popArgs_Frame_int(&frame, 1);
        if !Merger::isPossiblyAssignableFrom(&type, self.args[0])        {
            throw SimException::new("expected type "+type.toHuman()+" but found "+self.args[0].getType().toHuman());
        }        
    }
    pub fn popArgs(&self, frame: &Frame, type1: &Type, type2: &Type)    {
        popArgs_Frame_int(&frame, 2);
        if !Merger::isPossiblyAssignableFrom(&type1, self.args[0])        {
            throw SimException::new("expected type "+type1.toHuman()+" but found "+self.args[0].getType().toHuman());
        }        
        if !Merger::isPossiblyAssignableFrom(&type2, self.args[1])        {
            throw SimException::new("expected type "+type2.toHuman()+" but found "+self.args[1].getType().toHuman());
        }        
    }
    pub fn popArgs(&self, frame: &Frame, type1: &Type, type2: &Type, type3: &Type)    {
        popArgs_Frame_int(&frame, 3);
        if !Merger::isPossiblyAssignableFrom(&type1, self.args[0])        {
            throw SimException::new("expected type "+type1.toHuman()+" but found "+self.args[0].getType().toHuman());
        }        
        if !Merger::isPossiblyAssignableFrom(&type2, self.args[1])        {
            throw SimException::new("expected type "+type2.toHuman()+" but found "+self.args[1].getType().toHuman());
        }        
        if !Merger::isPossiblyAssignableFrom(&type3, self.args[2])        {
            throw SimException::new("expected type "+type3.toHuman()+" but found "+self.args[2].getType().toHuman());
        }        
    }
    pub fn localArg(&self, frame: &Frame, idx: i32)    {
        clearArgs();
        self.args[0]=frame.getLocals().get(idx);
        self.argCount=1;
        self.localIndex=idx;
    }
    pub fn localInfo(&self, local: boolean)    {
        self.localInfo=local;
    }
    pub fn auxType(&self, type: &Type)    {
        self.auxType=type;
    }
    pub fn auxIntArg(&self, value: i32)    {
        self.auxInt=value;
    }
    pub fn auxCstArg(&self, cst: &Constant)    {
        if cst==None        {
            throw NullPointerException::new("cst == null");
        }        
        self.auxCst=cst;
    }
    pub fn auxTargetArg(&self, target: i32)    {
        self.auxTarget=target;
    }
    pub fn auxSwitchArg(&self, cases: &SwitchList)    {
        if cases==None        {
            throw NullPointerException::new("cases == null");
        }        
        self.auxCases=cases;
    }
    pub fn auxInitValues(&self, initValues: &ArrayList<Constant>)    {
        self.auxInitValues=initValues;
    }
    pub fn localTarget(&self, idx: i32, type: &Type, local: &LocalItem)    {
        self.localTarget=RegisterSpec::makeLocalOptional(idx, &type, &local);
    }
    pub fn argCount(&self) -> i32    {
        return self.argCount;
    }
    pub fn argWidth(&self) -> i32    {
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<self.argCounti += 1)        {
            result+=self.args[i].getType().getCategory();
        }
        return result;
    }
    pub fn arg(&self, n: i32) -> TypeBearer    {
        if n>=self.argCount        {
            throw IllegalArgumentException::new("n >= argCount");
        }        
        try        {
            return self.args[n];
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            throw IllegalArgumentException::new("n < 0");
        }
    }
    pub fn getAuxType(&self) -> Type    {
        return self.auxType;
    }
    pub fn getAuxInt(&self) -> i32    {
        return self.auxInt;
    }
    pub fn getAuxCst(&self) -> Constant    {
        return self.auxCst;
    }
    pub fn getAuxTarget(&self) -> i32    {
        return self.auxTarget;
    }
    pub fn getAuxCases(&self) -> SwitchList    {
        return self.auxCases;
    }
    pub fn getInitValues(&self) -> ArrayList<Constant>    {
        return self.auxInitValues;
    }
    pub fn getLocalIndex(&self) -> i32    {
        return self.localIndex;
    }
    pub fn getLocalInfo(&self) -> boolean    {
        return self.localInfo;
    }
    pub fn getLocalTarget(&self, isMove: boolean) -> RegisterSpec    {
        if self.localTarget==None        {
            return None;
        }        
        if self.resultCount!=1        {
            throw SimException::new("local target with "+(if (self.resultCount==0) { "no" } else { "multiple" })+" results");
                }                
                let result: TypeBearer = self.results[0];
                let resultType: Type = result.getType();
                let localType: Type = self.localTarget.getType();
                if resultType==localType                {
                    if isMove                    {
                        return self.localTarget.withType(&result);
                    }                    else                     {
                        return self.localTarget;
                    }
                }                
                if !Merger::isPossiblyAssignableFrom(&localType, &resultType)                {
                    BaseMachine::throwLocalMismatch(&resultType, &localType);
                    return None;
                }                
                if localType==Type::OBJECT                {
                    self.localTarget=self.localTarget.withType(&result);
                }                
                return self.localTarget;
            }
            pub fn clearResult(&self)            {
                self.resultCount=0;
            }
            pub fn setResult(&self, result: &TypeBearer)            {
                if result==None                {
                    throw NullPointerException::new("result == null");
                }                
                self.results[0]=result;
                self.resultCount=1;
            }
            pub fn addResult(&self, result: &TypeBearer)            {
                if result==None                {
                    throw NullPointerException::new("result == null");
                }                
                self.results[self.resultCount]=result;
                self.resultCount += 1;
            }
            pub fn resultCount(&self) -> i32            {
                if self.resultCount<0                {
                    throw SimException::new("results never set");
                }                
                return self.resultCount;
            }
            pub fn resultWidth(&self) -> i32            {
                let width: i32 = 0;
                for(                let i: i32 = 0;;i<self.resultCounti += 1)                {
                    width+=self.results[i].getType().getCategory();
                }
                return width;
            }
            pub fn result(&self, n: i32) -> TypeBearer            {
                if n>=self.resultCount                {
                    throw IllegalArgumentException::new("n >= resultCount");
                }                
                try                {
                    return self.results[n];
                }                catch(                let ex: ArrayIndexOutOfBoundsException)                {
                    throw IllegalArgumentException::new("n < 0");
                }
            }
            pub fn storeResults(&self, frame: &Frame)            {
                if self.resultCount<0                {
                    throw SimException::new("results never set");
                }                
                if self.resultCount==0                {
                    return;
                }                
                if self.localTarget!=None                {
                    frame.getLocals().set_RegisterSpec(getLocalTarget(false));
                }                else                 {
                    let stack: ExecutionStack = frame.getStack();
                    for(                    let i: i32 = 0;;i<self.resultCounti += 1)                    {
                        if self.localInfo                        {
                            stack.setLocal();
                        }                        
                        stack.push(self.results[i]);
                    }
                }
            }
            pub fn throwLocalMismatch(found: &TypeBearer, local: &TypeBearer)            {
                throw SimException::new("local variable type mismatch: "+"attempt to set or access a value of type "+found.toHuman()+" using a local variable of type "+local.toHuman()+". This is symptomatic of .class transformation tools "+"that ignore local variable information.");
            }
}
