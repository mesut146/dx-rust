use crate::helper;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::type::Type;

let static EMPTY: StdTypeList = StdTypeList::new(0);
let static INT: StdTypeList = StdTypeList::make_Type(&Type::INT);
let static LONG: StdTypeList = StdTypeList::make_Type(&Type::LONG);
let static FLOAT: StdTypeList = StdTypeList::make_Type(&Type::FLOAT);
let static DOUBLE: StdTypeList = StdTypeList::make_Type(&Type::DOUBLE);
let static OBJECT: StdTypeList = StdTypeList::make_Type(&Type::OBJECT);
let static RETURN_ADDRESS: StdTypeList = StdTypeList::make_Type(&Type::RETURN_ADDRESS);
let static THROWABLE: StdTypeList = StdTypeList::make_Type(&Type::THROWABLE);
let static INT_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::INT, &Type::INT);
let static LONG_LONG: StdTypeList = StdTypeList::make_Type_Type(&Type::LONG, &Type::LONG);
let static FLOAT_FLOAT: StdTypeList = StdTypeList::make_Type_Type(&Type::FLOAT, &Type::FLOAT);
let static DOUBLE_DOUBLE: StdTypeList = StdTypeList::make_Type_Type(&Type::DOUBLE, &Type::DOUBLE);
let static OBJECT_OBJECT: StdTypeList = StdTypeList::make_Type_Type(&Type::OBJECT, &Type::OBJECT);
let static INT_OBJECT: StdTypeList = StdTypeList::make_Type_Type(&Type::INT, &Type::OBJECT);
let static LONG_OBJECT: StdTypeList = StdTypeList::make_Type_Type(&Type::LONG, &Type::OBJECT);
let static FLOAT_OBJECT: StdTypeList = StdTypeList::make_Type_Type(&Type::FLOAT, &Type::OBJECT);
let static DOUBLE_OBJECT: StdTypeList = StdTypeList::make_Type_Type(&Type::DOUBLE, &Type::OBJECT);
let static LONG_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::LONG, &Type::INT);
let static INTARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::INT_ARRAY, &Type::INT);
let static LONGARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::LONG_ARRAY, &Type::INT);
let static FLOATARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::FLOAT_ARRAY, &Type::INT);
let static DOUBLEARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::DOUBLE_ARRAY, &Type::INT);
let static OBJECTARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::OBJECT_ARRAY, &Type::INT);
let static BOOLEANARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::BOOLEAN_ARRAY, &Type::INT);
let static BYTEARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::BYTE_ARRAY, &Type::INT);
let static CHARARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::CHAR_ARRAY, &Type::INT);
let static SHORTARR_INT: StdTypeList = StdTypeList::make_Type_Type(&Type::SHORT_ARRAY, &Type::INT);
let static INT_INTARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::INT, &Type::INT_ARRAY, &Type::INT);
let static LONG_LONGARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::LONG, &Type::LONG_ARRAY, &Type::INT);
let static FLOAT_FLOATARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::FLOAT, &Type::FLOAT_ARRAY, &Type::INT);
let static DOUBLE_DOUBLEARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::DOUBLE, &Type::DOUBLE_ARRAY, &Type::INT);
let static OBJECT_OBJECTARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::OBJECT, &Type::OBJECT_ARRAY, &Type::INT);
let static INT_BOOLEANARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::INT, &Type::BOOLEAN_ARRAY, &Type::INT);
let static INT_BYTEARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::INT, &Type::BYTE_ARRAY, &Type::INT);
let static INT_CHARARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::INT, &Type::CHAR_ARRAY, &Type::INT);
let static INT_SHORTARR_INT: StdTypeList = StdTypeList::make_Type_Type_Type(&Type::INT, &Type::SHORT_ARRAY, &Type::INT);
struct StdTypeList{
}
impl StdTypeList{
    pub fn make(type: &Type) -> StdTypeList    {
        let result: StdTypeList = StdTypeList::new(1);
        result.set(0, &type);
        return result;
    }
    pub fn make(type0: &Type, type1: &Type) -> StdTypeList    {
        let result: StdTypeList = StdTypeList::new(2);
        result.set(0, &type0);
        result.set(1, &type1);
        return result;
    }
    pub fn make(type0: &Type, type1: &Type, type2: &Type) -> StdTypeList    {
        let result: StdTypeList = StdTypeList::new(3);
        result.set(0, &type0);
        result.set(1, &type1);
        result.set(2, &type2);
        return result;
    }
    pub fn make(type0: &Type, type1: &Type, type2: &Type, type3: &Type) -> StdTypeList    {
        let result: StdTypeList = StdTypeList::new(4);
        result.set(0, &type0);
        result.set(1, &type1);
        result.set(2, &type2);
        result.set(3, &type3);
        return result;
    }
    pub fn toHuman(list: &TypeList) -> String    {
        let size: i32 = list.size();
        if size==0        {
            return "<empty>";
        }        
        let sb: StringBuilder = StringBuilder::new(100);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            if i!=0            {
                sb.append_String(", ");
            }            
            sb.append_String(list.getType(i).toHuman());
        }
        return sb.toString();
    }
    pub fn hashContents(list: &TypeList) -> i32    {
        let size: i32 = list.size();
        let hash: i32 = 0;
        for(        let i: i32 = 0;;i<sizei += 1)        {
            hash=(hash*31)+list.getType(i).hashCode();
        }
        return hash;
    }
    pub fn equalContents(list1: &TypeList, list2: &TypeList) -> boolean    {
        let size: i32 = list1.size();
        if list2.size()!=size        {
            return false;
        }        
        for(        let i: i32 = 0;;i<sizei += 1)        {
            if !list1.getType(i).equals(list2.getType(i))            {
                return false;
            }            
        }
        return true;
    }
    pub fn compareContents(list1: &TypeList, list2: &TypeList) -> i32    {
        let size1: i32 = list1.size();
        let size2: i32 = list2.size();
        let size: i32 = Math::min_int_int(size1, size2);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let comparison: i32 = list1.getType(i).compareTo(list2.getType(i));
            if comparison!=0            {
                return comparison;
            }            
        }
        if size1==size2        {
            return 0;
        }        else         if size1<size2        {
            return -1;
        }        else         {
            return 1;
        }
    }
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn getType(&self, n: i32) -> Type    {
        return get(n);
    }
    pub fn getWordCount(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            result+=get(i).getCategory();
        }
        return result;
    }
    pub fn withAddedType(&self, type: &Type) -> TypeList    {
        let sz: i32 = size();
        let result: StdTypeList = StdTypeList::new(sz+1);
        for(        let i: i32 = 0;;i<szi += 1)        {
            result.set0(i, get0(i));
        }
        result.set(sz, &type);
        result.setImmutable();
        return result;
    }
    pub fn get(&self, n: i32) -> Type    {
        return (Type*)get0(n);
    }
    pub fn set(&self, n: i32, type: &Type)    {
        set0(n, &type);
    }
    pub fn withFirst(&self, type: &Type) -> StdTypeList    {
        let sz: i32 = size();
        let result: StdTypeList = StdTypeList::new(sz+1);
        result.set0(0, &type);
        for(        let i: i32 = 0;;i<szi += 1)        {
            result.set0(i+1, getOrNull0(i));
        }
        return result;
    }
}
