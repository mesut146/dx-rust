use crate::helper;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::annotation::Annotations;

let static EMPTY: Annotations = Annotations::new();
struct Annotations{
    pub annotations: TreeMap<CstType,Annotation>,
}
impl Annotations{
    pub fn combine(a1: &Annotations, a2: &Annotations) -> Annotations    {
        let result: Annotations = Annotations::new();
        result.addAll(&a1);
        result.addAll(&a2);
        result.setImmutable();
        return result;
    }
    pub fn combine(annotations: &Annotations, annotation: &Annotation) -> Annotations    {
        let result: Annotations = Annotations::new();
        result.addAll(&annotations);
        result.add(&annotation);
        result.setImmutable();
        return result;
    }
    pub fn new(&self)    {
        self.annotations=TreeMap<CstType,Annotation>::new();
    }
    pub fn hashCode(&self) -> i32    {
        return self.annotations.hashCode();
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if !(//other instanceof Annotations)        {
            return false;
        }        
        let otherAnnotations: Annotations = (Annotations*)other;
        return self.annotations.equals(&);
    }
    pub fn compareTo(&self, other: &Annotations) -> i32    {
        let thisIter: Iterator<Annotation> = self.annotations.values().iterator();
        let otherIter: Iterator<Annotation> = .values().iterator();
        while thisIter.hasNext()&&otherIter.hasNext()        {
            let thisOne: Annotation = thisIter.next();
            let otherOne: Annotation = otherIter.next();
            let result: i32 = thisOne.compareTo(&otherOne);
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
        let sb: StringBuilder = StringBuilder::new();
        let first: boolean = true;
        sb.append_String("annotations{");
        for a in self.annotations.values()        {
            if first            {
                first=false;
            }            else             {
                sb.append_String(", ");
            }
            sb.append_String(a.toHuman());
        }
        sb.append_String("}");
        return sb.toString();
    }
    pub fn size(&self) -> i32    {
        return self.annotations.size();
    }
    pub fn add(&self, annotation: &Annotation)    {
        throwIfImmutable();
        if annotation==None        {
            throw NullPointerException::new("annotation == null");
        }        
        let type: CstType = annotation.getType();
        if self.annotations.containsKey(&type_renamed)        {
            throw IllegalArgumentException::new("duplicate type: "+type_renamed.toHuman());
        }        
        self.annotations.put(&type_renamed, &annotation);
    }
    pub fn addAll(&self, toAdd: &Annotations)    {
        throwIfImmutable();
        if toAdd==None        {
            throw NullPointerException::new("toAdd == null");
        }        
        for a in .values()        {
            add(&a);
        }
    }
    pub fn getAnnotations(&self) -> Collection<Annotation>    {
        return Collections::unmodifiableCollection(self.annotations.values());
    }
}
