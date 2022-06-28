use crate::helper;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::cf::attrib::RawAttribute;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct AttributeFactory{
}
impl AttributeFactory{
    pub const CTX_CLASS: i32 = 0;
    pub const CTX_FIELD: i32 = 1;
    pub const CTX_METHOD: i32 = 2;
    pub const CTX_CODE: i32 = 3;
    pub const CTX_COUNT: i32 = 4;
    pub fn new(&self)    {
    }
    pub fn parse(&self, cf: &DirectClassFile, context: i32, offset: i32, observer: &ParseObserver) -> Attribute    {
        if cf==None        {
            throw NullPointerException::new("cf == null");
        }        
        if (context<0)||(context>=AttributeFactory::CTX_COUNT)        {
            throw IllegalArgumentException::new("bad context");
        }        
        let name: CstString = None;
        try        {
            let bytes: ByteArray = cf.getBytes();
            let pool: ConstantPool = cf.getConstantPool();
            let nameIdx: i32 = bytes.getUnsignedShort(offset);
            let length: i32 = bytes.getInt(offset+2);
            name=(CstString*)pool.get(nameIdx);
            if observer!=None            {
                observer.parsed(&bytes, offset, 2, "name: "+name.toHuman());
                observer.parsed(&bytes, offset+2, 4, "length: "+Hex::u4(length));
            }            
            return parse0(&cf, context, name.getString(), offset+6, length, &observer);
        }        catch(        let ex: ParseException)        {
            ex.addContext("...while parsing "+(if (name!=None) { (name.toHuman()+" ") } else { "" })+"attribute at offset "+Hex::u4(offset));
                    throw ex;
                }
            }
            pub fn parse0(&self, cf: &DirectClassFile, context: i32, name: &String, offset: i32, length: i32, observer: &ParseObserver) -> Attribute            {
                let bytes: ByteArray = cf.getBytes();
                let pool: ConstantPool = cf.getConstantPool();
                let result: Attribute = RawAttribute::new(&name, &bytes, offset, length, &pool);
                if observer!=None                {
                    observer.parsed(&bytes, offset, length, "attribute data");
                }                
                return result;
            }
}
