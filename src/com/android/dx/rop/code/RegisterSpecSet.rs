use crate::helper;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecSet;

let static EMPTY: RegisterSpecSet = RegisterSpecSet::new(0);
struct RegisterSpecSet{
    pub specs: Vec<RegisterSpec>,
    pub size: i32,
}
impl RegisterSpecSet{
    pub fn new(&self, maxSize: i32)    {
        super(maxSize != 0);

        self->specs=new RegisterSpec[maxSize];
        self->size=0;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof RegisterSpecSet)        {
            return false;
        }        
        let otherSet: RegisterSpecSet = (RegisterSpecSet*)other;
        let otherSpecs: Vec<RegisterSpec> = ;
        let len: i32 = self.specs.len();
        if (len!=otherSpecs.len())||(size()!=otherSet.size())        {
            return false;
        }        
        for(        let i: i32 = 0;;i<leni += 1)        {
            let s1: RegisterSpec = self.specs[i];
            let s2: RegisterSpec = otherSpecs[i];
            if s1==s2            {
                continue;
            }            
            if (s1==None)||!s1.equals_Object(&s2)            {
                return false;
            }            
        }
        return true;
    }
    pub fn hashCode(&self) -> i32    {
        let len: i32 = self.specs.len();
        let hash: i32 = 0;
        for(        let i: i32 = 0;;i<leni += 1)        {
            let spec: RegisterSpec = self.specs[i];
            let oneHash: i32 = if (spec==None) { 0 } else { spec.hashCode() };
                    hash=(hash*31)+oneHash;
                }
                return hash;
            }
            pub fn toString(&self) -> String            {
                let len: i32 = self.specs.len();
                let sb: StringBuilder = StringBuilder::new(len*25);
                sb.append_char('{');
                let any: boolean = false;
                for(                let i: i32 = 0;;i<leni += 1)                {
                    let spec: RegisterSpec = self.specs[i];
                    if spec!=None                    {
                        if any                        {
                            sb.append_String(", ");
                        }                        else                         {
                            any=true;
                        }
                        sb.append_Object(&spec);
                    }                    
                }
                sb.append_char('}');
                return sb.toString();
            }
            pub fn getMaxSize(&self) -> i32            {
                return self.specs.len();
            }
            pub fn size(&self) -> i32            {
                let result: i32 = self.size;
                if result<0                {
                    let len: i32 = self.specs.len();
                    result=0;
                    for(                    let i: i32 = 0;;i<leni += 1)                    {
                        if self.specs[i]!=None                        {
                            result += 1;
                        }                        
                    }
                    self.size=result;
                }                
                return result;
            }
            pub fn get(&self, reg: i32) -> RegisterSpec            {
                try                {
                    return self.specs[reg];
                }                catch(                let ex: ArrayIndexOutOfBoundsException)                {
                    throw IllegalArgumentException::new("bogus reg");
                }
            }
            pub fn get(&self, spec: &RegisterSpec) -> RegisterSpec            {
                return get_int(spec.getReg());
            }
            pub fn findMatchingLocal(&self, spec: &RegisterSpec) -> RegisterSpec            {
                let length: i32 = self.specs.len();
                for(                let reg: i32 = 0;;reg<lengthreg += 1)                {
                    let s: RegisterSpec = self.specs[reg];
                    if s==None                    {
                        continue;
                    }                    
                    if spec.matchesVariable(&s)                    {
                        return s;
                    }                    
                }
                return None;
            }
            pub fn localItemToSpec(&self, local: &LocalItem) -> RegisterSpec            {
                let length: i32 = self.specs.len();
                for(                let reg: i32 = 0;;reg<lengthreg += 1)                {
                    let spec: RegisterSpec = self.specs[reg];
                    if (spec!=None)&&local.equals(spec.getLocalItem())                    {
                        return spec;
                    }                    
                }
                return None;
            }
            pub fn remove(&self, toRemove: &RegisterSpec)            {
                try                {
                    self.specs[toRemove.getReg()]=None;
                    self.size=-1;
                }                catch(                let ex: ArrayIndexOutOfBoundsException)                {
                    throw IllegalArgumentException::new("bogus reg");
                }
            }
            pub fn put(&self, spec: &RegisterSpec)            {
                throwIfImmutable();
                if spec==None                {
                    throw NullPointerException::new("spec == null");
                }                
                self.size=-1;
                try                {
                    let reg: i32 = spec.getReg();
                    self.specs[reg]=spec;
                    if reg>0                    {
                        let prevReg: i32 = reg-1;
                        let prevSpec: RegisterSpec = self.specs[prevReg];
                        if (prevSpec!=None)&&(prevSpec.getCategory()==2)                        {
                            self.specs[prevReg]=None;
                        }                        
                    }                    
                    if spec.getCategory()==2                    {
                        self.specs[reg+1]=None;
                    }                    
                }                catch(                let ex: ArrayIndexOutOfBoundsException)                {
                    throw IllegalArgumentException::new("spec.getReg() out of range");
                }
            }
            pub fn putAll(&self, set: &RegisterSpecSet)            {
                let max: i32 = set.getMaxSize();
                for(                let i: i32 = 0;;i<maxi += 1)                {
                    let spec: RegisterSpec = set.get_int(i);
                    if spec!=None                    {
                        put(&spec);
                    }                    
                }
            }
            pub fn intersect(&self, other: &RegisterSpecSet, localPrimary: boolean)            {
                throwIfImmutable();
                let otherSpecs: Vec<RegisterSpec> = ;
                let thisLen: i32 = self.specs.len();
                let len: i32 = Math::min_int_int(thisLen, otherSpecs.len());
                self.size=-1;
                for(                let i: i32 = 0;;i<leni += 1)                {
                    let spec: RegisterSpec = self.specs[i];
                    if spec==None                    {
                        continue;
                    }                    
                    let intersection: RegisterSpec = spec.intersect(otherSpecs[i], localPrimary);
                    if intersection!=spec                    {
                        self.specs[i]=intersection;
                    }                    
                }
                for(                let i: i32 = len;;i<thisLeni += 1)                {
                    self.specs[i]=None;
                }
            }
            pub fn withOffset(&self, delta: i32) -> RegisterSpecSet            {
                let len: i32 = self.specs.len();
                let result: RegisterSpecSet = RegisterSpecSet::new(len+delta);
                for(                let i: i32 = 0;;i<leni += 1)                {
                    let spec: RegisterSpec = self.specs[i];
                    if spec!=None                    {
                        result.put(spec.withOffset(delta));
                    }                    
                }
                =self.size;
                if isImmutable()                {
                    result.setImmutable();
                }                
                return result;
            }
            pub fn mutableCopy(&self) -> RegisterSpecSet            {
                let len: i32 = self.specs.len();
                let copy: RegisterSpecSet = RegisterSpecSet::new(len);
                for(                let i: i32 = 0;;i<leni += 1)                {
                    let spec: RegisterSpec = self.specs[i];
                    if spec!=None                    {
                        copy.put(&spec);
                    }                    
                }
                =self.size;
                return copy;
            }
}
