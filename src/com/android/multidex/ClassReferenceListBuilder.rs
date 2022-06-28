use crate::helper;
use crate::com::android::dx::cf::iface::FieldList;
use crate::com::android::dx::cf::iface::MethodList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::multidex::Path;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::multidex::MainDexListBuilder;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::cf::iface::Field;
use crate::com::android::dx::cf::direct::DirectClassFile;

let static CLASS_EXTENSION: String = ".class";
struct ClassReferenceListBuilder{
    pub path: Path,
    pub classNames: Set<String>,
}
impl ClassReferenceListBuilder{
    pub fn new(&self, path: &Path)    {
        self->path=path;
    }
    pub fn main(args: &Vec<String>)    {
        MainDexListBuilder::main(&args);
    }
    pub fn addRoots(&self, jarOfRoots: &ZipFile)    {
        for(        let entries: Enumeration<? extends ZipEntry> = jarOfRoots.entries();;entries.hasMoreElements())        {
            let entry: ZipEntry = entries.nextElement();
            let name: String = entry.getName();
            if name.endsWith(&ClassReferenceListBuilder::CLASS_EXTENSION)            {
                self.classNames.add(name.substring_int_int(0, name.length()-ClassReferenceListBuilder::CLASS_EXTENSION.length()));
            }            
        }
        for(        let entries: Enumeration<? extends ZipEntry> = jarOfRoots.entries();;entries.hasMoreElements())        {
            let entry: ZipEntry = entries.nextElement();
            let name: String = entry.getName();
            if name.endsWith(&ClassReferenceListBuilder::CLASS_EXTENSION)            {
                let classFile: DirectClassFile;
                try                {
                    classFile=self.path.getClass(&name);
                }                catch(                let e: FileNotFoundException)                {
                    throw IOException::new("Class "+name+" is missing form original class path "+self.path, &e);
                }
                addDependencies(&classFile);
            }            
        }
    }
    pub fn getClassNames(&self) -> Set<String>    {
        return self.classNames;
    }
    pub fn addDependencies(&self, classFile: &DirectClassFile)    {
        for constant in classFile.getConstantPool().getEntries()        {
            if //constant instanceof CstType            {
                checkDescriptor(((CstType*)constant).getClassType().getDescriptor());
            }            else             if //constant instanceof CstFieldRef            {
                checkDescriptor(((CstFieldRef*)constant).getType().getDescriptor());
            }            else             if //constant instanceof CstBaseMethodRef            {
                checkPrototype(((CstBaseMethodRef*)constant).getPrototype());
            }            
        }
        let fields: FieldList = classFile.getFields();
        let nbField: i32 = fields.size();
        for(        let i: i32 = 0;;i<nbFieldi += 1)        {
            checkDescriptor(fields.get(i).getDescriptor().getString());
        }
        let methods: MethodList = classFile.getMethods();
        let nbMethods: i32 = methods.size();
        for(        let i: i32 = 0;;i<nbMethodsi += 1)        {
            checkPrototype(Prototype::intern_String(methods.get(i).getDescriptor().getString()));
        }
    }
    pub fn checkPrototype(&self, proto: &Prototype)    {
        checkDescriptor(proto.getReturnType().getDescriptor());
        let args: StdTypeList = proto.getParameterTypes();
        for(        let i: i32 = 0;;i<args.size()i += 1)        {
            checkDescriptor(args.get(i).getDescriptor());
        }
    }
    pub fn checkDescriptor(&self, typeDescriptor: &String)    {
        if typeDescriptor.endsWith(";")        {
            let lastBrace: i32 = typeDescriptor.lastIndexOf_int('[');
            if lastBrace<0            {
                addClassWithHierachy(typeDescriptor.substring_int_int(1, typeDescriptor.length()-1));
            }            else             {
//assert typeDescriptor.length() > lastBrace + 3 && typeDescriptor.charAt(lastBrace + 1) == 'L';

                addClassWithHierachy(typeDescriptor.substring_int_int(lastBrace+2, typeDescriptor.length()-1));
            }
        }        
    }
    pub fn addClassWithHierachy(&self, classBinaryName: &String)    {
        if self.classNames.contains(&classBinaryName)        {
            return;
        }        
        try        {
            let classFile: DirectClassFile = self.path.getClass(classBinaryName+ClassReferenceListBuilder::CLASS_EXTENSION);
            self.classNames.add(&classBinaryName);
            let superClass: CstType = classFile.getSuperclass();
            if superClass!=None            {
                addClassWithHierachy(superClass.getClassType().getClassName());
            }            
            let interfaceList: TypeList = classFile.getInterfaces();
            let interfaceNumber: i32 = interfaceList.size();
            for(            let i: i32 = 0;;i<interfaceNumberi += 1)            {
                addClassWithHierachy(interfaceList.getType(i).getClassName());
            }
        }        catch(        let e: FileNotFoundException)        {
        }
    }
}
