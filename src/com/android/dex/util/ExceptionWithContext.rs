use crate::helper;
use crate::com::android::dex::util::ExceptionWithContext;

struct ExceptionWithContext{
    pub context: StringBuffer,
}
impl ExceptionWithContext{
    pub fn withContext(ex: &Throwable, str: &String) -> ExceptionWithContext    {
        let ewc: ExceptionWithContext;
        if //ex instanceof ExceptionWithContext        {
            ewc=(ExceptionWithContext*)ex;
        }        else         {
            ewc=ExceptionWithContext::new(&ex);
        }
        ewc.addContext(&str);
        return ewc;
    }
    pub fn new(&self, message: &String)    {
        this(message,null);

    }
    pub fn new(&self, cause: &Throwable)    {
        this(null,cause);

    }
    pub fn new(&self, message: &String, cause: &Throwable)    {
        super((message != null) ? message : (cause != null) ? cause.getMessage() : null,cause);

        if //cause instanceof ExceptionWithContext        {
            let ctx: String = ((ExceptionWithContext*)cause)->context.toString();
            self.context=StringBuffer::new(ctx.length()+200);
            self.context.append_String(&ctx);
        }        else         {
            self.context=StringBuffer::new(200);
        }
    }
    pub fn printStackTrace(&self, out: &PrintStream)    {
        super.printStackTrace(out);
        out.println_Object(&self.context);
    }
    pub fn printStackTrace(&self, out: &PrintWriter)    {
        super.printStackTrace(out);
        out.println_Object(&self.context);
    }
    pub fn addContext(&self, str: &String)    {
        if str==None        {
            throw NullPointerException::new("str == null");
        }        
        self.context.append_String(&str);
        if !str.endsWith("\n")        {
            self.context.append_char('\n');
        }        
    }
    pub fn getContext(&self) -> String    {
        return self.context.toString();
    }
    pub fn printContext(&self, out: &PrintStream)    {
        out.println_String(getMessage());
        out.print_Object(&self.context);
    }
    pub fn printContext(&self, out: &PrintWriter)    {
        out.println_String(getMessage());
        out.print_Object(&self.context);
    }
}
