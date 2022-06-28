use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::rop::annotation::AnnotationVisibility;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::rop::cst::CstType;

struct Annotation{
    pub type: CstType,
    pub visibility: AnnotationVisibility,
    pub elements: TreeMap<CstString,NameValuePair>,
}
impl Annotation{
    pub fn new(&self, type: &CstType, visibility: &AnnotationVisibility)    {
        if type==None        {
            throw NullPointerException::new("type == null");
        }        
        if visibility==None        {
            throw NullPointerException::new("visibility == null");
        }        
        self->type=type;
        self->visibility=visibility;
        self->elements=TreeMap<CstString,NameValuePair>::new();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof Annotation)        {
            return false;
        }        
        let otherAnnotation: Annotation = (Annotation*)other;
        if !(self.type_renamed.equals(&)&&(self.visibility==))        {
            return false;
        }        
        return self.elements.equals(&);
    }
    pub fn hashCode(&self) -> i32    {
        let hash: i32 = self.type_renamed.hashCode();
        hash=(hash*31)+self.elements.hashCode();
        hash=(hash*31)+self.visibility.hashCode();
        return hash;
    }
    pub fn compareTo(&self, other: &Annotation) -> i32    {
        let result: i32 = self.type_renamed.compareTo(&);
        if result!=0        {
            return result;
        }        
        result=self.visibility.compareTo(&);
        if result!=0        {
            return result;
        }        
        let thisIter: Iterator<NameValuePair> = self.elements.values().iterator();
        let otherIter: Iterator<NameValuePair> = .values().iterator();
        while thisIter.hasNext()&&otherIter.hasNext()        {
            let thisOne: NameValuePair = thisIter.next();
            let otherOne: NameValuePair = otherIter.next();
            result=thisOne.compareTo(&otherOne);
            if result!=0            {
                return result;
            }            
        }
        if thisIter.hasNext()        {
            return 1;
        }        else         if otherIter.hasNext()        {
            return -1;
        }        
        return 0;
    }
    pub fn toString(&self) -> String    {
        return toHuman();
    }
    pub fn toHuman(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        sb.append_String(self.visibility.toHuman());
        sb.append_String("-annotation ");
        sb.append_String(self.type_renamed.toHuman());
        sb.append_String(" {");
        let first: boolean = true;
        for pair in self.elements.values()        {
            if first            {
                first=false;
            }            else             {
                sb.append_String(", ");
            }
            sb.append_String(pair.getName().toHuman());
            sb.append_String(": ");
            sb.append_String(pair.getValue().toHuman());
        }
        sb.append_String("}");
        return sb.toString();
    }
    pub fn getType(&self) -> CstType    {
        return self.type_renamed;
    }
    pub fn getVisibility(&self) -> AnnotationVisibility    {
        return self.visibility;
    }
    pub fn put(&self, pair: &NameValuePair)    {
        throwIfImmutable();
        if pair==None        {
            throw NullPointerException::new("pair == null");
        }        
        self.elements.put(pair.getName(), &pair);
    }
    pub fn add(&self, pair: &NameValuePair)    {
        throwIfImmutable();
        if pair==None        {
            throw NullPointerException::new("pair == null");
        }        
        let name: CstString = pair.getName();
        if self.elements.get(&name)!=None        {
            throw IllegalArgumentException::new("name already added: "+name);
        }        
        self.elements.put(&name, &pair);
    }
    pub fn getNameValuePairs(&self) -> Collection<NameValuePair>    {
        return Collections::unmodifiableCollection(self.elements.values());
    }
}
