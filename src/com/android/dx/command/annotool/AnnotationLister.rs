use crate::helper;
use crate::com::android::dx::command::annotool::Main::Arguments;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::cf::direct::ClassPathOpener;
use crate::com::android::dx::cf::direct::ClassPathOpener::Consumer;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::cf::attrib::AttRuntimeVisibleAnnotations;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::cf::attrib::AttRuntimeInvisibleAnnotations;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::cf::attrib::BaseAnnotations;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::cf::direct::DirectClassFile;

let static PACKAGE_INFO: String = "package-info";
struct AnnotationLister{
    pub args: Main.Arguments,
    pub matchInnerClassesOf: HashSet<String>,
    pub matchPackages: HashSet<String>,
}
impl AnnotationLister{
    pub fn new(&self, args: &Main.Arguments)    {
        self->args=args;
    }
    pub fn process(&self)    {
        for path in         {
            let opener: ClassPathOpener;
            opener=ClassPathOpener::new(&path, true, /*new ClassPathOpener.Consumer(){
  @Override public boolean processFileBytes(  String name,  long lastModified,  byte[] bytes){
    if (!name.endsWith(".class")) {
      return true;
    }
    ByteArray ba=new ByteArray(bytes);
    DirectClassFile cf=new DirectClassFile(ba,name,true);
    cf.setAttributeFactory(StdAttributeFactory.THE_ONE);
    AttributeList attributes=cf.getAttributes();
    Attribute att;
    String cfClassName=cf.getThisClass().getClassType().getClassName();
    if (cfClassName.endsWith(PACKAGE_INFO)) {
      att=attributes.findFirst(AttRuntimeInvisibleAnnotations.ATTRIBUTE_NAME);
      for (; att != null; att=attributes.findNext(att)) {
        BaseAnnotations ann=(BaseAnnotations)att;
        visitPackageAnnotation(cf,ann);
      }
      att=attributes.findFirst(AttRuntimeVisibleAnnotations.ATTRIBUTE_NAME);
      for (; att != null; att=attributes.findNext(att)) {
        BaseAnnotations ann=(BaseAnnotations)att;
        visitPackageAnnotation(cf,ann);
      }
    }
 else     if (isMatchingInnerClass(cfClassName) || isMatchingPackage(cfClassName)) {
      printMatch(cf);
    }
 else {
      att=attributes.findFirst(AttRuntimeInvisibleAnnotations.ATTRIBUTE_NAME);
      for (; att != null; att=attributes.findNext(att)) {
        BaseAnnotations ann=(BaseAnnotations)att;
        visitClassAnnotation(cf,ann);
      }
      att=attributes.findFirst(AttRuntimeVisibleAnnotations.ATTRIBUTE_NAME);
      for (; att != null; att=attributes.findNext(att)) {
        BaseAnnotations ann=(BaseAnnotations)att;
        visitClassAnnotation(cf,ann);
      }
    }
    return true;
  }
  @Override public void onException(  Exception ex){
    throw new RuntimeException(ex);
  }
  @Override public void onProcessArchiveStart(  File file){
  }
}
*/);
            opener.process();
        }
    }
    pub fn visitClassAnnotation(&self, cf: &DirectClassFile, ann: &BaseAnnotations)    {
        if !.contains(&ElementType::TYPE)        {
            return;
        }        
        for anAnn in ann.getAnnotations().getAnnotations()        {
            let annClassName: String = anAnn.getType().getClassType().getClassName();
            if .equals(&annClassName)            {
                printMatch(&cf);
            }            
        }
    }
    pub fn visitPackageAnnotation(&self, cf: &DirectClassFile, ann: &BaseAnnotations)    {
        if !.contains(&ElementType::PACKAGE)        {
            return;
        }        
        let packageName: String = cf.getThisClass().getClassType().getClassName();
        let slashIndex: i32 = packageName.lastIndexOf_int('/');
        if slashIndex==-1        {
            packageName="";
        }        else         {
            packageName=packageName.substring_int_int(0, slashIndex);
        }
        for anAnn in ann.getAnnotations().getAnnotations()        {
            let annClassName: String = anAnn.getType().getClassType().getClassName();
            if .equals(&annClassName)            {
                printMatchPackage(&packageName);
            }            
        }
    }
    pub fn printMatchPackage(&self, packageName: &String)    {
        for pt in         {
            match pt{PrintType::CLASS => PrintType::INNERCLASS => PrintType::METHOD =>                 self.matchPackages.add(&packageName);                break;PrintType::PACKAGE =>                 System::out.println_String(packageName.replace_char_char('/', '.'));                break;            }
        }
    }
    pub fn printMatch(&self, cf: &DirectClassFile)    {
        for pt in         {
            match pt{PrintType::CLASS =>                 let classname: String;                classname=cf.getThisClass().getClassType().getClassName();                classname=classname.replace_char_char('/', '.');                System::out.println_String(&classname);                break;PrintType::INNERCLASS =>                 self.matchInnerClassesOf.add(cf.getThisClass().getClassType().getClassName());                break;PrintType::METHOD =>                 break;PrintType::PACKAGE =>                 break;            }
        }
    }
    pub fn isMatchingInnerClass(&self, s: &String) -> boolean    {
        let i: i32;
        while 0<(i=s.lastIndexOf_int('$'))        {
            s=s.substring_int_int(0, i);
            if self.matchInnerClassesOf.contains(&s)            {
                return true;
            }            
        }
        return false;
    }
    pub fn isMatchingPackage(&self, s: &String) -> boolean    {
        let slashIndex: i32 = s.lastIndexOf_int('/');
        let packageName: String;
        if slashIndex==-1        {
            packageName="";
        }        else         {
            packageName=s.substring_int_int(0, slashIndex);
        }
        return self.matchPackages.contains(&packageName);
    }
}
