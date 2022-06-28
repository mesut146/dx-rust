use crate::helper;
use crate::com::android::dx::util::IndentingWriter;
use crate::com::android::dx::util::TwoColumnOutput;

struct TwoColumnOutput{
    pub out: Writer,
    pub leftWidth: i32,
    pub leftBuf: StringBuffer,
    pub rightBuf: StringBuffer,
    pub leftColumn: IndentingWriter,
    pub rightColumn: IndentingWriter,
}
impl TwoColumnOutput{
    pub fn toString(s1: &String, width1: i32, spacer: &String, s2: &String, width2: i32) -> String    {
        let len1: i32 = s1.length();
        let len2: i32 = s2.length();
        let sw: StringWriter = StringWriter::new((len1+len2)*3);
        let twoOut: TwoColumnOutput = TwoColumnOutput::new(&sw, width1, width2, &spacer);
        try        {
            twoOut.getLeft().write_String(&s1);
            twoOut.getRight().write_String(&s2);
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new("shouldn't happen", &ex);
        }
        twoOut.flush();
        return sw.toString();
    }
    pub fn new(&self, out: &Writer, leftWidth: i32, rightWidth: i32, spacer: &String)    {
        if out==None        {
            throw NullPointerException::new("out == null");
        }        
        if leftWidth<1        {
            throw IllegalArgumentException::new("leftWidth < 1");
        }        
        if rightWidth<1        {
            throw IllegalArgumentException::new("rightWidth < 1");
        }        
        if spacer==None        {
            throw NullPointerException::new("spacer == null");
        }        
        let leftWriter: StringWriter = StringWriter::new(1000);
        let rightWriter: StringWriter = StringWriter::new(1000);
        self->out=out;
        self->leftWidth=leftWidth;
        self->leftBuf=leftWriter.getBuffer();
        self->rightBuf=rightWriter.getBuffer();
        self->leftColumn=IndentingWriter::new(&leftWriter, leftWidth);
        self->rightColumn=IndentingWriter::new(&rightWriter, rightWidth, &spacer);
    }
    pub fn new(&self, out: &OutputStream, leftWidth: i32, rightWidth: i32, spacer: &String)    {
        this(new OutputStreamWriter(out),leftWidth,rightWidth,spacer);

    }
    pub fn getLeft(&self) -> Writer    {
        return self.leftColumn;
    }
    pub fn getRight(&self) -> Writer    {
        return self.rightColumn;
    }
    pub fn flush(&self)    {
        try        {
            TwoColumnOutput::appendNewlineIfNecessary(&self.leftBuf, &self.leftColumn);
            TwoColumnOutput::appendNewlineIfNecessary(&self.rightBuf, &self.rightColumn);
            outputFullLines();
            flushLeft();
            flushRight();
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new(&ex);
        }
    }
    pub fn outputFullLines(&self)    {
        for(;)        {
            let leftLen: i32 = self.leftBuf.indexOf_String("\n");
            if leftLen<0            {
                return;
            }            
            let rightLen: i32 = self.rightBuf.indexOf_String("\n");
            if rightLen<0            {
                return;
            }            
            if leftLen!=0            {
                self.out.write_String(self.leftBuf.substring_int_int(0, leftLen));
            }            
            if rightLen!=0            {
                TwoColumnOutput::writeSpaces(&self.out, self.leftWidth-leftLen);
                self.out.write_String(self.rightBuf.substring_int_int(0, rightLen));
            }            
            self.out.write_int('\n');
            self.leftBuf.delete(0, leftLen+1);
            self.rightBuf.delete(0, rightLen+1);
        }
    }
    pub fn flushLeft(&self)    {
        TwoColumnOutput::appendNewlineIfNecessary(&self.leftBuf, &self.leftColumn);
        while self.leftBuf.length()!=0        {
            self.rightColumn.write_int('\n');
            outputFullLines();
        }
    }
    pub fn flushRight(&self)    {
        TwoColumnOutput::appendNewlineIfNecessary(&self.rightBuf, &self.rightColumn);
        while self.rightBuf.length()!=0        {
            self.leftColumn.write_int('\n');
            outputFullLines();
        }
    }
    pub fn appendNewlineIfNecessary(buf: &StringBuffer, out: &Writer)    {
        let len: i32 = buf.length();
        if (len!=0)&&(buf.charAt(len-1)!='\n')        {
            out.write_int('\n');
        }        
    }
    pub fn writeSpaces(out: &Writer, amt: i32)    {
        while amt>0        {
            out.write_int(' ');
            amt -= 1;
        }
    }
}
