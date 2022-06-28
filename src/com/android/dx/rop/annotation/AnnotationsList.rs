use crate::helper;
use crate::com::android::dx::rop::annotation::AnnotationsList;
use crate::com::android::dx::rop::annotation::Annotations;

let static EMPTY: AnnotationsList = AnnotationsList::new(0);
struct AnnotationsList{
}
impl AnnotationsList{
    pub fn combine(list1: &AnnotationsList, list2: &AnnotationsList) -> AnnotationsList    {
        let size: i32 = list1.size();
        if size!=list2.size()        {
            throw IllegalArgumentException::new("list1.size() != list2.size()");
        }        
        let result: AnnotationsList = AnnotationsList::new(size);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let a1: Annotations = list1.get(i);
            let a2: Annotations = list2.get(i);
            result.set(i, Annotations::combine_Annotations_Annotations(&a1, &a2));
        }
        result.setImmutable();
        return result;
    }
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Annotations    {
        return (Annotations*)get0(n);
    }
    pub fn set(&self, n: i32, a: &Annotations)    {
        a.throwIfMutable();
        set0(n, &a);
    }
}
