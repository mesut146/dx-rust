use crate::helper;
use crate::com::android::dex::DexFormat;

struct DexOptions{
    pub ALIGN_64BIT_REGS_IN_OUTPUT_FINISHER: boolean,
    pub minSdkVersion: i32,
    pub forceJumbo: boolean,
    pub allowAllInterfaceMethodInvokes: boolean,
    pub err: PrintStream,
}
impl DexOptions{
    pub const ALIGN_64BIT_REGS_SUPPORT: boolean = true;
    pub fn new(&self)    {
        self.err=System::err;
    }
    pub fn new(&self, stream: &PrintStream)    {
        self.err=stream;
    }
    pub fn getMagic(&self) -> String    {
        return DexFormat::apiToMagic(self.minSdkVersion);
    }
    pub fn apiIsSupported(&self, apiLevel: i32) -> boolean    {
        return self.minSdkVersion>=apiLevel;
    }
}
