use crate::helper;
use crate::com::android::dx::cf::code::BootstrapMethodsList;
use crate::com::android::dx::cf::code::BootstrapMethodArgumentsList;
use crate::com::android::dx::cf::code::BootstrapMethodsList::Item;

let static ATTRIBUTE_NAME: String = "BootstrapMethods";
struct AttBootstrapMethods{
    pub bootstrapMethods: BootstrapMethodsList,
    pub byteLength: i32,
}
impl AttBootstrapMethods{
    pub const ATTRIBUTE_HEADER_BYTES: i32 = 8;
    pub const BOOTSTRAP_METHOD_BYTES: i32 = 4;
    pub const BOOTSTRAP_ARGUMENT_BYTES: i32 = 2;
    pub fn new(&self, bootstrapMethods: &BootstrapMethodsList)    {
        super(ATTRIBUTE_NAME);

        self->bootstrapMethods=bootstrapMethods;
        let bytes: i32 = AttBootstrapMethods::ATTRIBUTE_HEADER_BYTES+bootstrapMethods.size()*AttBootstrapMethods::BOOTSTRAP_METHOD_BYTES;
        for(        let i: i32 = 0;;i<bootstrapMethods.size()++i)        {
            let numberOfArguments: i32 = bootstrapMethods.get(i).getBootstrapMethodArguments().size();
            bytes+=numberOfArguments*AttBootstrapMethods::BOOTSTRAP_ARGUMENT_BYTES;
        }
        self->byteLength=bytes;
    }
    pub fn byteLength(&self) -> i32    {
        return self.byteLength;
    }
    pub fn getBootstrapMethods(&self) -> BootstrapMethodsList    {
        return self.bootstrapMethods;
    }
}
