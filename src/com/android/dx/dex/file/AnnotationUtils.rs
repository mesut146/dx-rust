use crate::helper;
use crate::com::android::dx::rop::cst::CstAnnotation;
use crate::com::android::dx::rop::cst::CstArray::List;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::cst::CstArray;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;

let static ANNOTATION_DEFAULT_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/AnnotationDefault;"));
let static ENCLOSING_CLASS_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/EnclosingClass;"));
let static ENCLOSING_METHOD_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/EnclosingMethod;"));
let static INNER_CLASS_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/InnerClass;"));
let static MEMBER_CLASSES_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/MemberClasses;"));
let static SIGNATURE_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/Signature;"));
let static SOURCE_DEBUG_EXTENSION_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/SourceDebugExtension;"));
let static THROWS_TYPE: CstType = CstType::intern(Type::intern("Ldalvik/annotation/Throws;"));
let static ACCESS_FLAGS_STRING: CstString = CstString::new("accessFlags");
let static NAME_STRING: CstString = CstString::new("name");
let static VALUE_STRING: CstString = CstString::new("value");
struct AnnotationUtils{
}
impl AnnotationUtils{
    pub fn new(&self)    {
    }
    pub fn makeAnnotationDefault(defaults: &Annotation) -> Annotation    {
        let result: Annotation = Annotation::new(&AnnotationUtils::ANNOTATION_DEFAULT_TYPE, &AnnotationVisibility::SYSTEM);
        result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, CstAnnotation::new(&defaults)));
        result.setImmutable();
        return result;
    }
    pub fn makeEnclosingClass(clazz: &CstType) -> Annotation    {
        let result: Annotation = Annotation::new(&AnnotationUtils::ENCLOSING_CLASS_TYPE, &AnnotationVisibility::SYSTEM);
        result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, &clazz));
        result.setImmutable();
        return result;
    }
    pub fn makeEnclosingMethod(method: &CstMethodRef) -> Annotation    {
        let result: Annotation = Annotation::new(&AnnotationUtils::ENCLOSING_METHOD_TYPE, &AnnotationVisibility::SYSTEM);
        result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, &method));
        result.setImmutable();
        return result;
    }
    pub fn makeInnerClass(name: &CstString, accessFlags: i32) -> Annotation    {
        let result: Annotation = Annotation::new(&AnnotationUtils::INNER_CLASS_TYPE, &AnnotationVisibility::SYSTEM);
        let nameCst: Constant = if (name!=None) { name } else { CstKnownNull::THE_ONE };
                result.put(NameValuePair::new(&AnnotationUtils::NAME_STRING, &nameCst));
                result.put(NameValuePair::new(&AnnotationUtils::ACCESS_FLAGS_STRING, CstInteger::make(accessFlags)));
                result.setImmutable();
                return result;
            }
            pub fn makeMemberClasses(types: &TypeList) -> Annotation            {
                let array: CstArray = AnnotationUtils::makeCstArray(&types);
                let result: Annotation = Annotation::new(&AnnotationUtils::MEMBER_CLASSES_TYPE, &AnnotationVisibility::SYSTEM);
                result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, &array));
                result.setImmutable();
                return result;
            }
            pub fn makeSignature(signature: &CstString) -> Annotation            {
                let result: Annotation = Annotation::new(&AnnotationUtils::SIGNATURE_TYPE, &AnnotationVisibility::SYSTEM);
                let raw: String = signature.getString();
                let rawLength: i32 = raw.length();
                let pieces: ArrayList<String> = ArrayList<String>::new(20);
                for(                let at: i32 = 0;;at<rawLength)                {
                    let c: char = raw.charAt(at);
                    let endAt: i32 = at+1;
                    if c=='L'                    {
                        while endAt<rawLength                        {
                            c=raw.charAt(endAt);
                            if c==';'                            {
                                endAt += 1;
                                break;
                            }                            else                             if c=='<'                            {
                                break;
                            }                            
                            endAt += 1;
                        }
                    }                    else                     {
                        while endAt<rawLength                        {
                            c=raw.charAt(endAt);
                            if c=='L'                            {
                                break;
                            }                            
                            endAt += 1;
                        }
                    }
                    pieces.add_String(raw.substring_int_int(at, endAt));
                    at=endAt;
                }
                let size: i32 = pieces.size();
                let list: List = CstArray.List::new(size);
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    list.set(i, CstString::new(pieces.get(i)));
                }
                list.setImmutable();
                result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, CstArray::new(&list)));
                result.setImmutable();
                return result;
            }
            pub fn makeSourceDebugExtension(smapString: &CstString) -> Annotation            {
                let result: Annotation = Annotation::new(&AnnotationUtils::SOURCE_DEBUG_EXTENSION_TYPE, &AnnotationVisibility::SYSTEM);
                result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, &smapString));
                result.setImmutable();
                return result;
            }
            pub fn makeThrows(types: &TypeList) -> Annotation            {
                let array: CstArray = AnnotationUtils::makeCstArray(&types);
                let result: Annotation = Annotation::new(&AnnotationUtils::THROWS_TYPE, &AnnotationVisibility::SYSTEM);
                result.put(NameValuePair::new(&AnnotationUtils::VALUE_STRING, &array));
                result.setImmutable();
                return result;
            }
            pub fn makeCstArray(types: &TypeList) -> CstArray            {
                let size: i32 = types.size();
                let list: List = CstArray.List::new(size);
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    list.set(i, CstType::intern(types.getType(i)));
                }
                list.setImmutable();
                return CstArray::new(&list);
            }
}
