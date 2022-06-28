use crate::helper;

let static ATTRIBUTE_NAME: String = "Deprecated";
struct AttDeprecated{
}
impl AttDeprecated{
    pub fn new(&self)    {
        super(ATTRIBUTE_NAME);

    }
    pub fn byteLength(&self) -> i32    {
        return 6;
    }
}
