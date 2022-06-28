use crate::helper;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::util::MutabilityException;
use crate::com::android::dx::cf::code::BytecodeArray;
use crate::com::android::dx::cf::code::ByteCatchList;

let static ATTRIBUTE_NAME: String = "Code";
struct AttCode{
    pub maxStack: i32,
    pub maxLocals: i32,
    pub code: BytecodeArray,
    pub catches: ByteCatchList,
    pub attributes: AttributeList,
}
impl AttCode{
    pub fn new(&self, maxStack: i32, maxLocals: i32, code: &BytecodeArray, catches: &ByteCatchList, attributes: &AttributeList)    {
        super(ATTRIBUTE_NAME);

        if maxStack<0        {
            throw IllegalArgumentException::new("maxStack < 0");
        }        
        if maxLocals<0        {
            throw IllegalArgumentException::new("maxLocals < 0");
        }        
        if code==None        {
            throw NullPointerException::new("code == null");
        }        
        try        {
            if catches.isMutable()            {
                throw MutabilityException::new("catches.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("catches == null");
        }
        try        {
            if attributes.isMutable()            {
                throw MutabilityException::new("attributes.isMutable()");
            }            
        }        catch(        let ex: NullPointerException)        {
            throw NullPointerException::new("attributes == null");
        }
        self->maxStack=maxStack;
        self->maxLocals=maxLocals;
        self->code=code;
        self->catches=catches;
        self->attributes=attributes;
    }
    pub fn byteLength(&self) -> i32    {
        return 10+self.code.byteLength()+self.catches.byteLength()+self.attributes.byteLength();
    }
    pub fn getMaxStack(&self) -> i32    {
        return self.maxStack;
    }
    pub fn getMaxLocals(&self) -> i32    {
        return self.maxLocals;
    }
    pub fn getCode(&self) -> BytecodeArray    {
        return self.code;
    }
    pub fn getCatches(&self) -> ByteCatchList    {
        return self.catches;
    }
    pub fn getAttributes(&self) -> AttributeList    {
        return self.attributes;
    }
}
