use crate::helper;
use crate::com::android::dx::cf::code::LocalsArraySet;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::cf::code::OneLocalsArray;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::cf::code::LocalsArray;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::util::Hex;

struct LocalsArraySet{
    pub primary: OneLocalsArray,
    pub secondaries: ArrayList<LocalsArray>,
}
impl LocalsArraySet{
    pub fn new(&self, maxLocals: i32)    {
        super(maxLocals != 0);

        self.primary=OneLocalsArray::new(maxLocals);
        self.secondaries=ArrayList::new();
    }
    pub fn new(&self, primary: &OneLocalsArray, secondaries: &ArrayList<LocalsArray>)    {
        super(primary.getMaxLocals() > 0);

        self->primary=primary;
        self->secondaries=secondaries;
    }
    pub fn new(&self, toCopy: &LocalsArraySet)    {
        super(toCopy.getMaxLocals() > 0);

        self.primary=.copy();
        self.secondaries=ArrayList::new(.size());
        let sz: i32 = .size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let la: LocalsArray = .get(i);
            if la==None            {
                self.secondaries.add_LocalsArray(None);
            }            else             {
                self.secondaries.add_LocalsArray(la.copy());
            }
        }
    }
    pub fn setImmutable(&self)    {
        self.primary.setImmutable();
        for la in self.secondaries        {
            if la!=None            {
                la.setImmutable();
            }            
        }
        super.setImmutable();
    }
    pub fn copy(&self) -> LocalsArray    {
        return LocalsArraySet::new(self);
    }
    pub fn annotate(&self, ex: &ExceptionWithContext)    {
        ex.addContext("(locals array set; primary)");
        self.primary.annotate(&ex);
        let sz: i32 = self.secondaries.size();
        for(        let label: i32 = 0;;label<szlabel += 1)        {
            let la: LocalsArray = self.secondaries.get(label);
            if la!=None            {
                ex.addContext("(locals array set: primary for caller "+Hex::u2(label)+')');
                la.getPrimary().annotate(&ex);
            }            
        }
    }
    pub fn toHuman(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        sb.append_String("(locals array set; primary)\n");
        sb.append_String(getPrimary().toHuman());
        sb.append_char('\n');
        let sz: i32 = self.secondaries.size();
        for(        let label: i32 = 0;;label<szlabel += 1)        {
            let la: LocalsArray = self.secondaries.get(label);
            if la!=None            {
                sb.append_String("(locals array set: primary for caller "+Hex::u2(label)+")\n");
                sb.append_String(la.getPrimary().toHuman());
                sb.append_char('\n');
            }            
        }
        return sb.toString();
    }
    pub fn makeInitialized(&self, type: &Type)    {
        let len: i32 = self.primary.getMaxLocals();
        if len==0        {
            return;
        }        
        throwIfImmutable();
        self.primary.makeInitialized(&type);
        for la in self.secondaries        {
            if la!=None            {
                la.makeInitialized(&type);
            }            
        }
    }
    pub fn getMaxLocals(&self) -> i32    {
        return self.primary.getMaxLocals();
    }
    pub fn set(&self, idx: i32, type: &TypeBearer)    {
        throwIfImmutable();
        self.primary.set_int_TypeBearer(idx, &type);
        for la in self.secondaries        {
            if la!=None            {
                la.set_int_TypeBearer(idx, &type);
            }            
        }
    }
    pub fn set(&self, spec: &RegisterSpec)    {
        set_int_TypeBearer(spec.getReg(), &spec);
    }
    pub fn invalidate(&self, idx: i32)    {
        throwIfImmutable();
        self.primary.invalidate(idx);
        for la in self.secondaries        {
            if la!=None            {
                la.invalidate(idx);
            }            
        }
    }
    pub fn getOrNull(&self, idx: i32) -> TypeBearer    {
        return self.primary.getOrNull(idx);
    }
    pub fn get(&self, idx: i32) -> TypeBearer    {
        return self.primary.get(idx);
    }
    pub fn getCategory1(&self, idx: i32) -> TypeBearer    {
        return self.primary.getCategory1(idx);
    }
    pub fn getCategory2(&self, idx: i32) -> TypeBearer    {
        return self.primary.getCategory2(idx);
    }
    pub fn mergeWithSet(&self, other: &LocalsArraySet) -> LocalsArraySet    {
        let newPrimary: OneLocalsArray;
        let newSecondaries: ArrayList<LocalsArray>;
        let secondariesChanged: boolean = false;
        newPrimary=self.primary.merge_OneLocalsArray(other.getPrimary());
        let sz1: i32 = self.secondaries.size();
        let sz2: i32 = .size();
        let sz: i32 = Math::max_int_int(sz1, sz2);
        newSecondaries=ArrayList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let la1: LocalsArray = (if i<sz1 { self.secondaries.get(i) } else { None });
                    let la2: LocalsArray = (if i<sz2 { .get(i) } else { None });
                            let resultla: LocalsArray = None;
                            if la1==la2                            {
                                resultla=la1;
                            }                            else                             if la1==None                            {
                                resultla=la2;
                            }                            else                             if la2==None                            {
                                resultla=la1;
                            }                            else                             {
                                try                                {
                                    resultla=la1.merge(&la2);
                                }                                catch(                                let ex: SimException)                                {
                                    ex.addContext("Merging locals set for caller block "+Hex::u2(i));
                                }
                            }
                            secondariesChanged=secondariesChanged||(la1!=resultla);
                            newSecondaries.add_LocalsArray(&resultla);
                        }
                        if (self.primary==newPrimary)&&!secondariesChanged                        {
                            return self;
                        }                        
                        return LocalsArraySet::new(&newPrimary, &newSecondaries);
                    }
                    pub fn mergeWithOne(&self, other: &OneLocalsArray) -> LocalsArraySet                    {
                        let newPrimary: OneLocalsArray;
                        let newSecondaries: ArrayList<LocalsArray>;
                        let secondariesChanged: boolean = false;
                        newPrimary=self.primary.merge_OneLocalsArray(other.getPrimary());
                        newSecondaries=ArrayList::new(self.secondaries.size());
                        let sz: i32 = self.secondaries.size();
                        for(                        let i: i32 = 0;;i<szi += 1)                        {
                            let la: LocalsArray = self.secondaries.get(i);
                            let resultla: LocalsArray = None;
                            if la!=None                            {
                                try                                {
                                    resultla=la.merge(&other);
                                }                                catch(                                let ex: SimException)                                {
                                    ex.addContext("Merging one locals against caller block "+Hex::u2(i));
                                }
                            }                            
                            secondariesChanged=secondariesChanged||(la!=resultla);
                            newSecondaries.add_LocalsArray(&resultla);
                        }
                        if (self.primary==newPrimary)&&!secondariesChanged                        {
                            return self;
                        }                        
                        return LocalsArraySet::new(&newPrimary, &newSecondaries);
                    }
                    pub fn merge(&self, other: &LocalsArray) -> LocalsArraySet                    {
                        let result: LocalsArraySet;
                        try                        {
                            if //other instanceof LocalsArraySet                            {
                                result=mergeWithSet((LocalsArraySet*)other);
                            }                            else                             {
                                result=mergeWithOne((OneLocalsArray*)other);
                            }
                        }                        catch(                        let ex: SimException)                        {
                            ex.addContext("underlay locals:");
                            annotate(&ex);
                            ex.addContext("overlay locals:");
                            other.annotate(&ex);
                            throw ex;
                        }
                        result.setImmutable();
                        return result;
                    }
                    pub fn getSecondaryForLabel(&self, label: i32) -> LocalsArray                    {
                        if label>=self.secondaries.size()                        {
                            return None;
                        }                        
                        return self.secondaries.get(label);
                    }
                    pub fn mergeWithSubroutineCaller(&self, other: &LocalsArray, predLabel: i32) -> LocalsArraySet                    {
                        let mine: LocalsArray = getSecondaryForLabel(predLabel);
                        let newSecondary: LocalsArray;
                        let newPrimary: OneLocalsArray;
                        newPrimary=self.primary.merge_OneLocalsArray(other.getPrimary());
                        if mine==other                        {
                            newSecondary=mine;
                        }                        else                         if mine==None                        {
                            newSecondary=other;
                        }                        else                         {
                            newSecondary=mine.merge(&other);
                        }
                        if (newSecondary==mine)&&(newPrimary==self.primary)                        {
                            return self;
                        }                        else                         {
                            newPrimary=None;
                            let szSecondaries: i32 = self.secondaries.size();
                            let sz: i32 = Math::max_int_int(predLabel+1, szSecondaries);
                            let newSecondaries: ArrayList<LocalsArray> = ArrayList::new(sz);
                            for(                            let i: i32 = 0;;i<szi += 1)                            {
                                let la: LocalsArray = None;
                                if i==predLabel                                {
                                    la=newSecondary;
                                }                                else                                 if i<szSecondaries                                {
                                    la=self.secondaries.get(i);
                                }                                
                                if la!=None                                {
                                    if newPrimary==None                                    {
                                        newPrimary=la.getPrimary();
                                    }                                    else                                     {
                                        newPrimary=newPrimary.merge_OneLocalsArray(la.getPrimary());
                                    }
                                }                                
                                newSecondaries.add_LocalsArray(&la);
                            }
                            let result: LocalsArraySet = LocalsArraySet::new(&newPrimary, &newSecondaries);
                            result.setImmutable();
                            return result;
                        }
                    }
                    pub fn subArrayForLabel(&self, subLabel: i32) -> LocalsArray                    {
                        let result: LocalsArray = getSecondaryForLabel(subLabel);
                        return result;
                    }
                    pub fn getPrimary(&self) -> OneLocalsArray                    {
                        return self.primary;
                    }
}
