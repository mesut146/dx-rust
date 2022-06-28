use crate::helper;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::util::TwoColumnOutput;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::command::dump::Args;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::util::IndentingWriter;

struct BaseDumper{
    pub bytes: Vec<i8>,
    pub rawBytes: boolean,
    pub out: PrintStream,
    pub width: i32,
    pub filePath: String,
    pub strictParse: boolean,
    pub hexCols: i32,
    pub indent: i32,
    pub separator: String,
    pub readBytes: i32,
    pub args: Args,
    pub dexOptions: DexOptions,
}
impl BaseDumper{
    pub fn new(&self, bytes: &Vec<i8>, out: &PrintStream, filePath: &String, args: &Args)    {
        self->bytes=bytes;
        self->rawBytes=;
        self->out=out;
        self->width=if (<=0) { 79 } else {  };
                self->filePath=filePath;
                self->strictParse=;
                self->indent=0;
                self->separator=if self.rawBytes { "|" } else { "" };
                        self->readBytes=0;
                        self->args=args;
                        self->dexOptions=DexOptions::new();
                        let hexCols: i32 = (((self.width-5)/15)+1)&~1;
                        if hexCols<6                        {
                            hexCols=6;
                        }                        else                         if hexCols>10                        {
                            hexCols=10;
                        }                        
                        self->hexCols=hexCols;
                    }
                    pub fn computeParamWidth(meth: &ConcreteMethod, isStatic: boolean) -> i32                    {
                        return meth.getEffectiveDescriptor().getParameterTypes().getWordCount();
                    }
                    pub fn changeIndent(&self, indentDelta: i32)                    {
                        self.indent+=indentDelta;
                        self.separator=if self.rawBytes { "|" } else { "" };
                                for(                                let i: i32 = 0;;i<self.indenti += 1)                                {
                                    self.separator+="  ";
                                }
                            }
                            pub fn parsed(&self, bytes: &ByteArray, offset: i32, len: i32, human: &String)                            {
                                offset=bytes.underlyingOffset(offset);
                                let rawBytes: boolean = getRawBytes();
                                let hex: String = if rawBytes { hexDump(offset, len) } else { "" };
                                        print(twoColumns(&hex, &human));
                                        self.readBytes+=len;
                                    }
                                    pub fn startParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String)                                    {
                                    }
                                    pub fn endParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String, member: &Member)                                    {
                                    }
                                    pub fn getReadBytes(&self) -> i32                                    {
                                        return self.readBytes;
                                    }
                                    pub fn getBytes(&self) -> Vec<i8>                                    {
                                        return self.bytes;
                                    }
                                    pub fn getFilePath(&self) -> String                                    {
                                        return self.filePath;
                                    }
                                    pub fn getStrictParse(&self) -> boolean                                    {
                                        return self.strictParse;
                                    }
                                    pub fn print(&self, s: &String)                                    {
                                        self.out.print_String(&s);
                                    }
                                    pub fn println(&self, s: &String)                                    {
                                        self.out.println_String(&s);
                                    }
                                    pub fn getRawBytes(&self) -> boolean                                    {
                                        return self.rawBytes;
                                    }
                                    pub fn getWidth1(&self) -> i32                                    {
                                        if self.rawBytes                                        {
                                            return 5+(self.hexCols*2)+(self.hexCols/2);
                                        }                                        
                                        return 0;
                                    }
                                    pub fn getWidth2(&self) -> i32                                    {
                                        let w1: i32 = if self.rawBytes { (getWidth1()+1) } else { 0 };
                                                return self.width-w1-(self.indent*2);
                                            }
                                            pub fn hexDump(&self, offset: i32, len: i32) -> String                                            {
                                                return Hex::dump(&self.bytes, offset, len, offset, self.hexCols, 4);
                                            }
                                            pub fn twoColumns(&self, s1: &String, s2: &String) -> String                                            {
                                                let w1: i32 = getWidth1();
                                                let w2: i32 = getWidth2();
                                                try                                                {
                                                    if w1==0                                                    {
                                                        let len2: i32 = s2.length();
                                                        let sw: StringWriter = StringWriter::new(len2*2);
                                                        let iw: IndentingWriter = IndentingWriter::new(&sw, w2, &self.separator);
                                                        iw.write_String(&s2);
                                                        if (len2==0)||(s2.charAt(len2-1)!='\n')                                                        {
                                                            iw.write_int('\n');
                                                        }                                                        
                                                        iw.flush();
                                                        return sw.toString();
                                                    }                                                    else                                                     {
                                                        return TwoColumnOutput::toString(&s1, w1, &self.separator, &s2, w2);
                                                    }
                                                }                                                catch(                                                let ex: IOException)                                                {
                                                    throw RuntimeException::new(&ex);
                                                }
                                            }
}
