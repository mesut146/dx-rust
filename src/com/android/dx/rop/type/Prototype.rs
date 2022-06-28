use crate::helper;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::type::Type;

let static internTable: ConcurrentMap<String,Prototype> = ConcurrentHashMap<>::new(10_000, 0.75f);
struct Prototype{
    pub descriptor: String,
    pub returnType: Type,
    pub parameterTypes: StdTypeList,
    pub parameterFrameTypes: StdTypeList,
}
impl Prototype{
    pub fn intern(descriptor: &String) -> Prototype    {
        if descriptor==None        {
            throw NullPointerException::new("descriptor == null");
        }        
        let result: Prototype = Prototype::internTable.get(&descriptor);
        if result!=None        {
            return result;
        }        
        result=Prototype::fromDescriptor(&descriptor);
        return Prototype::putIntern(&result);
    }
    pub fn fromDescriptor(descriptor: &String) -> Prototype    {
        let result: Prototype = Prototype::internTable.get(&descriptor);
        if result!=None        {
            return result;
        }        
        let params: Vec<Type> = Prototype::makeParameterArray(&descriptor);
        let paramCount: i32 = 0;
        let at: i32 = 1;
        for(;)        {
            let startAt: i32 = at;
            let c: char = descriptor.charAt(at);
            if c==')'            {
                at += 1;
                break;
            }            
            while c=='['            {
                at += 1;
                c=descriptor.charAt(at);
            }
            if c=='L'            {
                let endAt: i32 = descriptor.indexOf_int_int(';', at);
                if endAt==-1                {
                    throw IllegalArgumentException::new("bad descriptor");
                }                
                at=endAt+1;
            }            else             {
                at += 1;
            }
            params[paramCount]=Type::intern(descriptor.substring_int_int(startAt, at));
            paramCount += 1;
        }
        let returnType: Type = Type::internReturnType(descriptor.substring_int(at));
        let parameterTypes: StdTypeList = StdTypeList::new(paramCount);
        for(        let i: i32 = 0;;i<paramCounti += 1)        {
            parameterTypes.set(i, params[i]);
        }
        return Prototype::new(&descriptor, &returnType, &parameterTypes);
    }
    pub fn clearInternTable()    {
        Prototype::internTable.clear();
    }
    pub fn makeParameterArray(descriptor: &String) -> Vec<Type>    {
        let length: i32 = descriptor.length();
        if descriptor.charAt(0)!='('        {
            throw IllegalArgumentException::new("bad descriptor");
        }        
        let closeAt: i32 = 0;
        let maxParams: i32 = 0;
        for(        let i: i32 = 1;;i<lengthi += 1)        {
            let c: char = descriptor.charAt(i);
            if c==')'            {
                closeAt=i;
                break;
            }            
            if (c>='A')&&(c<='Z')            {
                maxParams += 1;
            }            
        }
        if (closeAt==0)||(closeAt==(length-1))        {
            throw IllegalArgumentException::new("bad descriptor");
        }        
        if descriptor.indexOf_int_int(')', closeAt+1)!=-1        {
            throw IllegalArgumentException::new("bad descriptor");
        }        
        return new Type[maxParams];
    }
    pub fn intern(descriptor: &String, definer: &Type, isStatic: boolean, isInit: boolean) -> Prototype    {
        let base: Prototype = Prototype::intern_String(&descriptor);
        if isStatic        {
            return base;
        }        
        if isInit        {
            definer=definer.asUninitialized(Integer::MAX_VALUE);
        }        
        return base.withFirstParameter(&definer);
    }
    pub fn internInts(returnType: &Type, count: i32) -> Prototype    {
        let sb: StringBuilder = StringBuilder::new(100);
        sb.append_char('(');
        for(        let i: i32 = 0;;i<counti += 1)        {
            sb.append_char('I');
        }
        sb.append_char(')');
        sb.append_String(returnType.getDescriptor());
        return Prototype::intern_String(sb.toString());
    }
    pub fn new(&self, descriptor: &String, returnType: &Type, parameterTypes: &StdTypeList)    {
        if descriptor==None        {
            throw NullPointerException::new("descriptor == null");
        }        
        if returnType==None        {
            throw NullPointerException::new("returnType == null");
        }        
        if parameterTypes==None        {
            throw NullPointerException::new("parameterTypes == null");
        }        
        self->descriptor=descriptor;
        self->returnType=returnType;
        self->parameterTypes=parameterTypes;
        self->parameterFrameTypes=None;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if self==other        {
            return true;
        }        
        if !(//other instanceof Prototype)        {
            return false;
        }        
        return self.descriptor.equals(((Prototype*)other)->descriptor);
    }
    pub fn hashCode(&self) -> i32    {
        return self.descriptor.hashCode();
    }
    pub fn compareTo(&self, other: &Prototype) -> i32    {
        if self==other        {
            return 0;
        }        
        let result: i32 = self.returnType.compareTo(&);
        if result!=0        {
            return result;
        }        
        let thisSize: i32 = self.parameterTypes.size();
        let otherSize: i32 = .size();
        let size: i32 = Math::min_int_int(thisSize, otherSize);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let thisType: Type = self.parameterTypes.get(i);
            let otherType: Type = .get(i);
            result=thisType.compareTo(&otherType);
            if result!=0            {
                return result;
            }            
        }
        if thisSize<otherSize        {
            return -1;
        }        else         if thisSize>otherSize        {
            return 1;
        }        else         {
            return 0;
        }
    }
    pub fn toString(&self) -> String    {
        return self.descriptor;
    }
    pub fn getDescriptor(&self) -> String    {
        return self.descriptor;
    }
    pub fn getReturnType(&self) -> Type    {
        return self.returnType;
    }
    pub fn getParameterTypes(&self) -> StdTypeList    {
        return self.parameterTypes;
    }
    pub fn getParameterFrameTypes(&self) -> StdTypeList    {
        if self.parameterFrameTypes==None        {
            let sz: i32 = self.parameterTypes.size();
            let list: StdTypeList = StdTypeList::new(sz);
            let any: boolean = false;
            for(            let i: i32 = 0;;i<szi += 1)            {
                let one: Type = self.parameterTypes.get(i);
                if one.isIntlike()                {
                    any=true;
                    one=Type::INT;
                }                
                list.set(i, &one);
            }
            self.parameterFrameTypes=if any { list } else { self.parameterTypes };
                }                
                return self.parameterFrameTypes;
            }
            pub fn withFirstParameter(&self, param: &Type) -> Prototype            {
                let newDesc: String = "("+param.getDescriptor()+self.descriptor.substring_int(1);
                let newParams: StdTypeList = self.parameterTypes.withFirst(&param);
                newParams.setImmutable();
                let result: Prototype = Prototype::new(&newDesc, &self.returnType, &newParams);
                return Prototype::putIntern(&result);
            }
            pub fn putIntern(desc: &Prototype) -> Prototype            {
                let result: Prototype = Prototype::internTable.putIfAbsent(desc.getDescriptor(), &desc);
                return if result!=None { result } else { desc };
                    }
}
