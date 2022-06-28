use crate::helper;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;

let static interns: ConcurrentMap<Type,CstType> = ConcurrentHashMap<>::new(1_000, 0.75f);
let static OBJECT: CstType = CstType::new(&Type::OBJECT);
let static BOOLEAN: CstType = CstType::new(&Type::BOOLEAN_CLASS);
let static BYTE: CstType = CstType::new(&Type::BYTE_CLASS);
let static CHARACTER: CstType = CstType::new(&Type::CHARACTER_CLASS);
let static DOUBLE: CstType = CstType::new(&Type::DOUBLE_CLASS);
let static FLOAT: CstType = CstType::new(&Type::FLOAT_CLASS);
let static LONG: CstType = CstType::new(&Type::LONG_CLASS);
let static INTEGER: CstType = CstType::new(&Type::INTEGER_CLASS);
let static SHORT: CstType = CstType::new(&Type::SHORT_CLASS);
let static VOID: CstType = CstType::new(&Type::VOID_CLASS);
let static BOOLEAN_ARRAY: CstType = CstType::new(&Type::BOOLEAN_ARRAY);
let static BYTE_ARRAY: CstType = CstType::new(&Type::BYTE_ARRAY);
let static CHAR_ARRAY: CstType = CstType::new(&Type::CHAR_ARRAY);
let static DOUBLE_ARRAY: CstType = CstType::new(&Type::DOUBLE_ARRAY);
let static FLOAT_ARRAY: CstType = CstType::new(&Type::FLOAT_ARRAY);
let static LONG_ARRAY: CstType = CstType::new(&Type::LONG_ARRAY);
let static INT_ARRAY: CstType = CstType::new(&Type::INT_ARRAY);
let static SHORT_ARRAY: CstType = CstType::new(&Type::SHORT_ARRAY);
let static METHOD_HANDLE: CstType = CstType::new(&Type::METHOD_HANDLE);
let static VAR_HANDLE: CstType = CstType::new(&Type::VAR_HANDLE);
struct CstType{
    pub type: Type,
    pub descriptor: CstString,
}
impl CstType{
    pub fn initInterns()    {
        CstType::internInitial(&CstType::OBJECT);
        CstType::internInitial(&CstType::BOOLEAN);
        CstType::internInitial(&CstType::BYTE);
        CstType::internInitial(&CstType::CHARACTER);
        CstType::internInitial(&CstType::DOUBLE);
        CstType::internInitial(&CstType::FLOAT);
        CstType::internInitial(&CstType::LONG);
        CstType::internInitial(&CstType::INTEGER);
        CstType::internInitial(&CstType::SHORT);
        CstType::internInitial(&CstType::VOID);
        CstType::internInitial(&CstType::BOOLEAN_ARRAY);
        CstType::internInitial(&CstType::BYTE_ARRAY);
        CstType::internInitial(&CstType::CHAR_ARRAY);
        CstType::internInitial(&CstType::DOUBLE_ARRAY);
        CstType::internInitial(&CstType::FLOAT_ARRAY);
        CstType::internInitial(&CstType::LONG_ARRAY);
        CstType::internInitial(&CstType::INT_ARRAY);
        CstType::internInitial(&CstType::SHORT_ARRAY);
        CstType::internInitial(&CstType::METHOD_HANDLE);
    }
    pub fn internInitial(cst: &CstType)    {
        if CstType::interns.putIfAbsent(cst.getClassType(), &cst)!=None        {
            throw IllegalStateException::new("Attempted re-init of "+cst);
        }        
    }
    pub fn forBoxedPrimitiveType(primitiveType: &Type) -> CstType    {
        match primitiveType.getBasicType(){Type::BT_BOOLEAN =>             return CstType::BOOLEAN;Type::BT_BYTE =>             return CstType::BYTE;Type::BT_CHAR =>             return CstType::CHARACTER;Type::BT_DOUBLE =>             return CstType::DOUBLE;Type::BT_FLOAT =>             return CstType::FLOAT;Type::BT_INT =>             return CstType::INTEGER;Type::BT_LONG =>             return CstType::LONG;Type::BT_SHORT =>             return CstType::SHORT;Type::BT_VOID =>             return CstType::VOID;        }
        throw IllegalArgumentException::new("not primitive: "+primitiveType);
    }
    pub fn intern(type: &Type) -> CstType    {
        let cst: CstType = CstType::new(&type);
        let result: CstType = CstType::interns.putIfAbsent(&type, &cst);
        return if result!=None { result } else { cst };
            }
            pub fn new(&self, type: &Type)            {
                if type==None                {
                    throw NullPointerException::new("type == null");
                }                
                if type==Type::KNOWN_NULL                {
                    throw UnsupportedOperationException::new("KNOWN_NULL is not representable");
                }                
                self->type=type;
                self->descriptor=None;
            }
            pub fn equals(&self, other: &Object) -> boolean            {
                if !(//other instanceof CstType)                {
                    return false;
                }                
                return self.type_renamed==((CstType*)other)->type;
            }
            pub fn hashCode(&self) -> i32            {
                return self.type_renamed.hashCode();
            }
            pub fn compareTo0(&self, other: &Constant) -> i32            {
                let thisDescriptor: String = self.type_renamed.getDescriptor();
                let otherDescriptor: String = ((CstType*)other)->type.getDescriptor();
                return thisDescriptor.compareTo(&otherDescriptor);
            }
            pub fn toString(&self) -> String            {
                return "type{"+toHuman()+'}';
            }
            pub fn getType(&self) -> Type            {
                return Type::CLASS;
            }
            pub fn typeName(&self) -> String            {
                return "type";
            }
            pub fn isCategory2(&self) -> boolean            {
                return false;
            }
            pub fn toHuman(&self) -> String            {
                return self.type_renamed.toHuman();
            }
            pub fn getClassType(&self) -> Type            {
                return self.type_renamed;
            }
            pub fn getDescriptor(&self) -> CstString            {
                if self.descriptor==None                {
                    self.descriptor=CstString::new(self.type_renamed.getDescriptor());
                }                
                return self.descriptor;
            }
            pub fn getPackageName(&self) -> String            {
                let descriptor: String = getDescriptor().getString();
                let lastSlash: i32 = descriptor.lastIndexOf_int('/');
                let lastLeftSquare: i32 = descriptor.lastIndexOf_int('[');
                if lastSlash==-1                {
                    return "default";
                }                else                 {
                    return descriptor.substring_int_int(lastLeftSquare+2, lastSlash).replace_char_char('/', '.');
                }
            }
            pub fn clearInternTable()            {
                CstType::interns.clear();
                CstType::initInterns();
            }
}
