use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dx::command::grep::Grep;

struct Main{
}
impl Main{
    pub fn main(args: &Vec<String>)    {
        let dexFile: String = args[0];
        let pattern: String = args[1];
        let dex: Dex = Dex::new(File::new(&dexFile));
        let count: i32 = Grep::new(&dex, Pattern::compile_String(&pattern), PrintWriter::new(&System::out)).grep();
        System::exit(if (count>0) { 0 } else { 1 });
            }
}
