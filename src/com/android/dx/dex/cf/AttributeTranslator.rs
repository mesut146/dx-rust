use crate::helper;
use crate::com::android::dx::cf::attrib::AttSignature;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::cf::attrib::AttExceptions;
use crate::com::android::dx::cf::iface::MethodList;
use crate::com::android::dx::rop::annotation::AnnotationVisibility;
use crate::com::android::dx::dex::file::AnnotationUtils;
use crate::com::android::dx::util::Warning;
use crate::com::android::dx::cf::attrib::InnerClassList;
use crate::com::android::dx::cf::attrib::AttRuntimeVisibleParameterAnnotations;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::cf::attrib::AttInnerClasses;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::cf::attrib::AttSourceDebugExtension;
use crate::com::android::dx::cf::attrib::AttRuntimeVisibleAnnotations;
use crate::com::android::dx::cf::attrib::AttRuntimeInvisibleParameterAnnotations;
use crate::com::android::dx::rop::annotation::AnnotationsList;
use crate::com::android::dx::cf::attrib::AttEnclosingMethod;
use crate::com::android::dx::cf::attrib::AttRuntimeInvisibleAnnotations;
use crate::com::android::dx::cf::attrib::InnerClassList::Item;
use crate::com::android::dx::cf::attrib::AttAnnotationDefault;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::dex::cf::CfOptions;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::rop::cst::CstNat;

struct AttributeTranslator{
}
impl AttributeTranslator{
    pub fn new(&self)    {
    }
    pub fn getExceptions(method: &Method) -> TypeList    {
        let attribs: AttributeList = method.getAttributes();
        let exceptions: AttExceptions = (AttExceptions*)attribs.findFirst(&AttExceptions::ATTRIBUTE_NAME);
        if exceptions==None        {
            return StdTypeList::EMPTY;
        }        
        return exceptions.getExceptions();
    }
    pub fn getAnnotations(attribs: &AttributeList) -> Annotations    {
        let result: Annotations = AttributeTranslator::getAnnotations0(&attribs);
        let signature: Annotation = AttributeTranslator::getSignature(&attribs);
        let sourceDebugExtension: Annotation = AttributeTranslator::getSourceDebugExtension(&attribs);
        if signature!=None        {
            result=Annotations::combine_Annotations_Annotation(&result, &signature);
        }        
        if sourceDebugExtension!=None        {
            result=Annotations::combine_Annotations_Annotation(&result, &sourceDebugExtension);
        }        
        return result;
    }
    pub fn getClassAnnotations(cf: &DirectClassFile, args: &CfOptions) -> Annotations    {
        let thisClass: CstType = cf.getThisClass();
        let attribs: AttributeList = cf.getAttributes();
        let result: Annotations = AttributeTranslator::getAnnotations(&attribs);
        let enclosingMethod: Annotation = AttributeTranslator::translateEnclosingMethod(&attribs);
        try        {
            let innerClassAnnotations: Annotations = AttributeTranslator::translateInnerClasses(&thisClass, &attribs, enclosingMethod==None);
            if innerClassAnnotations!=None            {
                result=Annotations::combine_Annotations_Annotations(&result, &innerClassAnnotations);
            }            
        }        catch(        let warn: Warning)        {
            .println_String("warning: "+warn.getMessage());
        }
        if enclosingMethod!=None        {
            result=Annotations::combine_Annotations_Annotation(&result, &enclosingMethod);
        }        
        if AccessFlags::isAnnotation(cf.getAccessFlags())        {
            let annotationDefault: Annotation = AttributeTranslator::translateAnnotationDefaults(&cf);
            if annotationDefault!=None            {
                result=Annotations::combine_Annotations_Annotation(&result, &annotationDefault);
            }            
        }        
        return result;
    }
    pub fn getMethodAnnotations(method: &Method) -> Annotations    {
        let result: Annotations = AttributeTranslator::getAnnotations(method.getAttributes());
        let exceptions: TypeList = AttributeTranslator::getExceptions(&method);
        if exceptions.size()!=0        {
            let throwsAnnotation: Annotation = AnnotationUtils::makeThrows(&exceptions);
            result=Annotations::combine_Annotations_Annotation(&result, &throwsAnnotation);
        }        
        return result;
    }
    pub fn getAnnotations0(attribs: &AttributeList) -> Annotations    {
        let visible: AttRuntimeVisibleAnnotations = (AttRuntimeVisibleAnnotations*)attribs.findFirst(&AttRuntimeVisibleAnnotations::ATTRIBUTE_NAME);
        let invisible: AttRuntimeInvisibleAnnotations = (AttRuntimeInvisibleAnnotations*)attribs.findFirst(&AttRuntimeInvisibleAnnotations::ATTRIBUTE_NAME);
        if visible==None        {
            if invisible==None            {
                return Annotations::EMPTY;
            }            
            return invisible.getAnnotations();
        }        
        if invisible==None        {
            return visible.getAnnotations();
        }        
        return Annotations::combine_Annotations_Annotations(visible.getAnnotations(), invisible.getAnnotations());
    }
    pub fn getSignature(attribs: &AttributeList) -> Annotation    {
        let signature: AttSignature = (AttSignature*)attribs.findFirst(&AttSignature::ATTRIBUTE_NAME);
        if signature==None        {
            return None;
        }        
        return AnnotationUtils::makeSignature(signature.getSignature());
    }
    pub fn getSourceDebugExtension(attribs: &AttributeList) -> Annotation    {
        let extension: AttSourceDebugExtension = (AttSourceDebugExtension*)attribs.findFirst(&AttSourceDebugExtension::ATTRIBUTE_NAME);
        if extension==None        {
            return None;
        }        
        return AnnotationUtils::makeSourceDebugExtension(extension.getSmapString());
    }
    pub fn translateEnclosingMethod(attribs: &AttributeList) -> Annotation    {
        let enclosingMethod: AttEnclosingMethod = (AttEnclosingMethod*)attribs.findFirst(&AttEnclosingMethod::ATTRIBUTE_NAME);
        if enclosingMethod==None        {
            return None;
        }        
        let enclosingClass: CstType = enclosingMethod.getEnclosingClass();
        let nat: CstNat = enclosingMethod.getMethod();
        if nat==None        {
            return AnnotationUtils::makeEnclosingClass(&enclosingClass);
        }        
        return AnnotationUtils::makeEnclosingMethod(CstMethodRef::new(&enclosingClass, &nat));
    }
    pub fn translateInnerClasses(thisClass: &CstType, attribs: &AttributeList, needEnclosingClass: boolean) -> Annotations    {
        let innerClasses: AttInnerClasses = (AttInnerClasses*)attribs.findFirst(&AttInnerClasses::ATTRIBUTE_NAME);
        if innerClasses==None        {
            return None;
        }        
        let list: InnerClassList = innerClasses.getInnerClasses();
        let size: i32 = list.size();
        let foundThisClass: Item = None;
        let membersList: ArrayList<Type> = ArrayList<Type>::new();
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let item: Item = list.get(i);
            let innerClass: CstType = item.getInnerClass();
            if innerClass.equals(&thisClass)            {
                foundThisClass=item;
            }            else             if thisClass.equals(item.getOuterClass())            {
                membersList.add_Type(innerClass.getClassType());
            }            
        }
        let membersSize: i32 = membersList.size();
        if (foundThisClass==None)&&(membersSize==0)        {
            return None;
        }        
        let result: Annotations = Annotations::new();
        if foundThisClass!=None        {
            result.add(AnnotationUtils::makeInnerClass(foundThisClass.getInnerName(), foundThisClass.getAccessFlags()));
            if needEnclosingClass            {
                let outer: CstType = foundThisClass.getOuterClass();
                if outer==None                {
                    throw Warning::new("Ignoring InnerClasses attribute for an "+"anonymous inner class\n"+"("+thisClass.toHuman()+") that doesn't come with an\n"+"associated EnclosingMethod attribute. "+"This class was probably produced by a\n"+"compiler that did not target the modern "+".class file format. The recommended\n"+"solution is to recompile the class from "+"source, using an up-to-date compiler\n"+"and without specifying any \"-target\" type "+"options. The consequence of ignoring\n"+"this warning is that reflective operations "+"on this class will incorrectly\n"+"indicate that it is *not* an inner class.");
                }                
                result.add(AnnotationUtils::makeEnclosingClass(foundThisClass.getOuterClass()));
            }            
        }        
        if membersSize!=0        {
            let typeList: StdTypeList = StdTypeList::new(membersSize);
            for(            let i: i32 = 0;;i<membersSizei += 1)            {
                typeList.set(i, membersList.get(i));
            }
            typeList.setImmutable();
            result.add(AnnotationUtils::makeMemberClasses(&typeList));
        }        
        result.setImmutable();
        return result;
    }
    pub fn getParameterAnnotations(method: &Method) -> AnnotationsList    {
        let attribs: AttributeList = method.getAttributes();
        let visible: AttRuntimeVisibleParameterAnnotations = (AttRuntimeVisibleParameterAnnotations*)attribs.findFirst(&AttRuntimeVisibleParameterAnnotations::ATTRIBUTE_NAME);
        let invisible: AttRuntimeInvisibleParameterAnnotations = (AttRuntimeInvisibleParameterAnnotations*)attribs.findFirst(&AttRuntimeInvisibleParameterAnnotations::ATTRIBUTE_NAME);
        if visible==None        {
            if invisible==None            {
                return AnnotationsList::EMPTY;
            }            
            return invisible.getParameterAnnotations();
        }        
        if invisible==None        {
            return visible.getParameterAnnotations();
        }        
        return AnnotationsList::combine(visible.getParameterAnnotations(), invisible.getParameterAnnotations());
    }
    pub fn translateAnnotationDefaults(cf: &DirectClassFile) -> Annotation    {
        let thisClass: CstType = cf.getThisClass();
        let methods: MethodList = cf.getMethods();
        let sz: i32 = methods.size();
        let result: Annotation = Annotation::new(&thisClass, &AnnotationVisibility::EMBEDDED);
        let any: boolean = false;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Method = methods.get(i);
            let attribs: AttributeList = one.getAttributes();
            let oneDefault: AttAnnotationDefault = (AttAnnotationDefault*)attribs.findFirst(&AttAnnotationDefault::ATTRIBUTE_NAME);
            if oneDefault!=None            {
                let pair: NameValuePair = NameValuePair::new(one.getNat().getName(), oneDefault.getValue());
                result.add(&pair);
                any=true;
            }            
        }
        if !any        {
            return None;
        }        
        result.setImmutable();
        return AnnotationUtils::makeAnnotationDefault(&result);
    }
}
