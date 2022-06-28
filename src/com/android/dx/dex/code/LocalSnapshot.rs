use crate::helper;
use crate::com::android::dx::ssa::RegisterMapper;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::dex::code::LocalSnapshot;
use crate::com::android::dx::dex::code::LocalStart;

struct LocalSnapshot{
    pub locals: RegisterSpecSet,
}
impl LocalSnapshot{
    pub fn new(&self, position: &SourcePosition, locals: &RegisterSpecSet)    {
        super(position);

        if locals==None        {
            throw NullPointerException::new("locals == null");
        }        
        self->locals=locals;
    }
    pub fn withRegisterOffset(&self, delta: i32) -> DalvInsn    {
        return LocalSnapshot::new(getPosition(), self.locals.withOffset(delta));
    }
    pub fn withRegisters(&self, registers: &RegisterSpecList) -> DalvInsn    {
        return LocalSnapshot::new(getPosition(), &self.locals);
    }
    pub fn getLocals(&self) -> RegisterSpecSet    {
        return self.locals;
    }
    pub fn argString(&self) -> String    {
        return self.locals.toString();
    }
    pub fn listingString0(&self, noteIndices: boolean) -> String    {
        let sz: i32 = self.locals.size();
        let max: i32 = self.locals.getMaxSize();
        let sb: StringBuilder = StringBuilder::new(100+sz*40);
        sb.append_String("local-snapshot");
        for(        let i: i32 = 0;;i<maxi += 1)        {
            let spec: RegisterSpec = self.locals.get_int(i);
            if spec!=None            {
                sb.append_String("\n  ");
                sb.append_String(LocalStart::localString(&spec));
            }            
        }
        return sb.toString();
    }
    pub fn withMapper(&self, mapper: &RegisterMapper) -> DalvInsn    {
        return LocalSnapshot::new(getPosition(), mapper.map_RegisterSpecSet(&self.locals));
    }
}
