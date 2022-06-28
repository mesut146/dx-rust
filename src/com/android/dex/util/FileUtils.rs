use crate::helper;

struct FileUtils{
}
impl FileUtils{
    pub fn new(&self)    {
    }
    pub fn readFile(fileName: &String) -> Vec<i8>    {
        let file: File = File::new(&fileName);
        return FileUtils::readFile_File(&file);
    }
    pub fn readFile(file: &File) -> Vec<i8>    {
        if !file.exists()        {
            throw RuntimeException::new(file+": file not found");
        }        
        if !file.isFile()        {
            throw RuntimeException::new(file+": not a file");
        }        
        if !file.canRead()        {
            throw RuntimeException::new(file+": file not readable");
        }        
        let longLength: i64 = file.length();
        let length: i32 = longLength as i32;
        if length!=longLength        {
            throw RuntimeException::new(file+": file too long");
        }        
        let result: Vec<i8> = new byte[length];
        try        {
            let in: FileInputStream = FileInputStream::new(&file);
            let at: i32 = 0;
            while length>0            {
                let amt: i32 = in_renamed.read_byte[]_int_int(&result, at, length);
                if amt==-1                {
                    throw RuntimeException::new(file+": unexpected EOF");
                }                
                at+=amt;
                length-=amt;
            }
            in_renamed.close();
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new(file+": trouble reading", &ex);
        }
        return result;
    }
    pub fn hasArchiveSuffix(fileName: &String) -> boolean    {
        return fileName.endsWith(".zip")||fileName.endsWith(".jar")||fileName.endsWith(".apk");
    }
}
