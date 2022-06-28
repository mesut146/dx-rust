use crate::helper;
use crate::com::android::dx::command::dump::DotDumper;
use crate::com::android::dex::util::FileUtils;
use crate::com::android::dx::util::HexParser;
use crate::com::android::dx::command::dump::Args;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::command::dump::Main;
use crate::com::android::dx::command::dump::SsaDumper;
use crate::com::android::dx::command::dump::BlockDumper;
use crate::com::android::dx::command::dump::ClassDumper;

struct Main{
    pub parsedArgs: Args,
}
impl Main{
    pub fn new(&self)    {
    }
    pub fn main(args: &Vec<String>)    {
        Main::new().run(&args);
    }
    pub fn run(&self, args: &Vec<String>)    {
        let at: i32 = 0;
        for(;at<args.len()at += 1)        {
            let arg: String = args[at];
            if arg.equals("--")||!arg.startsWith_String("--")            {
                break;
            }            else             if arg.equals("--bytes")            {
                =true;
            }            else             if arg.equals("--basic-blocks")            {
                =true;
            }            else             if arg.equals("--rop-blocks")            {
                =true;
            }            else             if arg.equals("--optimize")            {
                =true;
            }            else             if arg.equals("--ssa-blocks")            {
                =true;
            }            else             if arg.startsWith_String("--ssa-step=")            {
                =arg.substring_int(arg.indexOf_int('=')+1);
            }            else             if arg.equals("--debug")            {
                =true;
            }            else             if arg.equals("--dot")            {
                =true;
            }            else             if arg.equals("--strict")            {
                =true;
            }            else             if arg.startsWith_String("--width=")            {
                arg=arg.substring_int(arg.indexOf_int('=')+1);
                =Integer::parseInt_String(&arg);
            }            else             if arg.startsWith_String("--method=")            {
                arg=arg.substring_int(arg.indexOf_int('=')+1);
                =arg;
            }            else             {
                System::err.println_String("unknown option: "+arg);
                throw RuntimeException::new("usage");
            }
        }
        if at==args.len()        {
            System::err.println_String("no input files specified");
            throw RuntimeException::new("usage");
        }        
        for(;at<args.len()at += 1)        {
            try            {
                let name: String = args[at];
                System::out.println_String("reading "+name+"...");
                let bytes: Vec<i8> = FileUtils::readFile_String(&name);
                if !name.endsWith(".class")                {
                    let src: String;
                    try                    {
                        src=String::new(&bytes, "utf-8");
                    }                    catch(                    let ex: UnsupportedEncodingException)                    {
                        throw RuntimeException::new("shouldn't happen", &ex);
                    }
                    bytes=HexParser::parse(&src);
                }                
                processOne(&name, &bytes);
            }            catch(            let ex: ParseException)            {
                System::err.println_String("\ntrouble parsing:");
                if                 {
                    ex.printStackTrace();
                }                else                 {
                    ex.printContext_PrintStream(&System::err);
                }
            }
        }
    }
    pub fn processOne(&self, name: &String, bytes: &Vec<i8>)    {
        if         {
            DotDumper::dump(&bytes, &name, &self.parsedArgs);
        }        else         if         {
            BlockDumper::dump_byte[]_PrintStream_String_boolean_Args(&bytes, &System::out, &name, false, &self.parsedArgs);
        }        else         if         {
            BlockDumper::dump_byte[]_PrintStream_String_boolean_Args(&bytes, &System::out, &name, true, &self.parsedArgs);
        }        else         if         {
            =false;
            SsaDumper::dump(&bytes, &System::out, &name, &self.parsedArgs);
        }        else         {
            ClassDumper::dump_byte[]_PrintStream_String_Args(&bytes, &System::out, &name, &self.parsedArgs);
        }
    }
}
