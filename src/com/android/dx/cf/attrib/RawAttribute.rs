use crate::helper;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::util::ByteArray;

struct RawAttribute{
    pub data: ByteArray,
    pub pool: ConstantPool,
}
impl RawAttribute{
    pub fn new(&self, name: &String, data: &ByteArray, pool: &ConstantPool)    {
        super(name);

        if data==None        {
            throw NullPointerException::new("data == null");
        }        
        self->data=data;
        self->pool=pool;
    }
    pub fn new(&self, name: &String, data: &ByteArray, offset: i32, length: i32, pool: &ConstantPool)    {
        this(name,data.slice(offset,offset + length),pool);

    }
    pub fn getData(&self) -> ByteArray    {
        return self.data;
    }
    pub fn byteLength(&self) -> i32    {
        return self.data.size()+6;
    }
    pub fn getPool(&self) -> ConstantPool    {
        return self.pool;
    }
}
