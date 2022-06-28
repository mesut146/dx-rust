use crate::helper;

struct FolderPathElement{
    pub baseFolder: File,
}
impl FolderPathElement{
    pub fn new(&self, baseFolder: &File)    {
        self->baseFolder=baseFolder;
    }
    pub fn open(&self, path: &String) -> InputStream    {
        return FileInputStream::new(File::new(&self.baseFolder, path.replace_char_char(ClassPathElement::SEPARATOR_CHAR, File::separatorChar)));
    }
    pub fn close(&self)    {
    }
    pub fn list(&self) -> Iterable<String>    {
        let result: ArrayList<String> = ArrayList<String>::new();
        collect(&self.baseFolder, "", &result);
        return result;
    }
    pub fn collect(&self, folder: &File, prefix: &String, result: &ArrayList<String>)    {
        for file in folder.listFiles()        {
            if file.isDirectory()            {
                collect(&file, prefix+ClassPathElement::SEPARATOR_CHAR+file.getName(), &result);
            }            else             {
                result.add_String(prefix+ClassPathElement::SEPARATOR_CHAR+file.getName());
            }
        }
    }
}
