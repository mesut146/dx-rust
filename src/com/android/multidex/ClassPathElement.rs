use crate::helper;

let static SEPARATOR_CHAR: char = '/';
let static SEPARATOR_CHAR: char = '/';
trait ClassPathElement{
    pub fn open(&self, path: &String) -> InputStream;
    pub fn close(&self);
    pub fn list(&self) -> Iterable<String>;
}
