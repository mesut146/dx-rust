use crate::helper;

struct Writers{
}
impl Writers{
    pub fn new(&self)    {
    }
    pub fn printWriterFor(writer: &Writer) -> PrintWriter    {
        if //writer instanceof PrintWriter        {
            return (PrintWriter*)writer;
        }        
        return PrintWriter::new(&writer);
    }
}
