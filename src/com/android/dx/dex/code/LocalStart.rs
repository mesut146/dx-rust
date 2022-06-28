use crate::helper;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::dex::code::LocalStart;

struct LocalStart{
    pub local: RegisterSpec,
}
impl LocalStart{
    pub fn localString(spec: &RegisterSpec) -> String    {
        return spec.regString()+' '+spec.getLocalItem().toString()+": "+spec.getTypeBearer().toHuman();
    }
    pub fn new(&self, position: &SourcePosition, local: &RegisterSpec)    {
        super(position);

        if local==None        {
            throw NullPointerException::new("local == null");
        }        
        self->local=local;
    }
    pub fn withRegisterOffset(&self, delta: i32) -> DalvInsn    {
        return LocalStart::new(getPosition(), self.local.withOffset(delta));
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return LocalStart::new(getPosition(), &self.local);
    }
    pub fn getLocal(&self) -> RegisterSpec    {
        return self.local;
    }
    pub fn argString(&self) -> String    {
        return self.local.toString();
    }
    pub fn listingString0(&self, noteIndices: boolean) -> String    {
        return "local-start "+LocalStart::localString(&self.local);
    }
    pub fn withMapper(&self, mapper: &RegisterMapper) -> DalvInsn    {
        return LocalStart::new(getPosition(), mapper.map_RegisterSpec(&self.local));
    }
}
