use crate::helper;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::command::dump::ClassDumper;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct ClassDumper{
}
impl ClassDumper{
    pub fn dump(bytes: &Vec<i8>, out: &PrintStream, filePath: &String, args: &Args)    {
        let cd: ClassDumper = ClassDumper::new(&bytes, &out, &filePath, &args);
        cd.dump();
    }
    pub fn new(&self, bytes: &Vec<i8>, out: &PrintStream, filePath: &String, args: &Args)    {
        super(bytes,out,filePath,args);

    }
    pub fn dump(&self)    {
        let bytes: Vec<i8> = getBytes();
        let ba: ByteArray = ByteArray::new(&bytes);
        let cf: DirectClassFile = DirectClassFile::new(&ba, getFilePath(), getStrictParse());
        cf.setAttributeFactory(&StdAttributeFactory::THE_ONE);
        cf.setObserver(self);
        cf.getMagic();
        let readBytes: i32 = getReadBytes();
        if readBytes!=bytes.len()        {
            parsed(&ba, readBytes, bytes.len()-readBytes, "<extra data at end of file>");
        }        
    }
}
