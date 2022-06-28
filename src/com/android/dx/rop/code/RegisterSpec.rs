use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::code::RegisterSpec::ForComparison;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::type::Type;

let static PREFIX: String = "v";
let static theInterns: ConcurrentHashMap<Object,RegisterSpec> = ConcurrentHashMap<Object,RegisterSpec>::new(10_000, 0.75f);
let static theInterningItem: ThreadLocal<ForComparison> = /*new ThreadLocal<ForComparison>(){
  @Override protected ForComparison initialValue(){
    return new ForComparison();
  }
}
*/;
struct RegisterSpec{
    pub reg: i32,
    pub type: TypeBearer,
    pub local: LocalItem,
}
impl RegisterSpec{
    pub fn intern(reg: i32, type: &TypeBearer, local: &LocalItem) -> RegisterSpec    {
        let interningItem: ForComparison = RegisterSpec::theInterningItem.get();
        interningItem.set(reg, &type, &local);
        let found: RegisterSpec = RegisterSpec::theInterns.get(&interningItem);
        if found==None        {
            found=interningItem.toRegisterSpec();
            let existing: RegisterSpec = RegisterSpec::theInterns.putIfAbsent(&found, &found);
            if existing!=None            {
                return existing;
            }            
        }        
        return found;
    }
    pub fn make(reg: i32, type: &TypeBearer) -> RegisterSpec    {
        return RegisterSpec::intern(reg, &type, None);
    }
    pub fn make(reg: i32, type: &TypeBearer, local: &LocalItem) -> RegisterSpec    {
        if local==None        {
            throw NullPointerException::new("local  == null");
        }        
        return RegisterSpec::intern(reg, &type, &local);
    }
    pub fn makeLocalOptional(reg: i32, type: &TypeBearer, local: &LocalItem) -> RegisterSpec    {
        return RegisterSpec::intern(reg, &type, &local);
    }
    pub fn regString(reg: i32) -> String    {
        return RegisterSpec::PREFIX+reg;
    }
    pub fn new(&self, reg: i32, type: &TypeBearer, local: &LocalItem)    {
        if reg<0        {
            throw IllegalArgumentException::new("reg < 0");
        }        
        if type==None        {
            throw NullPointerException::new("type == null");
        }        
        self->reg=reg;
        self->type=type;
        self->local=local;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if self==other        {
            return true;
        }        
        if !(//other instanceof RegisterSpec)        {
            if //other instanceof ForComparison            {
                let fc: ForComparison = (ForComparison*)other;
                return equals_int_TypeBearer_LocalItem(, &, &);
            }            
            return false;
        }        
        let spec: RegisterSpec = (RegisterSpec*)other;
        return equals_int_TypeBearer_LocalItem(, &, &);
    }
    pub fn equalsUsingSimpleType(&self, other: &RegisterSpec) -> boolean    {
        if !matchesVariable(&other)        {
            return false;
        }        
        return (self.reg==);
    }
    pub fn matchesVariable(&self, other: &RegisterSpec) -> boolean    {
        if other==None        {
            return false;
        }        
        return self.type_renamed.getType().equals(.getType())&&((self.local==)||((self.local!=None)&&self.local.equals(&)));
    }
    pub fn equals(&self, reg: i32, type: &TypeBearer, local: &LocalItem) -> boolean    {
        return (self->reg==reg)&&self->type.equals(&type)&&((self->local==local)||((self->local!=None)&&self->local.equals(&local)));
    }
    pub fn compareTo(&self, other: &RegisterSpec) -> i32    {
        if self->reg<        {
            return -1;
        }        else         if self->reg>        {
            return 1;
        }        else         if self==other        {
            return 0;
        }        
        let compare: i32 = self.type_renamed.getType().compareTo(.getType());
        if compare!=0        {
            return compare;
        }        
        if self->local==None        {
            return if (==None) { 0 } else { -1 };
                }                else                 if ==None                {
                    return 1;
                }                
                return self->local.compareTo(&);
            }
            pub fn hashCode(&self) -> i32            {
                return RegisterSpec::hashCodeOf(self.reg, &self.type_renamed, &self.local);
            }
            pub fn hashCodeOf(reg: i32, type: &TypeBearer, local: &LocalItem) -> i32            {
                let hash: i32 = if (local!=None) { local.hashCode() } else { 0 };
                        hash=(hash*31+type.hashCode())*31+reg;
                        return hash;
                    }
                    pub fn toString(&self) -> String                    {
                        return toString0(false);
                    }
                    pub fn toHuman(&self) -> String                    {
                        return toString0(true);
                    }
                    pub fn getType(&self) -> Type                    {
                        return self.type_renamed.getType();
                    }
                    pub fn getFrameType(&self) -> TypeBearer                    {
                        return self.type_renamed.getFrameType();
                    }
                    pub fn getBasicType(&self) -> i32                    {
                        return self.type_renamed.getBasicType();
                    }
                    pub fn getBasicFrameType(&self) -> i32                    {
                        return self.type_renamed.getBasicFrameType();
                    }
                    pub fn isConstant(&self) -> boolean                    {
                        return false;
                    }
                    pub fn getReg(&self) -> i32                    {
                        return self.reg;
                    }
                    pub fn getTypeBearer(&self) -> TypeBearer                    {
                        return self.type_renamed;
                    }
                    pub fn getLocalItem(&self) -> LocalItem                    {
                        return self.local;
                    }
                    pub fn getNextReg(&self) -> i32                    {
                        return self.reg+getCategory();
                    }
                    pub fn getCategory(&self) -> i32                    {
                        return self.type_renamed.getType().getCategory();
                    }
                    pub fn isCategory1(&self) -> boolean                    {
                        return self.type_renamed.getType().isCategory1();
                    }
                    pub fn isCategory2(&self) -> boolean                    {
                        return self.type_renamed.getType().isCategory2();
                    }
                    pub fn regString(&self) -> String                    {
                        return RegisterSpec::regString_int(self.reg);
                    }
                    pub fn intersect(&self, other: &RegisterSpec, localPrimary: boolean) -> RegisterSpec                    {
                        if self==other                        {
                            return self;
                        }                        
                        if (other==None)||(self.reg!=other.getReg())                        {
                            return None;
                        }                        
                        let resultLocal: LocalItem = if ((self.local==None)||!self.local.equals(other.getLocalItem())) { None } else { self.local };
                                let sameName: boolean = (resultLocal==self.local);
                                if localPrimary&&!sameName                                {
                                    return None;
                                }                                
                                let thisType: Type = getType();
                                let otherType: Type = other.getType();
                                if thisType!=otherType                                {
                                    return None;
                                }                                
                                let resultTypeBearer: TypeBearer = if self.type_renamed.equals(other.getTypeBearer()) { self.type_renamed } else { thisType };
                                        if (resultTypeBearer==self.type_renamed)&&sameName                                        {
                                            return self;
                                        }                                        
                                        return if (resultLocal==None) { RegisterSpec::make_int_TypeBearer(self.reg, &resultTypeBearer) } else { RegisterSpec::make_int_TypeBearer_LocalItem(self.reg, &resultTypeBearer, &resultLocal) };
                                            }
                                            pub fn withReg(&self, newReg: i32) -> RegisterSpec                                            {
                                                if self.reg==newReg                                                {
                                                    return self;
                                                }                                                
                                                return RegisterSpec::makeLocalOptional(newReg, &self.type_renamed, &self.local);
                                            }
                                            pub fn withType(&self, newType: &TypeBearer) -> RegisterSpec                                            {
                                                return RegisterSpec::makeLocalOptional(self.reg, &newType, &self.local);
                                            }
                                            pub fn withOffset(&self, delta: i32) -> RegisterSpec                                            {
                                                if delta==0                                                {
                                                    return self;
                                                }                                                
                                                return withReg(self.reg+delta);
                                            }
                                            pub fn withSimpleType(&self) -> RegisterSpec                                            {
                                                let orig: TypeBearer = self.type_renamed;
                                                let newType: Type;
                                                if //orig instanceof Type                                                {
                                                    newType=(Type*)orig;
                                                }                                                else                                                 {
                                                    newType=orig.getType();
                                                }
                                                if newType.isUninitialized()                                                {
                                                    newType=newType.getInitializedType();
                                                }                                                
                                                if newType==orig                                                {
                                                    return self;
                                                }                                                
                                                return RegisterSpec::makeLocalOptional(self.reg, &newType, &self.local);
                                            }
                                            pub fn withLocalItem(&self, local: &LocalItem) -> RegisterSpec                                            {
                                                if (self->local==local)||((self->local!=None)&&self->local.equals(&local))                                                {
                                                    return self;
                                                }                                                
                                                return RegisterSpec::makeLocalOptional(self.reg, &self.type_renamed, &local);
                                            }
                                            pub fn isEvenRegister(&self) -> boolean                                            {
                                                return ((getReg()&1)==0);
                                            }
                                            pub fn toString0(&self, human: boolean) -> String                                            {
                                                let sb: StringBuilder = StringBuilder::new(40);
                                                sb.append_String(regString());
                                                sb.append_String(":");
                                                if self.local!=None                                                {
                                                    sb.append_String(self.local.toString());
                                                }                                                
                                                let justType: Type = self.type_renamed.getType();
                                                sb.append_Object(&justType);
                                                if justType!=self.type_renamed                                                {
                                                    sb.append_String("=");
                                                    if human&&(//type instanceof CstString)                                                    {
                                                        sb.append_String(((CstString*)self.type_renamed).toQuoted());
                                                    }                                                    else                                                     if human&&(//type instanceof Constant)                                                    {
                                                        sb.append_String(self.type_renamed.toHuman());
                                                    }                                                    else                                                     {
                                                        sb.append_Object(&self.type_renamed);
                                                    }
                                                }                                                
                                                return sb.toString();
                                            }
                                            pub fn clearInternTable()                                            {
                                                RegisterSpec::theInterns.clear();
                                            }
}
