use crate::helper;
use crate::com::android::dx::rop::code::RegisterSpecList::Expander;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::type::Type;

let static EMPTY: RegisterSpecList = RegisterSpecList::new(0);
struct RegisterSpecList{
}
impl RegisterSpecList{
    pub fn make(spec: &RegisterSpec) -> RegisterSpecList    {
        let result: RegisterSpecList = RegisterSpecList::new(1);
        result.set_int_RegisterSpec(0, &spec);
        return result;
    }
    pub fn make(spec0: &RegisterSpec, spec1: &RegisterSpec) -> RegisterSpecList    {
        let result: RegisterSpecList = RegisterSpecList::new(2);
        result.set_int_RegisterSpec(0, &spec0);
        result.set_int_RegisterSpec(1, &spec1);
        return result;
    }
    pub fn make(spec0: &RegisterSpec, spec1: &RegisterSpec, spec2: &RegisterSpec) -> RegisterSpecList    {
        let result: RegisterSpecList = RegisterSpecList::new(3);
        result.set_int_RegisterSpec(0, &spec0);
        result.set_int_RegisterSpec(1, &spec1);
        result.set_int_RegisterSpec(2, &spec2);
        return result;
    }
    pub fn make(spec0: &RegisterSpec, spec1: &RegisterSpec, spec2: &RegisterSpec, spec3: &RegisterSpec) -> RegisterSpecList    {
        let result: RegisterSpecList = RegisterSpecList::new(4);
        result.set_int_RegisterSpec(0, &spec0);
        result.set_int_RegisterSpec(1, &spec1);
        result.set_int_RegisterSpec(2, &spec2);
        result.set_int_RegisterSpec(3, &spec3);
        return result;
    }
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn getType(&self, n: i32) -> Type    {
        return get(n).getType().getType();
    }
    pub fn getWordCount(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            result+=getType(i).getCategory();
        }
        return result;
    }
    pub fn withAddedType(&self, type: &Type) -> TypeList    {
        throw UnsupportedOperationException::new("unsupported");
    }
    pub fn get(&self, n: i32) -> RegisterSpec    {
        return (RegisterSpec*)get0(n);
    }
    pub fn specForRegister(&self, reg: i32) -> RegisterSpec    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let rs: RegisterSpec;
            rs=get(i);
            if rs.getReg()==reg            {
                return rs;
            }            
        }
        return None;
    }
    pub fn indexOfRegister(&self, reg: i32) -> i32    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let rs: RegisterSpec;
            rs=get(i);
            if rs.getReg()==reg            {
                return i;
            }            
        }
        return -1;
    }
    pub fn set(&self, n: i32, spec: &RegisterSpec)    {
        set0(n, &spec);
    }
    pub fn getRegistersSize(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let spec: RegisterSpec = (RegisterSpec*)get0(i);
            if spec!=None            {
                let min: i32 = spec.getNextReg();
                if min>result                {
                    result=min;
                }                
            }            
        }
        return result;
    }
    pub fn withFirst(&self, spec: &RegisterSpec) -> RegisterSpecList    {
        let sz: i32 = size();
        let result: RegisterSpecList = RegisterSpecList::new(sz+1);
        for(        let i: i32 = 0;;i<szi += 1)        {
            result.set0(i+1, get0(i));
        }
        result.set0(0, &spec);
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
    pub fn withoutFirst(&self) -> RegisterSpecList    {
        let newSize: i32 = size()-1;
        if newSize==0        {
            return RegisterSpecList::EMPTY;
        }        
        let result: RegisterSpecList = RegisterSpecList::new(newSize);
        for(        let i: i32 = 0;;i<newSizei += 1)        {
            result.set0(i, get0(i+1));
        }
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
    pub fn withoutLast(&self) -> RegisterSpecList    {
        let newSize: i32 = size()-1;
        if newSize==0        {
            return RegisterSpecList::EMPTY;
        }        
        let result: RegisterSpecList = RegisterSpecList::new(newSize);
        for(        let i: i32 = 0;;i<newSizei += 1)        {
            result.set0(i, get0(i));
        }
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
    pub fn subset(&self, exclusionSet: &BitSet) -> RegisterSpecList    {
        let newSize: i32 = size()-exclusionSet.cardinality();
        if newSize==0        {
            return RegisterSpecList::EMPTY;
        }        
        let result: RegisterSpecList = RegisterSpecList::new(newSize);
        let newIndex: i32 = 0;
        for(        let oldIndex: i32 = 0;;oldIndex<size()oldIndex += 1)        {
            if !exclusionSet.get_int(oldIndex)            {
                result.set0(newIndex, get0(oldIndex));
                newIndex += 1;
            }            
        }
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
    pub fn withOffset(&self, delta: i32) -> RegisterSpecList    {
        let sz: i32 = size();
        if sz==0        {
            return self;
        }        
        let result: RegisterSpecList = RegisterSpecList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: RegisterSpec = (RegisterSpec*)get0(i);
            if one!=None            {
                result.set0(i, one.withOffset(delta));
            }            
        }
        if isImmutable()        {
            result.setImmutable();
        }        
        return result;
    }
    pub fn withExpandedRegisters(&self, base: i32, duplicateFirst: boolean, compatRegs: &BitSet) -> RegisterSpecList    {
        let sz: i32 = size();
        if sz==0        {
            return self;
        }        
        let expander: Expander = Expander::new(self, &compatRegs, base, duplicateFirst);
        for(        let regIdx: i32 = 0;;regIdx<szregIdx += 1)        {
            expander.expandRegister_int(regIdx);
        }
        return expander.getResult();
    }
}
