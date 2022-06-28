use crate::helper;
use crate::com::android::dx::command::annotool::Main::InvalidArgumentException;
use crate::com::android::dx::command::annotool::Main::Arguments;
use crate::com::android::dx::command::annotool::AnnotationLister;
use crate::com::android::dx::command::annotool::Main::PrintType;

struct Main{
}
impl Main{
    pub fn new(&self)    {
    }
    pub fn main(argArray: &Vec<String>)    {
        let args: Arguments = Arguments::new();
        try        {
            args.parse(&argArray);
        }        catch(        let ex: InvalidArgumentException)        {
            System::err.println_String(ex.getMessage());
            throw RuntimeException::new("usage");
        }
        AnnotationLister::new(&args).process();
    }
}
