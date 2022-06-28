use crate::helper;
use crate::com::android::dx::dex::cf::OptimizerOptions;
use crate::com::android::dx::dex::cf::CodeStatistics;

struct DxContext{
    pub codeStatistics: CodeStatistics,
    pub optimizerOptions: OptimizerOptions,
    pub out: PrintStream,
    pub err: PrintStream,
    pub noop: PrintStream,
}
impl DxContext{
    pub fn new(&self, out: &OutputStream, err: &OutputStream)    {
        self->out=PrintStream::new(&out);
        self->err=PrintStream::new(&err);
    }
    pub fn new(&self)    {
        this(System.out,System.err);

    }
}
