use crate::helper;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::direct::AttributeListParser;
use crate::com::android::dx::cf::direct::AttributeFactory;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::cf::iface::StdAttributeList;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::rop::cst::CstNat;

struct MemberListParser{
    pub cf: DirectClassFile,
    pub definer: CstType,
    pub offset: i32,
    pub attributeFactory: AttributeFactory,
    pub endOffset: i32,
    pub observer: ParseObserver,
}
impl MemberListParser{
    pub fn new(&self, cf: &DirectClassFile, definer: &CstType, offset: i32, attributeFactory: &AttributeFactory)    {
        if cf==None        {
            throw NullPointerException::new("cf == null");
        }        
        if offset<0        {
            throw IllegalArgumentException::new("offset < 0");
        }        
        if attributeFactory==None        {
            throw NullPointerException::new("attributeFactory == null");
        }        
        self->cf=cf;
        self->definer=definer;
        self->offset=offset;
        self->attributeFactory=attributeFactory;
        self->endOffset=-1;
    }
    pub fn getEndOffset(&self) -> i32    {
        parseIfNecessary();
        return self.endOffset;
    }
    pub fn setObserver(&self, observer: &ParseObserver)    {
        self->observer=observer;
    }
    pub fn parseIfNecessary(&self)    {
        if self.endOffset<0        {
            parse();
        }        
    }
    pub fn getCount(&self) -> i32    {
        let bytes: ByteArray = self.cf.getBytes();
        return bytes.getUnsignedShort(self.offset);
    }
    pub fn getDefiner(&self) -> CstType    {
        return self.definer;
    }
    pub fn humanName(&self) -> String;
    pub fn humanAccessFlags(&self, accessFlags: i32) -> String;
    pub fn getAttributeContext(&self) -> i32;
    pub fn set(&self, n: i32, accessFlags: i32, nat: &CstNat, attributes: &AttributeList) -> Member;
    pub fn parse(&self)    {
        let attributeContext: i32 = getAttributeContext();
        let count: i32 = getCount();
        let at: i32 = self.offset+2;
        let bytes: ByteArray = self.cf.getBytes();
        let pool: ConstantPool = self.cf.getConstantPool();
        if self.observer!=None        {
            self.observer.parsed(&bytes, self.offset, 2, humanName()+"s_count: "+Hex::u2(count));
        }        
        for(        let i: i32 = 0;;i<counti += 1)        {
            try            {
                let accessFlags: i32 = bytes.getUnsignedShort(at);
                let nameIdx: i32 = bytes.getUnsignedShort(at+2);
                let descIdx: i32 = bytes.getUnsignedShort(at+4);
                let name: CstString = (CstString*)pool.get(nameIdx);
                let desc: CstString = (CstString*)pool.get(descIdx);
                if self.observer!=None                {
                    self.observer.startParsingMember(&bytes, at, name.getString(), desc.getString());
                    self.observer.parsed(&bytes, at, 0, "\n"+humanName()+"s["+i+"]:\n");
                    self.observer.changeIndent(1);
                    self.observer.parsed(&bytes, at, 2, "access_flags: "+humanAccessFlags(accessFlags));
                    self.observer.parsed(&bytes, at+2, 2, "name: "+name.toHuman());
                    self.observer.parsed(&bytes, at+4, 2, "descriptor: "+desc.toHuman());
                }                
                at+=6;
                let parser: AttributeListParser = AttributeListParser::new(&self.cf, attributeContext, at, &self.attributeFactory);
                parser.setObserver(&self.observer);
                at=parser.getEndOffset();
                let attributes: StdAttributeList = parser.getList();
                attributes.setImmutable();
                let nat: CstNat = CstNat::new(&name, &desc);
                let member: Member = set_int_int_CstNat_AttributeList(i, accessFlags, &nat, &attributes);
                if self.observer!=None                {
                    self.observer.changeIndent(-1);
                    self.observer.parsed(&bytes, at, 0, "end "+humanName()+"s["+i+"]\n");
                    self.observer.endParsingMember(&bytes, at, name.getString(), desc.getString(), &member);
                }                
            }            catch(            let ex: ParseException)            {
                ex.addContext("...while parsing "+humanName()+"s["+i+"]");
                throw ex;
            }            catch(            let ex: RuntimeException)            {
                let pe: ParseException = ParseException::new(&ex);
                pe.addContext("...while parsing "+humanName()+"s["+i+"]");
                throw pe;
            }
        }
        self.endOffset=at;
    }
}
