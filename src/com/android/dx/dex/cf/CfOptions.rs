use crate::helper;
use crate::com::android::dx::dex::code::PositionList;

struct CfOptions{
    pub positionInfo: i32,
    pub localInfo: boolean,
    pub strictNameCheck: boolean,
    pub optimize: boolean,
    pub optimizeListFile: String,
    pub dontOptimizeListFile: String,
    pub statistics: boolean,
    pub warn: PrintStream,
}
impl CfOptions{
}
