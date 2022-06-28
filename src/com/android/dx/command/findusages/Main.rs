use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dx::command::findusages::FindUsages;

struct Main{
}
impl Main{
    pub fn main(args: &Vec<String>)    {
        let dexFile: String = args[0];
        let declaredBy: String = args[1];
        let memberName: String = args[2];
        let dex: Dex = Dex::new(File::new(&dexFile));
        let out: PrintWriter = PrintWriter::new(&System::out);
        FindUsages::new(&dex, &declaredBy, &memberName, &out).findUsages();
        out.flush();
    }
}
