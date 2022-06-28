use crate::helper;
use crate::com::android::dx::util::MutabilityException;

struct MutabilityControl{
    pub mutable: boolean,
}
impl MutabilityControl{
    pub fn new(&self)    {
        self.mutable=true;
    }
    pub fn new(&self, mutable: boolean)    {
        self->mutable=mutable_renamed;
    }
    pub fn setImmutable(&self)    {
        self.mutable=false;
    }
    pub fn isImmutable(&self) -> boolean    {
        return !self.mutable;
    }
    pub fn isMutable(&self) -> boolean    {
        return self.mutable;
    }
    pub fn throwIfImmutable(&self)    {
        if !self.mutable        {
            throw MutabilityException::new("immutable instance");
        }        
    }
    pub fn throwIfMutable(&self)    {
        if self.mutable        {
            throw MutabilityException::new("mutable instance");
        }        
    }
}
