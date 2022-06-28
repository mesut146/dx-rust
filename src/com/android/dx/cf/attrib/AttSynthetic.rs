use crate::helper;

let static ATTRIBUTE_NAME: String = "Synthetic";
struct AttSynthetic{
}
impl AttSynthetic{
    pub fn new(&self)    {
        super(ATTRIBUTE_NAME);

    }
    pub fn byteLength(&self) -> i32    {
        return 6;
    }
}
