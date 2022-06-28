use crate::helper;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::cf::code::ExecutionStack;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::cf::code::Merger;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

struct ExecutionStack{
    pub stack: Vec<TypeBearer>,
    pub local: Vec<boolean>,
    pub stackPtr: i32,
}
impl ExecutionStack{
    pub fn new(&self, maxStack: i32)    {
        super(maxStack != 0);

        self.stack=new TypeBearer[maxStack];
        self.local=new boolean[maxStack];
        self.stackPtr=0;
    }
    pub fn copy(&self) -> ExecutionStack    {
        let result: ExecutionStack = ExecutionStack::new(self.stack.len());
        System::arraycopy(&self.stack, 0, &, 0, self.stack.len());
        System::arraycopy(&self.local, 0, &, 0, self.local.len());
        =self.stackPtr;
        return result;
    }
    pub fn annotate(&self, ex: &ExceptionWithContext)    {
        let limit: i32 = self.stackPtr-1;
        for(        let i: i32 = 0;;i<=limiti += 1)        {
            let idx: String = if (i==limit) { "top0" } else { Hex::u2(limit-i) };
                    ex.addContext("stack["+idx+"]: "+ExecutionStack::stackElementString(self.stack[i]));
                }
            }
            pub fn makeInitialized(&self, type: &Type)            {
                if self.stackPtr==0                {
                    return;
                }                
                throwIfImmutable();
                let initializedType: Type = type.getInitializedType();
                for(                let i: i32 = 0;;i<self.stackPtri += 1)                {
                    if self.stack[i]==type                    {
                        self.stack[i]=initializedType;
                    }                    
                }
            }
            pub fn getMaxStack(&self) -> i32            {
                return self.stack.len();
            }
            pub fn size(&self) -> i32            {
                return self.stackPtr;
            }
            pub fn clear(&self)            {
                throwIfImmutable();
                for(                let i: i32 = 0;;i<self.stackPtri += 1)                {
                    self.stack[i]=None;
                    self.local[i]=false;
                }
                self.stackPtr=0;
            }
            pub fn push(&self, type: &TypeBearer)            {
                throwIfImmutable();
                let category: i32;
                try                {
                    type=type.getFrameType();
                    category=type.getType().getCategory();
                }                catch(                let ex: NullPointerException)                {
                    throw NullPointerException::new("type == null");
                }
                if (self.stackPtr+category)>self.stack.len()                {
                    ExecutionStack::throwSimException("overflow");
                    return;
                }                
                if category==2                {
                    self.stack[self.stackPtr]=None;
                    self.stackPtr += 1;
                }                
                self.stack[self.stackPtr]=type;
                self.stackPtr += 1;
            }
            pub fn setLocal(&self)            {
                throwIfImmutable();
                self.local[self.stackPtr]=true;
            }
            pub fn peek(&self, n: i32) -> TypeBearer            {
                if n<0                {
                    throw IllegalArgumentException::new("n < 0");
                }                
                if n>=self.stackPtr                {
                    return ExecutionStack::throwSimException("underflow");
                }                
                return self.stack[self.stackPtr-n-1];
            }
            pub fn peekLocal(&self, n: i32) -> boolean            {
                if n<0                {
                    throw IllegalArgumentException::new("n < 0");
                }                
                if n>=self.stackPtr                {
                    throw SimException::new("stack: underflow");
                }                
                return self.local[self.stackPtr-n-1];
            }
            pub fn peekType(&self, n: i32) -> Type            {
                return peek(n).getType();
            }
            pub fn pop(&self) -> TypeBearer            {
                throwIfImmutable();
                let result: TypeBearer = peek(0);
                self.stack[self.stackPtr-1]=None;
                self.local[self.stackPtr-1]=false;
                self.stackPtr-=result.getType().getCategory();
                return result;
            }
            pub fn change(&self, n: i32, type: &TypeBearer)            {
                throwIfImmutable();
                try                {
                    type=type.getFrameType();
                }                catch(                let ex: NullPointerException)                {
                    throw NullPointerException::new("type == null");
                }
                let idx: i32 = self.stackPtr-n-1;
                let orig: TypeBearer = self.stack[idx];
                if (orig==None)||(orig.getType().getCategory()!=type.getType().getCategory())                {
                    ExecutionStack::throwSimException("incompatible substitution: "+ExecutionStack::stackElementString(&orig)+" -> "+ExecutionStack::stackElementString(&type));
                }                
                self.stack[idx]=type;
            }
            pub fn merge(&self, other: &ExecutionStack) -> ExecutionStack            {
                try                {
                    return Merger::mergeStack(self, &other);
                }                catch(                let ex: SimException)                {
                    ex.addContext("underlay stack:");
                    self.annotate(&ex);
                    ex.addContext("overlay stack:");
                    other.annotate(&ex);
                    throw ex;
                }
            }
            pub fn stackElementString(type: &TypeBearer) -> String            {
                if type==None                {
                    return "<invalid>";
                }                
                return type.toString();
            }
            pub fn throwSimException(msg: &String) -> TypeBearer            {
                throw SimException::new("stack: "+msg);
            }
}
