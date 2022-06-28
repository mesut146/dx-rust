use crate::helper;
use crate::com::android::multidex::ArchivePathElement::DirectoryEntryException;

struct ArchivePathElement{
    pub archive: ZipFile,
}
impl ArchivePathElement{
    pub fn new(&self, archive: &ZipFile)    {
        self->archive=archive;
    }
    pub fn open(&self, path: &String) -> InputStream    {
        let entry: ZipEntry = self.archive.getEntry_String(&path);
        if entry==None        {
            throw FileNotFoundException::new("File \""+path+"\" not found");
        }        else         if entry.isDirectory()        {
            throw DirectoryEntryException::new();
        }        else         {
            return self.archive.getInputStream(&entry);
        }
    }
    pub fn close(&self)    {
        self.archive.close();
    }
    pub fn list(&self) -> Iterable<String>    {
        return /*new Iterable<String>(){
  @Override public Iterator<String> iterator(){
    return new Iterator<String>(){
      Enumeration<? extends ZipEntry> delegate=archive.entries();
      ZipEntry next=null;
      @Override public boolean hasNext(){
        while (next == null && delegate.hasMoreElements()) {
          next=delegate.nextElement();
          if (next.isDirectory()) {
            next=null;
          }
        }
        return next != null;
      }
      @Override public String next(){
        if (hasNext()) {
          String name=next.getName();
          next=null;
          return name;
        }
 else {
          throw new NoSuchElementException();
        }
      }
      @Override public void remove(){
        throw new UnsupportedOperationException();
      }
    }
;
  }
}
*/;
    }
}
