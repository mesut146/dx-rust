use crate::helper;
use crate::com::android::dx::cf::direct::ClassPathOpener::FileNameFilter;
use crate::com::android::dex::util::FileUtils;
use crate::com::android::dx::cf::direct::ClassPathOpener::Consumer;

let static acceptAll: FileNameFilter = /*new FileNameFilter(){
  @Override public boolean accept(  String path){
    return true;
  }
}
*/;
struct ClassPathOpener{
    pub pathname: String,
    pub consumer: Consumer,
    pub sort: boolean,
    pub filter: FileNameFilter,
}
impl ClassPathOpener{
    pub fn new(&self, pathname: &String, sort: boolean, consumer: &Consumer)    {
        this(pathname,sort,acceptAll,consumer);

    }
    pub fn new(&self, pathname: &String, sort: boolean, filter: &FileNameFilter, consumer: &Consumer)    {
        self->pathname=pathname;
        self->sort=sort;
        self->consumer=consumer;
        self->filter=filter;
    }
    pub fn process(&self) -> boolean    {
        let file: File = File::new(&self.pathname);
        return processOne(&file, true);
    }
    pub fn processOne(&self, file: &File, topLevel: boolean) -> boolean    {
        try        {
            if file.isDirectory()            {
                return processDirectory(&file, topLevel);
            }            
            let path: String = file.getPath();
            if path.endsWith(".zip")||path.endsWith(".jar")||path.endsWith(".apk")            {
                return processArchive(&file);
            }            
            if self.filter.accept(&path)            {
                let bytes: Vec<i8> = FileUtils::readFile_File(&file);
                return self.consumer.processFileBytes(&path, file.lastModified(), &bytes);
            }            else             {
                return false;
            }
        }        catch(        let ex: Exception)        {
            self.consumer.onException(&ex);
            return false;
        }
    }
    pub fn compareClassNames(a: &String, b: &String) -> i32    {
        a=a.replace_char_char('$', '0');
        b=b.replace_char_char('$', '0');
        a=a.replace_CharSequence_CharSequence("package-info", "");
        b=b.replace_CharSequence_CharSequence("package-info", "");
        return a.compareTo(&b);
    }
    pub fn processDirectory(&self, dir: &File, topLevel: boolean) -> boolean    {
        if topLevel        {
            dir=File::new(&dir, ".");
        }        
        let files: Vec<File> = dir.listFiles();
        let len: i32 = files.len();
        let any: boolean = false;
        if self.sort        {
            Arrays::sort_File[]_Comparator<? super File>(&files, /*new Comparator<File>(){
  @Override public int compare(  File a,  File b){
    return compareClassNames(a.getName(),b.getName());
  }
}
*/);
        }        
        for(        let i: i32 = 0;;i<leni += 1)        {
            any|=processOne(files[i], false);
        }
        return any;
    }
    pub fn processArchive(&self, file: &File) -> boolean    {
        let zip: ZipFile = ZipFile::new(&file);
        let entriesList: ArrayList<? extends ZipEntry> = Collections::list(zip.entries());
        if self.sort        {
            Collections::sort_List<>_Comparator<? super >(&entriesList, /*new Comparator<ZipEntry>(){
  @Override public int compare(  ZipEntry a,  ZipEntry b){
    return compareClassNames(a.getName(),b.getName());
  }
}
*/);
        }        
        self.consumer.onProcessArchiveStart(&file);
        let baos: ByteArrayOutputStream = ByteArrayOutputStream::new(40000);
        let buf: Vec<i8> = new byte[20000];
        let any: boolean = false;
        for one in entriesList        {
            let isDirectory: boolean = one.isDirectory();
            let path: String = one.getName();
            if self.filter.accept(&path)            {
                let bytes: Vec<i8>;
                if !isDirectory                {
                    let in: InputStream = zip.getInputStream(&one);
                    baos.reset();
                    let read: i32;
                    while (read=in_renamed.read_byte[](&buf))!=-1                    {
                        baos.write_byte[]_int_int(&buf, 0, read);
                    }
                    in_renamed.close();
                    bytes=baos.toByteArray();
                }                else                 {
                    bytes=new byte[0];
                }
                any|=self.consumer.processFileBytes(&path, one.getTime(), &bytes);
            }            
        }
        zip.close();
        return any;
    }
}
