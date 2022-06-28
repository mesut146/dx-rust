use crate::helper;
use crate::com::android::dx::command::UsageException;
use crate::com::android::dx::Version;
use crate::com::android::dx::command::annotool::Main;
use crate::com::android::dx::command::findusages::Main;
use crate::com::android::dx::command::dump::Main;
use crate::com::android::dx::command::dexer::Main;

let static USAGE_MESSAGE: String = "usage:\n"+"  dx --dex [--debug] [--verbose] [--positions=<style>] [--no-locals]\n"+"  [--no-optimize] [--statistics] [--[no-]optimize-list=<file>] [--no-strict]\n"+"  [--keep-classes] [--output=<file>] [--dump-to=<file>] [--dump-width=<n>]\n"+"  [--dump-method=<name>[*]] [--verbose-dump] [--no-files] [--core-library]\n"+"  [--num-threads=<n>] [--incremental] [--force-jumbo] [--no-warning]\n"+"  [--multi-dex [--main-dex-list=<file> [--minimal-main-dex]]\n"+"  [--input-list=<file>] [--min-sdk-version=<n>]\n"+"  [--allow-all-interface-method-invokes]\n"+"  [<file>.class | <file>.{zip,jar,apk} | <directory>] ...\n"+"    Convert a set of classfiles into a dex file, optionally embedded in a\n"+"    jar/zip. Output name must end with one of: .dex .jar .zip .apk or be a\n"+"    directory.\n"+"    Positions options: none, important, lines.\n"+"    --multi-dex: allows to generate several dex files if needed. This option is\n"+"    exclusive with --incremental, causes --num-threads to be ignored and only\n"+"    supports folder or archive output.\n"+"    --main-dex-list=<file>: <file> is a list of class file names, classes\n"+"    defined by those class files are put in classes.dex.\n"+"    --minimal-main-dex: only classes selected by --main-dex-list are to be put\n"+"    in the main dex.\n"+"    --input-list: <file> is a list of inputs.\n"+"    Each line in <file> must end with one of: .class .jar .zip .apk or be a\n"+"    directory.\n"+"    --min-sdk-version=<n>: Enable dex file features that require at least sdk\n"+"    version <n>.\n"+"  dx --annotool --annotation=<class> [--element=<element types>]\n"+"  [--print=<print types>]\n"+"  dx --dump [--debug] [--strict] [--bytes] [--optimize]\n"+"  [--basic-blocks | --rop-blocks | --ssa-blocks | --dot] [--ssa-step=<step>]\n"+"  [--width=<n>] [<file>.class | <file>.txt] ...\n"+"    Dump classfiles, or transformations thereof, in a human-oriented format.\n"+"  dx --find-usages <file.dex> <declaring type> <member>\n"+"    Find references and declarations to a field or method.\n"+"    <declaring type> is a class name in internal form, like Ljava/lang/Object;\n"+"    <member> is a field or method name, like hashCode.\n"+"  dx -J<option> ... <arguments, in one of the above forms>\n"+"    Pass VM-specific options to the virtual machine that runs dx.\n"+"  dx --version\n"+"    Print the version of this tool ("+Version::VERSION+").\n"+"  dx --help\n"+"    Print this message.";
struct Main{
}
impl Main{
    pub fn new(&self)    {
    }
    pub fn main(args: &Vec<String>) -> i32    {
        let gotCmd: boolean = false;
        let showUsage: boolean = false;
        let result: i32 = -1;
        try        {
            for(            let i: i32 = 0;;i<args.len()i += 1)            {
                let arg: String = args[i];
                if arg.equals("--")||!arg.startsWith_String("--")                {
                    gotCmd=false;
                    showUsage=true;
                    break;
                }                
                gotCmd=true;
                if arg.equals("--dex")                {
                    result=Main::main(Main::without(&args, i));
                    break;
                }                else                 if arg.equals("--dump")                {
                    Main::main(Main::without(&args, i));
                    break;
                }                else                 if arg.equals("--annotool")                {
                    Main::main(Main::without(&args, i));
                    break;
                }                else                 if arg.equals("--find-usages")                {
                    Main::main(Main::without(&args, i));
                    break;
                }                else                 if arg.equals("--version")                {
                    Main::version();
                    break;
                }                else                 if arg.equals("--help")                {
                    showUsage=true;
                    break;
                }                else                 {
                    gotCmd=false;
                }
            }
        }        catch(        let ex: UsageException)        {
            showUsage=true;
        }        catch(        let ex: RuntimeException)        {
            System::err.println_String("\nUNEXPECTED TOP-LEVEL EXCEPTION:");
            ex.printStackTrace();
            return 2;
        }        catch(        let ex: Throwable)        {
            System::err.println_String("\nUNEXPECTED TOP-LEVEL ERROR:");
            ex.printStackTrace();
            if (//ex instanceof NoClassDefFoundError)||(//ex instanceof NoSuchMethodError)            {
                System::err.println_String("Note: You may be using an incompatible "+"virtual machine or class library.\n"+"(This program is known to be incompatible "+"with recent releases of GCJ.)");
            }            
            return 3;
        }
        if !gotCmd        {
            System::err.println_String("error: no command specified");
            showUsage=true;
        }        
        if showUsage        {
            Main::usage();
            return 1;
        }        
        return result;
    }
    pub fn version()    {
        System::err.println_String("dx version "+Version::VERSION);
        System::exit(0);
    }
    pub fn usage()    {
        System::err.println_String(&Main::USAGE_MESSAGE);
    }
    pub fn without(orig: &Vec<String>, n: i32) -> Vec<String>    {
        let len: i32 = orig.len()-1;
        let newa: Vec<String> = new String[len];
        System::arraycopy(&orig, 0, &newa, 0, n);
        System::arraycopy(&orig, n+1, &newa, n, len-n);
        return newa;
    }
}
