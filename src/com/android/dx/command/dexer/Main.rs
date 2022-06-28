use crate::helper;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::command::dexer::DxContext;
use crate::com::android::dx::dex::cf::CodeStatistics;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::command::dexer::Main::RemoveModuleInfoFilter;
use crate::com::android::dx::cf::direct::ClassPathOpener;
use crate::com::android::dx::command::dexer::Main::NotFilter;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::command::dexer::Main::MainDexListFilter;
use crate::com::android::dx::command::dexer::Main::Arguments;
use crate::com::android::dex::DexFormat;
use crate::com::android::dx::dex::cf::CfOptions;
use crate::com::android::dx::dex::file::ClassDefsSection;
use crate::com::android::dx::command::UsageException;
use crate::com::android::dx::command::dexer::Main::ClassParserTask;
use crate::com::android::dx::cf::iface::FieldList;
use crate::com::android::dx::merge::DexMerger;
use crate::com::android::dx::command::dexer::Main::Arguments::ArgumentsParser;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::dex::cf::CfTranslator;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dex::Dex;
use crate::com::android::dx::rop::annotation::AnnotationsList;
use crate::com::android::dx::dex::file::ClassDefItem;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dex::DexException;
use crate::com::android::dx::command::dexer::Main::StopProcessing;
use crate::com::android::dx::cf::iface::MethodList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::command::dexer::Main::BestEffortMainDexListFilter;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::Version;
use crate::com::android::dx::command::dexer::Main::DexWriter;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::cf::direct::ClassPathOpener::FileNameFilter;
use crate::com::android::dx::command::dexer::Main::ClassTranslatorTask;
use crate::com::android::dx::command::dexer::Main::ClassDefItemConsumer;
use crate::com::android::dx::dex::file::Statistics;
use crate::com::android::dx::merge::CollisionPolicy;
use crate::com::android::dx::command::dexer::Main::DirectClassFileConsumer;
use crate::com::android::dx::command::dexer::Main::FileBytesConsumer;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::util::FileUtils;
use crate::com::android::dx::dex::file::EncodedMethod;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::dex::code::PositionList;
use crate::com::android::dx::command::dexer::Main;

let static DEX_EXTENSION: String = ".dex";
let static DEX_PREFIX: String = "classes";
let static IN_RE_CORE_CLASSES: String = "Ill-advised or mistaken usage of a core class (java.* or javax.*)\n"+"when not building a core library.\n\n"+"This is often due to inadvertently including a core library file\n"+"in your application's project, when using an IDE (such as\n"+"Eclipse). If you are sure you're not intentionally defining a\n"+"core class, then this is the most likely explanation of what's\n"+"going on.\n\n"+"However, you might actually be trying to define a class in a core\n"+"namespace, the source of which you may have taken, for example,\n"+"from a non-Android virtual machine project. This will most\n"+"assuredly not work. At a minimum, it jeopardizes the\n"+"compatibility of your app with future versions of the platform.\n"+"It is also often of questionable legality.\n\n"+"If you really intend to build a core library -- which is only\n"+"appropriate as part of creating a full virtual machine\n"+"distribution, as opposed to compiling an application -- then use\n"+"the \"--core-library\" option to suppress this error message.\n\n"+"If you go ahead and use \"--core-library\" but are in fact\n"+"building an application, then be forewarned that your application\n"+"will still fail to build or run, at some point. Please be\n"+"prepared for angry customers who find, for example, that your\n"+"application ceases to function once they upgrade their operating\n"+"system. You will be to blame for this problem.\n\n"+"If you are legitimately using some code that happens to be in a\n"+"core package, then the easiest safe alternative you have is to\n"+"repackage that code. That is, move the classes in question into\n"+"your own package namespace. This means that they will never be in\n"+"conflict with core system classes. JarJar is a tool that may help\n"+"you in this endeavor. If you find that you cannot do this, then\n"+"that is an indication that the path you are on will ultimately\n"+"lead to pain, suffering, grief, and lamentation.\n";
let static MANIFEST_NAME: String = "META-INF/MANIFEST.MF";
let static CREATED_BY: Attributes.Name = Attributes.Name::new("Created-By");
let static JAVAX_CORE: Vec<String> = vec!["accessibility", "crypto", "imageio", "management", "naming", "net", "print", "rmi", "security", "sip", "sound", "sql", "swing", "transaction", "xml"];
struct Main{
    pub errors: AtomicInteger,
    pub args: Arguments,
    pub outputDex: DexFile,
    pub outputResources: TreeMap<String,byte[]>,
    pub libraryDexBuffers: List<byte[]>,
    pub classTranslatorPool: ExecutorService,
    pub classDefItemConsumer: ExecutorService,
    pub addToDexFutures: List<Future<Boolean>>,
    pub dexOutPool: ExecutorService,
    pub dexOutputFutures: List<Future<byte[]>>,
    pub dexRotationLock: Object,
    pub maxMethodIdsInProcess: i32,
    pub maxFieldIdsInProcess: i32,
    pub anyFilesProcessed: boolean,
    pub minimumFileAge: i64,
    pub classesInMainDex: Set<String>,
    pub dexOutputArrays: List<byte[]>,
    pub humanOutWriter: OutputStreamWriter,
    pub context: DxContext,
}
impl Main{
    pub const MAX_METHOD_ADDED_DURING_DEX_CREATION: i32 = 2;
    pub const MAX_FIELD_ADDED_DURING_DEX_CREATION: i32 = 9;
    pub fn new(&self, context: &DxContext)    {
        self->context=context;
    }
    pub fn main(argArray: &Vec<String>) -> i32    {
        let context: DxContext = DxContext::new();
        let arguments: Arguments = Arguments::new(&context);
        arguments.parse(&argArray);
        let result: i32 = Main::new(&context).runDx(&arguments);
        if result!=0        {
        }        
        return result;
    }
    pub fn clearInternTables()    {
        Prototype::clearInternTable();
        RegisterSpec::clearInternTable();
        CstType::clearInternTable();
        Type::clearInternTable();
    }
    pub fn run(arguments: &Arguments) -> i32    {
        return Main::new(DxContext::new()).runDx(&arguments);
    }
    pub fn runDx(&self, arguments: &Arguments) -> i32    {
        self.errors.set(0);
        self.libraryDexBuffers.clear();
        self.args=arguments;
        self.args.makeOptionsObjects();
        let humanOutRaw: OutputStream = None;
        if !=None        {
            humanOutRaw=openOutput(&);
            self.humanOutWriter=OutputStreamWriter::new(&humanOutRaw);
        }        
        try        {
            if             {
                return runMultiDex();
            }            else             {
                return runMonoDex();
            }
        }        finally        {
            closeOutput(&humanOutRaw);
        }
    }
    pub fn runMonoDex(&self) -> i32    {
        let incrementalOutFile: File = None;
        if         {
            if ==None            {
                .println_String("error: no incremental output name specified");
                return -1;
            }            
            incrementalOutFile=File::new(&);
            if incrementalOutFile.exists()            {
                self.minimumFileAge=incrementalOutFile.lastModified();
            }            
        }        
        if !processAllFiles()        {
            return 1;
        }        
        if &&!self.anyFilesProcessed        {
            return 0;
        }        
        let outArray: Vec<i8> = None;
        if !self.outputDex.isEmpty()||(!=None)        {
            outArray=writeDex(&self.outputDex);
            if outArray==None            {
                return 2;
            }            
        }        
        if         {
            outArray=mergeIncremental(&outArray, &incrementalOutFile);
        }        
        outArray=mergeLibraryDexBuffers(&outArray);
        if         {
            self.outputDex=None;
            if outArray!=None            {
                self.outputResources.put(&DexFormat::DEX_IN_JAR_NAME, &outArray);
            }            
            if !createJar(&)            {
                return 3;
            }            
        }        else         if outArray!=None&&!=None        {
            let out: OutputStream = openOutput(&);
            out.write_byte[](&outArray);
            closeOutput(&out);
        }        
        return 0;
    }
    pub fn runMultiDex(&self) -> i32    {
//assert !args.incremental;

        if !=None        {
            self.classesInMainDex=HashSet<String>::new();
            Main::readPathsFromFile(&, &self.classesInMainDex);
        }        
        self.dexOutPool=Executors::newFixedThreadPool_int();
        if !processAllFiles()        {
            return 1;
        }        
        if !self.libraryDexBuffers.isEmpty()        {
            throw DexException::new("Library dex files are not supported in multi-dex mode");
        }        
        if self.outputDex!=None        {
            self.dexOutputFutures.add_Future<byte[]>(self.dexOutPool.submit_Callable<byte[]>(DexWriter::new(&self.outputDex, self)));
            self.outputDex=None;
        }        
        try        {
            self.dexOutPool.shutdown();
            if !self.dexOutPool.awaitTermination(600L, &TimeUnit::SECONDS)            {
                throw RuntimeException::new("Timed out waiting for dex writer threads.");
            }            
            for f in self.dexOutputFutures            {
                self.dexOutputArrays.add_byte[](f.get());
            }
        }        catch(        let ex: InterruptedException)        {
            self.dexOutPool.shutdownNow();
            throw RuntimeException::new("A dex writer thread has been interrupted.");
        }        catch(        let e: Exception)        {
            self.dexOutPool.shutdownNow();
            throw RuntimeException::new("Unexpected exception in dex writer thread");
        }
        if         {
            for(            let i: i32 = 0;;i<self.dexOutputArrays.size()i += 1)            {
                self.outputResources.put(Main::getDexFileName(i), self.dexOutputArrays.get(i));
            }
            if !createJar(&)            {
                return 3;
            }            
        }        else         if !=None        {
            let outDir: File = File::new(&);
//assert outDir.isDirectory();

            for(            let i: i32 = 0;;i<self.dexOutputArrays.size()i += 1)            {
                let out: OutputStream = FileOutputStream::new(File::new(&outDir, Main::getDexFileName(i)));
                try                {
                    out.write_byte[](self.dexOutputArrays.get(i));
                }                finally                {
                    closeOutput(&out);
                }
            }
        }        
        return 0;
    }
    pub fn getDexFileName(i: i32) -> String    {
        if i==0        {
            return DexFormat::DEX_IN_JAR_NAME;
        }        else         {
            return Main::DEX_PREFIX+(i+1)+Main::DEX_EXTENSION;
        }
    }
    pub fn readPathsFromFile(fileName: &String, paths: &Collection<String>)    {
        let bfr: BufferedReader = None;
        try        {
            let fr: FileReader = FileReader::new(&fileName);
            bfr=BufferedReader::new(&fr);
            let line: String;
            while None!=(line=bfr.readLine())            {
                paths.add(Main::fixPath(&line));
            }
        }        finally        {
            if bfr!=None            {
                bfr.close();
            }            
        }
    }
    pub fn mergeIncremental(&self, update: &Vec<i8>, base: &File) -> Vec<i8>    {
        let dexA: Dex = None;
        let dexB: Dex = None;
        if update!=None        {
            dexA=Dex::new(&update);
        }        
        if base.exists()        {
            dexB=Dex::new(&base);
        }        
        let result: Dex;
        if dexA==None&&dexB==None        {
            return None;
        }        else         if dexA==None        {
            result=dexB;
        }        else         if dexB==None        {
            result=dexA;
        }        else         {
            result=DexMerger::new(vec![dexA, dexB], &CollisionPolicy::KEEP_FIRST, &self.context).merge();
        }
        let bytesOut: ByteArrayOutputStream = ByteArrayOutputStream::new();
        result.writeTo_OutputStream(&bytesOut);
        return bytesOut.toByteArray();
    }
    pub fn mergeLibraryDexBuffers(&self, outArray: &Vec<i8>) -> Vec<i8>    {
        let dexes: ArrayList<Dex> = ArrayList<Dex>::new();
        if outArray!=None        {
            dexes.add_Dex(Dex::new(&outArray));
        }        
        for libraryDex in self.libraryDexBuffers        {
            dexes.add_Dex(Dex::new(&libraryDex));
        }
        if dexes.isEmpty()        {
            return None;
        }        
        let merged: Dex = DexMerger::new(dexes.toArray_Dex[](new Dex[dexes.size()]), &CollisionPolicy::FAIL, &self.context).merge();
        return merged.getBytes();
    }
    pub fn processAllFiles(&self) -> boolean    {
        createDexFile();
        if         {
            self.outputResources=TreeMap<String,byte[]>::new();
        }        
        self.anyFilesProcessed=false;
        let fileNames: Vec<String> = ;
        Arrays::sort_Object[](&fileNames);
        self.classTranslatorPool=ThreadPoolExecutor::new(, , 0, &TimeUnit::SECONDS, ArrayBlockingQueue<Runnable>::new(2*, true), ThreadPoolExecutor.CallerRunsPolicy::new());
        self.classDefItemConsumer=Executors::newSingleThreadExecutor();
        try        {
            if !=None            {
                let mainPassFilter: FileNameFilter = if  { MainDexListFilter::new(, self) } else { BestEffortMainDexListFilter::new(, self) };
                        for(                        let i: i32 = 0;;i<fileNames.len()i += 1)                        {
                            processOne(fileNames[i], &mainPassFilter);
                        }
                        if self.dexOutputFutures.size()>0                        {
                            throw DexException::new("Too many classes in "+Arguments::MAIN_DEX_LIST_OPTION+", main dex capacity exceeded");
                        }                        
                        if                         {
//synchronized (dexRotationLock) {
  while (maxMethodIdsInProcess > 0 || maxFieldIdsInProcess > 0) {
    try {
      dexRotationLock.wait();
    }
 catch (    InterruptedException ex) {
    }
  }
}

                            rotateDexFile();
                        }                        
                        let filter: FileNameFilter = RemoveModuleInfoFilter::new(NotFilter::new(&mainPassFilter));
                        for(                        let i: i32 = 0;;i<fileNames.len()i += 1)                        {
                            processOne(fileNames[i], &filter);
                        }
                    }                    else                     {
                        let filter: FileNameFilter = RemoveModuleInfoFilter::new(&ClassPathOpener::acceptAll);
                        for(                        let i: i32 = 0;;i<fileNames.len()i += 1)                        {
                            processOne(fileNames[i], &filter);
                        }
                    }
                }                catch(                let ex: StopProcessing)                {
                }
                try                {
                    self.classTranslatorPool.shutdown();
                    self.classTranslatorPool.awaitTermination(600L, &TimeUnit::SECONDS);
                    self.classDefItemConsumer.shutdown();
                    self.classDefItemConsumer.awaitTermination(600L, &TimeUnit::SECONDS);
                    for f in self.addToDexFutures                    {
                        try                        {
                            f.get();
                        }                        catch(                        let ex: ExecutionException)                        {
                            let count: i32 = self.errors.incrementAndGet();
                            if count<10                            {
                                if                                 {
                                    .println_String("Uncaught translation error:");
                                    ex.getCause().printStackTrace_PrintStream(&);
                                }                                else                                 {
                                    .println_String("Uncaught translation error: "+ex.getCause());
                                }
                            }                            else                             {
                                throw InterruptedException::new("Too many errors");
                            }
                        }
                    }
                }                catch(                let ie: InterruptedException)                {
                    self.classTranslatorPool.shutdownNow();
                    self.classDefItemConsumer.shutdownNow();
                    throw RuntimeException::new("Translation has been interrupted", &ie);
                }                catch(                let e: Exception)                {
                    self.classTranslatorPool.shutdownNow();
                    self.classDefItemConsumer.shutdownNow();
                    e.printStackTrace_PrintStream(&);
                    throw RuntimeException::new("Unexpected exception in translator thread.", &e);
                }
                let errorNum: i32 = self.errors.get();
                if errorNum!=0                {
                    .println_String(errorNum+" error"+(if (errorNum==1) { "" } else { "s" })+"; aborting");
                            return false;
                        }                        
                        if &&!self.anyFilesProcessed                        {
                            return true;
                        }                        
                        if !(self.anyFilesProcessed||)                        {
                            .println_String("no classfiles specified");
                            return false;
                        }                        
                        if &&                        {
                            .dumpStatistics(&);
                        }                        
                        return true;
                    }
                    pub fn createDexFile(&self)                    {
                        self.outputDex=DexFile::new(&);
                        if !=0                        {
                            self.outputDex.setDumpWidth();
                        }                        
                    }
                    pub fn rotateDexFile(&self)                    {
                        if self.outputDex!=None                        {
                            if self.dexOutPool!=None                            {
                                self.dexOutputFutures.add_Future<byte[]>(self.dexOutPool.submit_Callable<byte[]>(DexWriter::new(&self.outputDex, self)));
                            }                            else                             {
                                self.dexOutputArrays.add_byte[](writeDex(&self.outputDex));
                            }
                        }                        
                        createDexFile();
                    }
                    pub fn processOne(&self, pathname: &String, filter: &FileNameFilter)                    {
                        let opener: ClassPathOpener;
                        opener=ClassPathOpener::new(&pathname, true, &filter, FileBytesConsumer::new(, self));
                        if opener.process()                        {
                            updateStatus(true);
                        }                        
                    }
                    pub fn updateStatus(&self, res: boolean)                    {
                        self.anyFilesProcessed|=res;
                    }
                    pub fn processFileBytes(&self, name: &String, lastModified: i64, bytes: &Vec<i8>) -> boolean                    {
                        let isClass: boolean = name.endsWith(".class");
                        let isClassesDex: boolean = name.equals(&DexFormat::DEX_IN_JAR_NAME);
                        let keepResources: boolean = (self.outputResources!=None);
                        if !isClass&&!isClassesDex&&!keepResources                        {
                            if                             {
                                .println_String("ignored resource "+name);
                            }                            
                            return false;
                        }                        
                        if                         {
                            .println_String("processing "+name+"...");
                        }                        
                        let fixedName: String = Main::fixPath(&name);
                        if isClass                        {
                            if keepResources&&                            {
//synchronized (outputResources) {
  outputResources.put(fixedName,bytes);
}

                            }                            
                            if lastModified<self.minimumFileAge                            {
                                return true;
                            }                            
                            processClass(&fixedName, &bytes);
                            return false;
                        }                        else                         if isClassesDex                        {
//synchronized (libraryDexBuffers) {
  libraryDexBuffers.add(bytes);
}

                            return true;
                        }                        else                         {
//synchronized (outputResources) {
  outputResources.put(fixedName,bytes);
}

                            return true;
                        }
                    }
                    pub fn processClass(&self, name: &String, bytes: &Vec<i8>) -> boolean                    {
                        if !                        {
                            checkClassName(&name);
                        }                        
                        try                        {
                            DirectClassFileConsumer::new(&name, &bytes, None, self).call_DirectClassFile(ClassParserTask::new(&name, &bytes, self).call());
                        }                        catch(                        let ex: ParseException)                        {
                            throw ex;
                        }                        catch(                        let ex: Exception)                        {
                            throw RuntimeException::new("Exception parsing classes", &ex);
                        }
                        return true;
                    }
                    pub fn parseClass(&self, name: &String, bytes: &Vec<i8>) -> DirectClassFile                    {
                        let cf: DirectClassFile = DirectClassFile::new(&bytes, &name, );
                        cf.setAttributeFactory(&StdAttributeFactory::THE_ONE);
                        cf.getMagic();
                        return cf;
                    }
                    pub fn translateClass(&self, bytes: &Vec<i8>, cf: &DirectClassFile) -> ClassDefItem                    {
                        try                        {
                            return CfTranslator::translate(&self.context, &cf, &bytes, &, &, &self.outputDex);
                        }                        catch(                        let ex: ParseException)                        {
                            .println_String("\ntrouble processing:");
                            if                             {
                                ex.printStackTrace_PrintStream(&);
                            }                            else                             {
                                ex.printContext_PrintStream(&);
                            }
                        }
                        self.errors.incrementAndGet();
                        return None;
                    }
                    pub fn addClassToDex(&self, clazz: &ClassDefItem) -> boolean                    {
//synchronized (outputDex) {
  outputDex.add(clazz);
}

                        return true;
                    }
                    pub fn checkClassName(&self, name: &String)                    {
                        let bogus: boolean = false;
                        if name.startsWith_String("java/")                        {
                            bogus=true;
                        }                        else                         if name.startsWith_String("javax/")                        {
                            let slashAt: i32 = name.indexOf_int_int('/', 6);
                            if slashAt==-1                            {
                                bogus=true;
                            }                            else                             {
                                let pkg: String = name.substring_int_int(6, slashAt);
                                bogus=(Arrays::binarySearch_Object[]_Object(&Main::JAVAX_CORE, &pkg)>=0);
                            }
                        }                        
                        if !bogus                        {
                            return;
                        }                        
                        .println_String("\ntrouble processing \""+name+"\":\n\n"+Main::IN_RE_CORE_CLASSES);
                        self.errors.incrementAndGet();
                        throw StopProcessing::new();
                    }
                    pub fn writeDex(&self, outputDex: &DexFile) -> Vec<i8>                    {
                        let outArray: Vec<i8> = None;
                        try                        {
                            try                            {
                                if !=None                                {
                                    outputDex.toDex(None, false);
                                    dumpMethod(&outputDex, &, &self.humanOutWriter);
                                }                                else                                 {
                                    outArray=outputDex.toDex(&self.humanOutWriter, );
                                }
                                if                                 {
                                    .println_String(outputDex.getStatistics().toHuman());
                                }                                
                            }                            finally                            {
                                if self.humanOutWriter!=None                                {
                                    self.humanOutWriter.flush();
                                }                                
                            }
                        }                        catch(                        let ex: Exception)                        {
                            if                             {
                                .println_String("\ntrouble writing output:");
                                ex.printStackTrace_PrintStream(&);
                            }                            else                             {
                                .println_String("\ntrouble writing output: "+ex.getMessage());
                            }
                            return None;
                        }
                        return outArray;
                    }
                    pub fn createJar(&self, fileName: &String) -> boolean                    {
                        try                        {
                            let manifest: Manifest = makeManifest();
                            let out: OutputStream = openOutput(&fileName);
                            let jarOut: JarOutputStream = JarOutputStream::new(&out, &manifest);
                            try                            {
                                for e in self.outputResources.entrySet()                                {
                                    let name: String = e.getKey();
                                    let contents: Vec<i8> = e.getValue();
                                    let entry: JarEntry = JarEntry::new(&name);
                                    let length: i32 = contents.len();
                                    if                                     {
                                        .println_String("writing "+name+"; size "+length+"...");
                                    }                                    
                                    entry.setSize(length);
                                    jarOut.putNextEntry(&entry);
                                    jarOut.write_byte[](&contents);
                                    jarOut.closeEntry();
                                }
                            }                            finally                            {
                                jarOut.finish();
                                jarOut.flush();
                                closeOutput(&out);
                            }
                        }                        catch(                        let ex: Exception)                        {
                            if                             {
                                .println_String("\ntrouble writing output:");
                                ex.printStackTrace_PrintStream(&);
                            }                            else                             {
                                .println_String("\ntrouble writing output: "+ex.getMessage());
                            }
                            return false;
                        }
                        return true;
                    }
                    pub fn makeManifest(&self) -> Manifest                    {
                        let manifestBytes: Vec<i8> = self.outputResources.get(&Main::MANIFEST_NAME);
                        let manifest: Manifest;
                        let attribs: Attributes;
                        if manifestBytes==None                        {
                            manifest=Manifest::new();
                            attribs=manifest.getMainAttributes();
                            attribs.put(&Name::MANIFEST_VERSION, "1.0");
                        }                        else                         {
                            manifest=Manifest::new(ByteArrayInputStream::new(&manifestBytes));
                            attribs=manifest.getMainAttributes();
                            self.outputResources.remove(&Main::MANIFEST_NAME);
                        }
                        let createdBy: String = attribs.getValue_Name(&Main::CREATED_BY);
                        if createdBy==None                        {
                            createdBy="";
                        }                        else                         {
                            createdBy+=" + ";
                        }
                        createdBy+="dx "+Version::VERSION;
                        attribs.put(&Main::CREATED_BY, &createdBy);
                        attribs.putValue("Dex-Location", &DexFormat::DEX_IN_JAR_NAME);
                        return manifest;
                    }
                    pub fn openOutput(&self, name: &String) -> OutputStream                    {
                        if name.equals("-")||name.startsWith_String("-.")                        {
                            return ;
                        }                        
                        return FileOutputStream::new(&name);
                    }
                    pub fn closeOutput(&self, stream: &OutputStream)                    {
                        if stream==None                        {
                            return;
                        }                        
                        stream.flush();
                        if stream!=                        {
                            stream.close();
                        }                        
                    }
                    pub fn fixPath(path: &String) -> String                    {
                        if File::separatorChar=='\\'                        {
                            path=path.replace_char_char('\\', '/');
                        }                        
                        let index: i32 = path.lastIndexOf_String("/./");
                        if index!=-1                        {
                            return path.substring_int(index+3);
                        }                        
                        if path.startsWith_String("./")                        {
                            return path.substring_int(2);
                        }                        
                        return path;
                    }
                    pub fn dumpMethod(&self, dex: &DexFile, fqName: &String, out: &OutputStreamWriter)                    {
                        let wildcard: boolean = fqName.endsWith("*");
                        let lastDot: i32 = fqName.lastIndexOf_int('.');
                        if (lastDot<=0)||(lastDot==(fqName.length()-1))                        {
                            .println_String("bogus fully-qualified method name: "+fqName);
                            return;
                        }                        
                        let className: String = fqName.substring_int_int(0, lastDot).replace_char_char('.', '/');
                        let methodName: String = fqName.substring_int(lastDot+1);
                        let clazz: ClassDefItem = dex.getClassOrNull(&className);
                        if clazz==None                        {
                            .println_String("no such class: "+className);
                            return;
                        }                        
                        if wildcard                        {
                            methodName=methodName.substring_int_int(0, methodName.length()-1);
                        }                        
                        let allMeths: ArrayList<EncodedMethod> = clazz.getMethods();
                        let meths: TreeMap<CstNat,EncodedMethod> = TreeMap<CstNat,EncodedMethod>::new();
                        for meth in allMeths                        {
                            let methName: String = meth.getName().getString();
                            if (wildcard&&methName.startsWith_String(&methodName))||(!wildcard&&methName.equals(&methodName))                            {
                                meths.put(meth.getRef().getNat(), &meth);
                            }                            
                        }
                        if meths.size()==0                        {
                            .println_String("no such method: "+fqName);
                            return;
                        }                        
                        let pw: PrintWriter = PrintWriter::new(&out);
                        for meth in meths.values()                        {
                            meth.debugPrint(&pw, );
                            let sourceFile: CstString = clazz.getSourceFile();
                            if sourceFile!=None                            {
                                pw.println_String("  source file: "+sourceFile.toQuoted());
                            }                            
                            let methodAnnotations: Annotations = clazz.getMethodAnnotations(meth.getRef());
                            let parameterAnnotations: AnnotationsList = clazz.getParameterAnnotations(meth.getRef());
                            if methodAnnotations!=None                            {
                                pw.println_String("  method annotations:");
                                for a in methodAnnotations.getAnnotations()                                {
                                    pw.println_String("    "+a);
                                }
                            }                            
                            if parameterAnnotations!=None                            {
                                pw.println_String("  parameter annotations:");
                                let sz: i32 = parameterAnnotations.size();
                                for(                                let i: i32 = 0;;i<szi += 1)                                {
                                    pw.println_String("    parameter "+i);
                                    let annotations: Annotations = parameterAnnotations.get(i);
                                    for a in annotations.getAnnotations()                                    {
                                        pw.println_String("      "+a);
                                    }
                                }
                            }                            
                        }
                        pw.flush();
                    }
}
