use crate::helper;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::util::Hex;

let static internTable: ConcurrentMap<String,Type> = ConcurrentHashMap<>::new(10_000, 0.75f);
let static BOOLEAN: Type = Type::new("Z", Type::BT_BOOLEAN);
let static BYTE: Type = Type::new("B", Type::BT_BYTE);
let static CHAR: Type = Type::new("C", Type::BT_CHAR);
let static DOUBLE: Type = Type::new("D", Type::BT_DOUBLE);
let static FLOAT: Type = Type::new("F", Type::BT_FLOAT);
let static INT: Type = Type::new("I", Type::BT_INT);
let static LONG: Type = Type::new("J", Type::BT_LONG);
let static SHORT: Type = Type::new("S", Type::BT_SHORT);
let static VOID: Type = Type::new("V", Type::BT_VOID);
let static KNOWN_NULL: Type = Type::new("<null>", Type::BT_OBJECT);
let static RETURN_ADDRESS: Type = Type::new("<addr>", Type::BT_ADDR);
let static ANNOTATION: Type = Type::new("Ljava/lang/annotation/Annotation;", Type::BT_OBJECT);
let static CLASS: Type = Type::new("Ljava/lang/Class;", Type::BT_OBJECT);
let static CLONEABLE: Type = Type::new("Ljava/lang/Cloneable;", Type::BT_OBJECT);
let static METHOD_HANDLE: Type = Type::new("Ljava/lang/invoke/MethodHandle;", Type::BT_OBJECT);
let static METHOD_TYPE: Type = Type::new("Ljava/lang/invoke/MethodType;", Type::BT_OBJECT);
let static VAR_HANDLE: Type = Type::new("Ljava/lang/invoke/VarHandle;", Type::BT_OBJECT);
let static OBJECT: Type = Type::new("Ljava/lang/Object;", Type::BT_OBJECT);
let static SERIALIZABLE: Type = Type::new("Ljava/io/Serializable;", Type::BT_OBJECT);
let static STRING: Type = Type::new("Ljava/lang/String;", Type::BT_OBJECT);
let static THROWABLE: Type = Type::new("Ljava/lang/Throwable;", Type::BT_OBJECT);
let static BOOLEAN_CLASS: Type = Type::new("Ljava/lang/Boolean;", Type::BT_OBJECT);
let static BYTE_CLASS: Type = Type::new("Ljava/lang/Byte;", Type::BT_OBJECT);
let static CHARACTER_CLASS: Type = Type::new("Ljava/lang/Character;", Type::BT_OBJECT);
let static DOUBLE_CLASS: Type = Type::new("Ljava/lang/Double;", Type::BT_OBJECT);
let static FLOAT_CLASS: Type = Type::new("Ljava/lang/Float;", Type::BT_OBJECT);
let static INTEGER_CLASS: Type = Type::new("Ljava/lang/Integer;", Type::BT_OBJECT);
let static LONG_CLASS: Type = Type::new("Ljava/lang/Long;", Type::BT_OBJECT);
let static SHORT_CLASS: Type = Type::new("Ljava/lang/Short;", Type::BT_OBJECT);
let static VOID_CLASS: Type = Type::new("Ljava/lang/Void;", Type::BT_OBJECT);
let static BOOLEAN_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static BYTE_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static CHAR_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static DOUBLE_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static FLOAT_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static INT_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static LONG_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static OBJECT_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
let static SHORT_ARRAY: Type = Type::new("["+, Type::BT_OBJECT);
struct Type{
    pub descriptor: String,
    pub basicType: i32,
    pub newAt: i32,
    pub className: String,
    pub arrayType: Type,
    pub componentType: Type,
    pub initializedType: Type,
}
impl Type{
    pub const BT_VOID: i32 = 0;
    pub const BT_BOOLEAN: i32 = 1;
    pub const BT_BYTE: i32 = 2;
    pub const BT_CHAR: i32 = 3;
    pub const BT_DOUBLE: i32 = 4;
    pub const BT_FLOAT: i32 = 5;
    pub const BT_INT: i32 = 6;
    pub const BT_LONG: i32 = 7;
    pub const BT_SHORT: i32 = 8;
    pub const BT_OBJECT: i32 = 9;
    pub const BT_ADDR: i32 = 10;
    pub const BT_COUNT: i32 = 11;
    pub fn initInterns()    {
        Type::putIntern(&Type::BOOLEAN);
        Type::putIntern(&Type::BYTE);
        Type::putIntern(&Type::CHAR);
        Type::putIntern(&Type::DOUBLE);
        Type::putIntern(&Type::FLOAT);
        Type::putIntern(&Type::INT);
        Type::putIntern(&Type::LONG);
        Type::putIntern(&Type::SHORT);
        Type::putIntern(&Type::ANNOTATION);
        Type::putIntern(&Type::CLASS);
        Type::putIntern(&Type::CLONEABLE);
        Type::putIntern(&Type::METHOD_HANDLE);
        Type::putIntern(&Type::VAR_HANDLE);
        Type::putIntern(&Type::OBJECT);
        Type::putIntern(&Type::SERIALIZABLE);
        Type::putIntern(&Type::STRING);
        Type::putIntern(&Type::THROWABLE);
        Type::putIntern(&Type::BOOLEAN_CLASS);
        Type::putIntern(&Type::BYTE_CLASS);
        Type::putIntern(&Type::CHARACTER_CLASS);
        Type::putIntern(&Type::DOUBLE_CLASS);
        Type::putIntern(&Type::FLOAT_CLASS);
        Type::putIntern(&Type::INTEGER_CLASS);
        Type::putIntern(&Type::LONG_CLASS);
        Type::putIntern(&Type::SHORT_CLASS);
        Type::putIntern(&Type::VOID_CLASS);
        Type::putIntern(&Type::BOOLEAN_ARRAY);
        Type::putIntern(&Type::BYTE_ARRAY);
        Type::putIntern(&Type::CHAR_ARRAY);
        Type::putIntern(&Type::DOUBLE_ARRAY);
        Type::putIntern(&Type::FLOAT_ARRAY);
        Type::putIntern(&Type::INT_ARRAY);
        Type::putIntern(&Type::LONG_ARRAY);
        Type::putIntern(&Type::OBJECT_ARRAY);
        Type::putIntern(&Type::SHORT_ARRAY);
    }
    pub fn intern(descriptor: &String) -> Type    {
        let result: Type = Type::internTable.get(&descriptor);
        if result!=None        {
            return result;
        }        
        let firstChar: char;
        try        {
            firstChar=descriptor.charAt(0);
        }        catch(        let ex: IndexOutOfBoundsException)        {
            throw IllegalArgumentException::new("descriptor is empty");
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("descriptor == null");
        }
        if firstChar=='['        {
            result=Type::intern(descriptor.substring_int(1));
            return result.getArrayType();
        }        
        let length: i32 = descriptor.length();
        if (firstChar!='L')||(descriptor.charAt(length-1)!=';')        {
            throw IllegalArgumentException::new("bad descriptor: "+descriptor);
        }        
        let limit: i32 = (length-1);
        for(        let i: i32 = 1;;i<limiti += 1)        {
            let c: char = descriptor.charAt(i);
            match c{'[' => ';' => '.' => '(' => ')' =>                 {
                    throw IllegalArgumentException::new("bad descriptor: "+descriptor);
                }'/' =>                 {
                    if (i==1)||(i==(length-1))||(descriptor.charAt(i-1)=='/')                    {
                        throw IllegalArgumentException::new("bad descriptor: "+descriptor);
                    }                    
                    break;
                }            }
        }
        result=Type::new(&descriptor, Type::BT_OBJECT);
        return Type::putIntern(&result);
    }
    pub fn internReturnType(descriptor: &String) -> Type    {
        try        {
            if descriptor.equals("V")            {
                return Type::VOID;
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("descriptor == null");
        }
        return Type::intern(&descriptor);
    }
    pub fn internClassName(name: &String) -> Type    {
        if name==None        {
            throw NullPointerException::new("name == null");
        }        
        if name.startsWith_String("[")        {
            return Type::intern(&name);
        }        
        return Type::intern('L'+name+';');
    }
    pub fn new(&self, descriptor: &String, basicType: i32, newAt: i32)    {
        if descriptor==None        {
            throw NullPointerException::new("descriptor == null");
        }        
        if (basicType<0)||(basicType>=Type::BT_COUNT)        {
            throw IllegalArgumentException::new("bad basicType");
        }        
        if newAt<-1        {
            throw IllegalArgumentException::new("newAt < -1");
        }        
        self->descriptor=descriptor;
        self->basicType=basicType;
        self->newAt=newAt;
        self->arrayType=None;
        self->componentType=None;
        self->initializedType=None;
    }
    pub fn new(&self, descriptor: &String, basicType: i32)    {
        this(descriptor,basicType,-1);

    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if self==other        {
            return true;
        }        
        if !(//other instanceof Type)        {
            return false;
        }        
        return self.descriptor.equals(((Type*)other)->descriptor);
    }
    pub fn hashCode(&self) -> i32    {
        return self.descriptor.hashCode();
    }
    pub fn compareTo(&self, other: &Type) -> i32    {
        return self.descriptor.compareTo(&);
    }
    pub fn toString(&self) -> String    {
        return self.descriptor;
    }
    pub fn toHuman(&self) -> String    {
        match self.basicType{Type::BT_VOID =>             return "void";Type::BT_BOOLEAN =>             return "boolean";Type::BT_BYTE =>             return "byte";Type::BT_CHAR =>             return "char";Type::BT_DOUBLE =>             return "double";Type::BT_FLOAT =>             return "float";Type::BT_INT =>             return "int";Type::BT_LONG =>             return "long";Type::BT_SHORT =>             return "short";Type::BT_OBJECT =>             break;        _ => {}        return self.descriptor;    }
    if isArray()    {
        return getComponentType().toHuman()+"[]";
    }    
    return getClassName().replace_CharSequence_CharSequence("/", ".");
}
pub fn getType(&self) -> Type{
    return self;
}
pub fn getFrameType(&self) -> Type{
    match self.basicType{Type::BT_BOOLEAN => Type::BT_BYTE => Type::BT_CHAR => Type::BT_INT => Type::BT_SHORT =>         {
            return Type::INT;
        }    }
    return self;
}
pub fn getBasicType(&self) -> i32{
    return self.basicType;
}
pub fn getBasicFrameType(&self) -> i32{
    match self.basicType{Type::BT_BOOLEAN => Type::BT_BYTE => Type::BT_CHAR => Type::BT_INT => Type::BT_SHORT =>         {
            return Type::BT_INT;
        }    }
    return self.basicType;
}
pub fn isConstant(&self) -> boolean{
    return false;
}
pub fn getDescriptor(&self) -> String{
    return self.descriptor;
}
pub fn getClassName(&self) -> String{
    if self.className==None    {
        if !isReference()        {
            throw IllegalArgumentException::new("not an object type: "+self.descriptor);
        }        
        if self.descriptor.charAt(0)=='['        {
            self.className=self.descriptor;
        }        else         {
            self.className=self.descriptor.substring_int_int(1, self.descriptor.length()-1);
        }
    }    
    return self.className;
}
pub fn getCategory(&self) -> i32{
    match self.basicType{Type::BT_LONG => Type::BT_DOUBLE =>         {
            return 2;
        }    }
    return 1;
}
pub fn isCategory1(&self) -> boolean{
    match self.basicType{Type::BT_LONG => Type::BT_DOUBLE =>         {
            return false;
        }    }
    return true;
}
pub fn isCategory2(&self) -> boolean{
    match self.basicType{Type::BT_LONG => Type::BT_DOUBLE =>         {
            return true;
        }    }
    return false;
}
pub fn isIntlike(&self) -> boolean{
    match self.basicType{Type::BT_BOOLEAN => Type::BT_BYTE => Type::BT_CHAR => Type::BT_INT => Type::BT_SHORT =>         {
            return true;
        }    }
    return false;
}
pub fn isPrimitive(&self) -> boolean{
    match self.basicType{Type::BT_BOOLEAN => Type::BT_BYTE => Type::BT_CHAR => Type::BT_DOUBLE => Type::BT_FLOAT => Type::BT_INT => Type::BT_LONG => Type::BT_SHORT => Type::BT_VOID =>         {
            return true;
        }    }
    return false;
}
pub fn isReference(&self) -> boolean{
    return (self.basicType==Type::BT_OBJECT);
}
pub fn isArray(&self) -> boolean{
    return (self.descriptor.charAt(0)=='[');
}
pub fn isArrayOrKnownNull(&self) -> boolean{
    return isArray()||equals(&Type::KNOWN_NULL);
}
pub fn isUninitialized(&self) -> boolean{
    return (self.newAt>=0);
}
pub fn getNewAt(&self) -> i32{
    return self.newAt;
}
pub fn getInitializedType(&self) -> Type{
    if self.initializedType==None    {
        throw IllegalArgumentException::new("initialized type: "+self.descriptor);
    }    
    return self.initializedType;
}
pub fn getArrayType(&self) -> Type{
    if self.arrayType==None    {
        self.arrayType=Type::putIntern(Type::new('['+self.descriptor, Type::BT_OBJECT));
    }    
    return self.arrayType;
}
pub fn getComponentType(&self) -> Type{
    if self.componentType==None    {
        if self.descriptor.charAt(0)!='['        {
            throw IllegalArgumentException::new("not an array type: "+self.descriptor);
        }        
        self.componentType=Type::intern(self.descriptor.substring_int(1));
    }    
    return self.componentType;
}
pub fn asUninitialized(&self, newAt: i32) -> Type{
    if newAt<0    {
        throw IllegalArgumentException::new("newAt < 0");
    }    
    if !isReference()    {
        throw IllegalArgumentException::new("not a reference type: "+self.descriptor);
    }    
    if isUninitialized()    {
        throw IllegalArgumentException::new("already uninitialized: "+self.descriptor);
    }    
    let newDesc: String = 'N'+Hex::u2(newAt)+self.descriptor;
    let result: Type = Type::new(&newDesc, Type::BT_OBJECT, newAt);
    =self;
    return Type::putIntern(&result);
}
pub fn putIntern(type: &Type) -> Type{
    let result: Type = Type::internTable.putIfAbsent(type.getDescriptor(), &type);
    return if result!=None { result } else { type };
        }
        pub fn clearInternTable()        {
            Type::internTable.clear();
            Type::initInterns();
        }
}
