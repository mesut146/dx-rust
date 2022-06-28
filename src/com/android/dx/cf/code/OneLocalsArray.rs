use crate::helper;
use crate::com::android::dx::cf::code::OneLocalsArray;
use crate::com::android::dx::cf::code::LocalsArray;
use crate::com::android::dx::cf::code::Merger;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::cf::code::LocalsArraySet;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::rop::type::Type;

struct OneLocalsArray{
    pub locals: Vec<TypeBearer>,
}
impl OneLocalsArray{
    pub fn new(&self, maxLocals: i32)    {
        super(maxLocals != 0);

        self.locals=new TypeBearer[maxLocals];
    }
    pub fn copy(&self) -> OneLocalsArray    {
        let result: OneLocalsArray = OneLocalsArray::new(self.locals.len());
        System::arraycopy(&self.locals, 0, &, 0, self.locals.len());
        return result;
    }
    pub fn annotate(&self, ex: &ExceptionWithContext)    {
        for(        let i: i32 = 0;;i<self.locals.len()i += 1)        {
            let type: TypeBearer = self.locals[i];
            let s: String = if (type_renamed==None) { "<invalid>" } else { type_renamed.toString() };
                    ex.addContext("locals["+Hex::u2(i)+"]: "+s);
                }
            }
            pub fn toHuman(&self) -> String            {
                let sb: StringBuilder = StringBuilder::new();
                for(                let i: i32 = 0;;i<self.locals.len()i += 1)                {
                    let type: TypeBearer = self.locals[i];
                    let s: String = if (type_renamed==None) { "<invalid>" } else { type_renamed.toString() };
                            sb.append_String("locals["+Hex::u2(i)+"]: "+s+"\n");
                        }
                        return sb.toString();
                    }
                    pub fn makeInitialized(&self, type: &Type)                    {
                        let len: i32 = self.locals.len();
                        if len==0                        {
                            return;
                        }                        
                        throwIfImmutable();
                        let initializedType: Type = type.getInitializedType();
                        for(                        let i: i32 = 0;;i<leni += 1)                        {
                            if self.locals[i]==type                            {
                                self.locals[i]=initializedType;
                            }                            
                        }
                    }
                    pub fn getMaxLocals(&self) -> i32                    {
                        return self.locals.len();
                    }
                    pub fn set(&self, idx: i32, type: &TypeBearer)                    {
                        throwIfImmutable();
                        try                        {
                            type=type.getFrameType();
                        }                        catch(                        let ex: NullPointerException)                        {
                            throw NullPointerException::new("type == null");
                        }
                        if idx<0                        {
                            throw IndexOutOfBoundsException::new("idx < 0");
                        }                        
                        if type.getType().isCategory2()                        {
                            self.locals[idx+1]=None;
                        }                        
                        self.locals[idx]=type;
                        if idx!=0                        {
                            let prev: TypeBearer = self.locals[idx-1];
                            if (prev!=None)&&prev.getType().isCategory2()                            {
                                self.locals[idx-1]=None;
                            }                            
                        }                        
                    }
                    pub fn set(&self, spec: &RegisterSpec)                    {
                        set_int_TypeBearer(spec.getReg(), &spec);
                    }
                    pub fn invalidate(&self, idx: i32)                    {
                        throwIfImmutable();
                        self.locals[idx]=None;
                    }
                    pub fn getOrNull(&self, idx: i32) -> TypeBearer                    {
                        return self.locals[idx];
                    }
                    pub fn get(&self, idx: i32) -> TypeBearer                    {
                        let result: TypeBearer = self.locals[idx];
                        if result==None                        {
                            return OneLocalsArray::throwSimException(idx, "invalid");
                        }                        
                        return result;
                    }
                    pub fn getCategory1(&self, idx: i32) -> TypeBearer                    {
                        let result: TypeBearer = get(idx);
                        let type: Type = result.getType();
                        if type_renamed.isUninitialized()                        {
                            return OneLocalsArray::throwSimException(idx, "uninitialized instance");
                        }                        
                        if type_renamed.isCategory2()                        {
                            return OneLocalsArray::throwSimException(idx, "category-2");
                        }                        
                        return result;
                    }
                    pub fn getCategory2(&self, idx: i32) -> TypeBearer                    {
                        let result: TypeBearer = get(idx);
                        if result.getType().isCategory1()                        {
                            return OneLocalsArray::throwSimException(idx, "category-1");
                        }                        
                        return result;
                    }
                    pub fn merge(&self, other: &LocalsArray) -> LocalsArray                    {
                        if //other instanceof OneLocalsArray                        {
                            return merge_OneLocalsArray((OneLocalsArray*)other);
                        }                        else                         {
                            return other.merge(self);
                        }
                    }
                    pub fn merge(&self, other: &OneLocalsArray) -> OneLocalsArray                    {
                        try                        {
                            return Merger::mergeLocals(self, &other);
                        }                        catch(                        let ex: SimException)                        {
                            ex.addContext("underlay locals:");
                            annotate(&ex);
                            ex.addContext("overlay locals:");
                            other.annotate(&ex);
                            throw ex;
                        }
                    }
                    pub fn mergeWithSubroutineCaller(&self, other: &LocalsArray, predLabel: i32) -> LocalsArraySet                    {
                        let result: LocalsArraySet = LocalsArraySet::new(getMaxLocals());
                        return result.mergeWithSubroutineCaller(&other, predLabel);
                    }
                    pub fn getPrimary(&self) -> OneLocalsArray                    {
                        return self;
                    }
                    pub fn throwSimException(idx: i32, msg: &String) -> TypeBearer                    {
                        throw SimException::new("local "+Hex::u2(idx)+": "+msg);
                    }
}
