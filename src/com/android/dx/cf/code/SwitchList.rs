use crate::helper;
use crate::com::android::dx::util::IntList;

struct SwitchList{
    pub values: IntList,
    pub targets: IntList,
    pub size: i32,
}
impl SwitchList{
    pub fn new(&self, size: i32)    {
        super(true);

        self->values=IntList::new(size);
        self->targets=IntList::new(size+1);
        self->size=size;
    }
    pub fn setImmutable(&self)    {
        self.values.setImmutable();
        self.targets.setImmutable();
        super.setImmutable();
    }
    pub fn size(&self) -> i32    {
        return self.size;
    }
    pub fn getValue(&self, n: i32) -> i32    {
        return self.values.get(n);
    }
    pub fn getTarget(&self, n: i32) -> i32    {
        return self.targets.get(n);
    }
    pub fn getDefaultTarget(&self) -> i32    {
        return self.targets.get(self.size);
    }
    pub fn getTargets(&self) -> IntList    {
        return self.targets;
    }
    pub fn getValues(&self) -> IntList    {
        return self.values;
    }
    pub fn setDefaultTarget(&self, target: i32)    {
        throwIfImmutable();
        if target<0        {
            throw IllegalArgumentException::new("target < 0");
        }        
        if self.targets.size()!=self.size        {
            throw RuntimeException::new("non-default elements not all set");
        }        
        self.targets.add(target);
    }
    pub fn add(&self, value: i32, target: i32)    {
        throwIfImmutable();
        if target<0        {
            throw IllegalArgumentException::new("target < 0");
        }        
        self.values.add(value);
        self.targets.add(target);
    }
    pub fn removeSuperfluousDefaults(&self)    {
        throwIfImmutable();
        let sz: i32 = self.size;
        if sz!=(self.targets.size()-1)        {
            throw IllegalArgumentException::new("incomplete instance");
        }        
        let defaultTarget: i32 = self.targets.get(sz);
        let at: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let target: i32 = self.targets.get(i);
            if target!=defaultTarget            {
                if i!=at                {
                    self.targets.set(at, target);
                    self.values.set(at, self.values.get(i));
                }                
                at += 1;
            }            
        }
        if at!=sz        {
            self.values.shrink(at);
            self.targets.set(at, defaultTarget);
            self.targets.shrink(at+1);
            self.size=at;
        }        
    }
}
