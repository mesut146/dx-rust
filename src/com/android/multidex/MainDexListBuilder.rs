use crate::helper;
use crate::com::android::multidex::ClassReferenceListBuilder;
use crate::com::android::dx::cf::iface::FieldList;
use crate::com::android::dx::cf::iface::MethodList;
use crate::com::android::dx::cf::attrib::AttRuntimeVisibleAnnotations;
use crate::com::android::multidex::Path;
use crate::com::android::multidex::MainDexListBuilder;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::multidex::ClassPathElement;
use crate::com::android::dx::cf::iface::HasAttribute;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::cf::direct::DirectClassFile;

let static CLASS_EXTENSION: String = ".class";
let static EOL: String = System::getProperty_String("line.separator");
let static USAGE_MESSAGE: String = "Usage:"+MainDexListBuilder::EOL+MainDexListBuilder::EOL+"Short version: Don't use this."+MainDexListBuilder::EOL+MainDexListBuilder::EOL+"Slightly longer version: This tool is used by mainDexClasses script to build"+MainDexListBuilder::EOL+"the main dex list."+MainDexListBuilder::EOL;
let static DISABLE_ANNOTATION_RESOLUTION_WORKAROUND: String = "--disable-annotation-resolution-workaround";
struct MainDexListBuilder{
    pub filesToKeep: Set<String>,
}
impl MainDexListBuilder{
    pub const STATUS_ERROR: i32 = 1;
    pub fn main(args: &Vec<String>)    {
        let argIndex: i32 = 0;
        let keepAnnotated: boolean = true;
        while argIndex<args.len()-2        {
            if args[argIndex].equals(&MainDexListBuilder::DISABLE_ANNOTATION_RESOLUTION_WORKAROUND)            {
                keepAnnotated=false;
            }            else             {
                System::err.println_String("Invalid option "+args[argIndex]);
                MainDexListBuilder::printUsage();
                System::exit(MainDexListBuilder::STATUS_ERROR);
            }
            argIndex += 1;
        }
        if args.len()-argIndex!=2        {
            MainDexListBuilder::printUsage();
            System::exit(MainDexListBuilder::STATUS_ERROR);
        }        
        try        {
            let builder: MainDexListBuilder = MainDexListBuilder::new(keepAnnotated, args[argIndex], args[argIndex+1]);
            let toKeep: Set<String> = builder.getMainDexList();
            MainDexListBuilder::printList(&toKeep);
        }        catch(        let e: IOException)        {
            System::err.println_String("A fatal error occured: "+e.getMessage());
            System::exit(MainDexListBuilder::STATUS_ERROR);
            return;
        }
    }
    pub fn new(&self, keepAnnotated: boolean, rootJar: &String, pathString: &String)    {
        let jarOfRoots: ZipFile = None;
        let path: Path = None;
        try        {
            try            {
                jarOfRoots=ZipFile::new(&rootJar);
            }            catch(            let e: IOException)            {
                throw IOException::new("\""+rootJar+"\" can not be read as a zip archive. ("+e.getMessage()+")", &e);
            }
            path=Path::new(&pathString);
            let mainListBuilder: ClassReferenceListBuilder = ClassReferenceListBuilder::new(&path);
            mainListBuilder.addRoots(&jarOfRoots);
            for className in mainListBuilder.getClassNames()            {
                self.filesToKeep.add(className+MainDexListBuilder::CLASS_EXTENSION);
            }
            if keepAnnotated            {
                keepAnnotated(&path);
            }            
        }        finally        {
            try            {
                jarOfRoots.close();
            }            catch(            let e: IOException)            {
            }
            if path!=None            {
                for element in                 {
                    try                    {
                        element.close();
                    }                    catch(                    let e: IOException)                    {
                    }
                }
            }            
        }
    }
    pub fn getMainDexList(&self) -> Set<String>    {
        return self.filesToKeep;
    }
    pub fn printUsage()    {
        System::err.print_String(&MainDexListBuilder::USAGE_MESSAGE);
    }
    pub fn printList(fileNames: &Set<String>)    {
        for fileName in fileNames        {
            System::out.println_String(&fileName);
        }
    }
    pub fn keepAnnotated(&self, path: &Path)    {
        for element in path.getElements()        {
            forClazz:
        }
    }
    pub fn hasRuntimeVisibleAnnotation(&self, element: &HasAttribute) -> boolean    {
        let att: Attribute = element.getAttributes().findFirst(&AttRuntimeVisibleAnnotations::ATTRIBUTE_NAME);
        return (att!=None&&((AttRuntimeVisibleAnnotations*)att).getAnnotations().size()>0);
    }
}
