use crate::helper;
use crate::com::android::multidex::ArchivePathElement;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::multidex::FolderPathElement;
use crate::com::android::multidex::ClassPathElement;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct Path{
    pub elements: List<ClassPathElement>,
    pub definition: String,
    pub baos: ByteArrayOutputStream,
    pub readBuffer: Vec<i8>,
}
impl Path{
    pub fn getClassPathElement(file: &File) -> ClassPathElement    {
        if file.isDirectory()        {
            return FolderPathElement::new(&file);
        }        else         if file.isFile()        {
            return ArchivePathElement::new(ZipFile::new(&file));
        }        else         if file.exists()        {
            throw IOException::new("\""+file.getPath()+"\" is not a directory neither a zip file");
        }        else         {
            throw FileNotFoundException::new("File \""+file.getPath()+"\" not found");
        }
    }
    pub fn new(&self, definition: &String)    {
        self->definition=definition;
        for filePath in definition.split_String(Pattern::quote(&File::pathSeparator))        {
            try            {
                addElement(Path::getClassPathElement(File::new(&filePath)));
            }            catch(            let e: IOException)            {
                throw IOException::new("Wrong classpath: "+e.getMessage(), &e);
            }
        }
    }
    pub fn readStream(in: &InputStream, baos: &ByteArrayOutputStream, readBuffer: &Vec<i8>) -> Vec<i8>    {
        try        {
            for(;)            {
                let amt: i32 = in.read_byte[](&readBuffer);
                if amt<0                {
                    break;
                }                
                baos.write_byte[]_int_int(&readBuffer, 0, amt);
            }
        }        finally        {
            in.close();
        }
        return baos.toByteArray();
    }
    pub fn toString(&self) -> String    {
        return self.definition;
    }
    pub fn getElements(&self) -> Iterable<ClassPathElement>    {
        return self.elements;
    }
    pub fn addElement(&self, element: &ClassPathElement)    {
//assert element != null;

        self.elements.add_ClassPathElement(&element);
    }
    pub fn getClass(&self, path: &String) -> DirectClassFile    {
        let classFile: DirectClassFile = None;
        for element in self.elements        {
            try            {
                let in: InputStream = element.open(&path);
                try                {
                    let bytes: Vec<i8> = Path::readStream(&in_renamed, &self.baos, &self.readBuffer);
                    self.baos.reset();
                    classFile=DirectClassFile::new(&bytes, &path, false);
                    classFile.setAttributeFactory(&StdAttributeFactory::THE_ONE);
                    break;
                }                finally                {
                    in_renamed.close();
                }
            }            catch(            let e: IOException)            {
            }
        }
        if classFile==None        {
            throw FileNotFoundException::new("File \""+path+"\" not found");
        }        
        return classFile;
    }
}
