use crate::helper;
use crate::com::android::dx::cf::iface::Attribute;
use crate::com::android::dx::cf::direct::AttributeFactory;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::cf::iface::StdAttributeList;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct AttributeListParser{
    pub cf: DirectClassFile,
    pub context: i32,
    pub offset: i32,
    pub attributeFactory: AttributeFactory,
    pub list: StdAttributeList,
    pub endOffset: i32,
    pub observer: ParseObserver,
}
impl AttributeListParser{
    pub fn new(&self, cf: &DirectClassFile, context: i32, offset: i32, attributeFactory: &AttributeFactory)    {
        if cf==None        {
            throw NullPointerException::new("cf == null");
        }        
        if attributeFactory==None        {
            throw NullPointerException::new("attributeFactory == null");
        }        
        let size: i32 = cf.getBytes().getUnsignedShort(offset);
        self->cf=cf;
        self->context=context;
        self->offset=offset;
        self->attributeFactory=attributeFactory;
        self->list=StdAttributeList::new(size);
        self->endOffset=-1;
    }
    pub fn setObserver(&self, observer: &ParseObserver)    {
        self->observer=observer;
    }
    pub fn getEndOffset(&self) -> i32    {
        parseIfNecessary();
        return self.endOffset;
    }
    pub fn getList(&self) -> StdAttributeList    {
        parseIfNecessary();
        return self.list;
    }
    pub fn parseIfNecessary(&self)    {
        if self.endOffset<0        {
            parse();
        }        
    }
    pub fn parse(&self)    {
        let sz: i32 = self.list.size();
        let at: i32 = self.offset+2;
        let bytes: ByteArray = self.cf.getBytes();
        if self.observer!=None        {
            self.observer.parsed(&bytes, self.offset, 2, "attributes_count: "+Hex::u2(sz));
        }        
        for(        let i: i32 = 0;;i<szi += 1)        {
            try            {
                if self.observer!=None                {
                    self.observer.parsed(&bytes, at, 0, "\nattributes["+i+"]:\n");
                    self.observer.changeIndent(1);
                }                
                let attrib: Attribute = self.attributeFactory.parse(&self.cf, self.context, at, &self.observer);
                at+=attrib.byteLength();
                self.list.set(i, &attrib);
                if self.observer!=None                {
                    self.observer.changeIndent(-1);
                    self.observer.parsed(&bytes, at, 0, "end attributes["+i+"]\n");
                }                
            }            catch(            let ex: ParseException)            {
                ex.addContext("...while parsing attributes["+i+"]");
                throw ex;
            }            catch(            let ex: RuntimeException)            {
                let pe: ParseException = ParseException::new(&ex);
                pe.addContext("...while parsing attributes["+i+"]");
                throw pe;
            }
        }
        self.endOffset=at;
    }
}
