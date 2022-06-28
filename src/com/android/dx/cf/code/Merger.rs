use crate::helper;
use crate::com::android::dx::cf::code::OneLocalsArray;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::cf::code::ExecutionStack;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::type::Type;

struct Merger{
}
impl Merger{
    pub fn new(&self)    {
    }
    pub fn mergeLocals(locals1: &OneLocalsArray, locals2: &OneLocalsArray) -> OneLocalsArray    {
        if locals1==locals2        {
            return locals1;
        }        
        let sz: i32 = locals1.getMaxLocals();
        let result: OneLocalsArray = None;
        if locals2.getMaxLocals()!=sz        {
            throw SimException::new("mismatched maxLocals values");
        }        
        for(        let i: i32 = 0;;i<szi += 1)        {
            let tb1: TypeBearer = locals1.getOrNull(i);
            let tb2: TypeBearer = locals2.getOrNull(i);
            let resultType: TypeBearer = Merger::mergeType(&tb1, &tb2);
            if resultType!=tb1            {
                if result==None                {
                    result=locals1.copy();
                }                
                if resultType==None                {
                    result.invalidate(i);
                }                else                 {
                    result.set_int_TypeBearer(i, &resultType);
                }
            }            
        }
        if result==None        {
            return locals1;
        }        
        result.setImmutable();
        return result;
    }
    pub fn mergeStack(stack1: &ExecutionStack, stack2: &ExecutionStack) -> ExecutionStack    {
        if stack1==stack2        {
            return stack1;
        }        
        let sz: i32 = stack1.size();
        let result: ExecutionStack = None;
        if stack2.size()!=sz        {
            throw SimException::new("mismatched stack depths");
        }        
        for(        let i: i32 = 0;;i<szi += 1)        {
            let tb1: TypeBearer = stack1.peek(i);
            let tb2: TypeBearer = stack2.peek(i);
            let resultType: TypeBearer = Merger::mergeType(&tb1, &tb2);
            if resultType!=tb1            {
                if result==None                {
                    result=stack1.copy();
                }                
                try                {
                    if resultType==None                    {
                        throw SimException::new("incompatible: "+tb1+", "+tb2);
                    }                    else                     {
                        result.change(i, &resultType);
                    }
                }                catch(                let ex: SimException)                {
                    ex.addContext("...while merging stack["+Hex::u2(i)+"]");
                    throw ex;
                }
            }            
        }
        if result==None        {
            return stack1;
        }        
        result.setImmutable();
        return result;
    }
    pub fn mergeType(ft1: &TypeBearer, ft2: &TypeBearer) -> TypeBearer    {
        if (ft1==None)||ft1.equals(&ft2)        {
            return ft1;
        }        else         if ft2==None        {
            return None;
        }        else         {
            let type1: Type = ft1.getType();
            let type2: Type = ft2.getType();
            if type1==type2            {
                return type1;
            }            else             if type1.isReference()&&type2.isReference()            {
                if type1==Type::KNOWN_NULL                {
                    return type2;
                }                else                 if type2==Type::KNOWN_NULL                {
                    return type1;
                }                else                 if type1.isArray()&&type2.isArray()                {
                    let componentUnion: TypeBearer = Merger::mergeType(type1.getComponentType(), type2.getComponentType());
                    if componentUnion==None                    {
                        return Type::OBJECT;
                    }                    
                    return ((Type*)componentUnion).getArrayType();
                }                else                 {
                    return Type::OBJECT;
                }
            }            else             if type1.isIntlike()&&type2.isIntlike()            {
                return Type::INT;
            }            else             {
                return None;
            }
        }
    }
    pub fn isPossiblyAssignableFrom(supertypeBearer: &TypeBearer, subtypeBearer: &TypeBearer) -> boolean    {
        let supertype: Type = supertypeBearer.getType();
        let subtype: Type = subtypeBearer.getType();
        if supertype.equals(&subtype)        {
            return true;
        }        
        let superBt: i32 = supertype.getBasicType();
        let subBt: i32 = subtype.getBasicType();
        if superBt==Type::BT_ADDR        {
            supertype=Type::OBJECT;
            superBt=Type::BT_OBJECT;
        }        
        if subBt==Type::BT_ADDR        {
            subtype=Type::OBJECT;
            subBt=Type::BT_OBJECT;
        }        
        if (superBt!=Type::BT_OBJECT)||(subBt!=Type::BT_OBJECT)        {
            return supertype.isIntlike()&&subtype.isIntlike();
        }        
        if supertype==Type::KNOWN_NULL        {
            return false;
        }        else         if subtype==Type::KNOWN_NULL        {
            return true;
        }        else         if supertype==Type::OBJECT        {
            return true;
        }        else         if supertype.isArray()        {
            if !subtype.isArray()            {
                return false;
            }            
            loop            {
                supertype=supertype.getComponentType();                subtype=subtype.getComponentType();                if !supertype.isArray()&&subtype.isArray(){ break; }            }
            return Merger::isPossiblyAssignableFrom(&supertype, &subtype);
        }        else         if subtype.isArray()        {
            return (supertype==Type::SERIALIZABLE)||(supertype==Type::CLONEABLE);
        }        else         {
            return true;
        }
    }
}
