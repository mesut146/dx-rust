use crate::helper;
use crate::com::android::dx::command::dump::DotDumper;
use crate::com::android::dx::rop::code::DexTranslationAdvice;
use crate::com::android::dx::cf::code::Ropper;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::command::dump::Args;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::command::dump::BaseDumper;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct DotDumper{
    pub classFile: DirectClassFile,
    pub bytes: Vec<i8>,
    pub filePath: String,
    pub strictParse: boolean,
    pub optimize: boolean,
    pub args: Args,
    pub dexOptions: DexOptions,
}
impl DotDumper{
    pub fn dump(bytes: &Vec<i8>, filePath: &String, args: &Args)    {
        DotDumper::new(&bytes, &filePath, &args).run();
    }
    pub fn new(&self, bytes: &Vec<i8>, filePath: &String, args: &Args)    {
        self->bytes=bytes;
        self->filePath=filePath;
        self->strictParse=;
        self->optimize=;
        self->args=args;
        self->dexOptions=DexOptions::new();
    }
    pub fn run(&self)    {
        let ba: ByteArray = ByteArray::new(&self.bytes);
        self.classFile=DirectClassFile::new(&ba, &self.filePath, self.strictParse);
        self.classFile.setAttributeFactory(&StdAttributeFactory::THE_ONE);
        self.classFile.getMagic();
        let liveCf: DirectClassFile = DirectClassFile::new(&ba, &self.filePath, self.strictParse);
        liveCf.setAttributeFactory(&StdAttributeFactory::THE_ONE);
        liveCf.setObserver(self);
        liveCf.getMagic();
    }
    pub fn shouldDumpMethod(&self, name: &String) -> boolean    {
        return ==None||.equals(&name);
    }
    pub fn changeIndent(&self, indentDelta: i32)    {
    }
    pub fn parsed(&self, bytes: &ByteArray, offset: i32, len: i32, human: &String)    {
    }
    pub fn startParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String)    {
    }
    pub fn endParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String, member: &Member)    {
        if !(//member instanceof Method)        {
            return;
        }        
        if !shouldDumpMethod(&name)        {
            return;
        }        
        let meth: ConcreteMethod = ConcreteMethod::new((Method*)member, &self.classFile, true, true);
        let advice: TranslationAdvice = DexTranslationAdvice::THE_ONE;
        let rmeth: RopMethod = Ropper::convert(&meth, &advice, self.classFile.getMethods(), &self.dexOptions);
        if self.optimize        {
            let isStatic: boolean = AccessFlags::isStatic(meth.getAccessFlags());
            rmeth=Optimizer::optimize_RopMethod_int_boolean_boolean_TranslationAdvice(&rmeth, BaseDumper::computeParamWidth(&meth, isStatic), isStatic, true, &advice);
        }        
        System::out.println_String("digraph "+name+"{");
        System::out.println_String("\tfirst -> n"+Hex::u2(rmeth.getFirstLabel())+";");
        let blocks: BasicBlockList = rmeth.getBlocks();
        let sz: i32 = blocks.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let bb: BasicBlock = blocks.get(i);
            let label: i32 = bb.getLabel();
            let successors: IntList = bb.getSuccessors();
            if successors.size()==0            {
                System::out.println_String("\tn"+Hex::u2(label)+" -> returns;");
            }            else             if successors.size()==1            {
                System::out.println_String("\tn"+Hex::u2(label)+" -> n"+Hex::u2(successors.get(0))+";");
            }            else             {
                System::out.print_String("\tn"+Hex::u2(label)+" -> {");
                for(                let j: i32 = 0;;j<successors.size()j += 1)                {
                    let successor: i32 = successors.get(j);
                    if successor!=bb.getPrimarySuccessor()                    {
                        System::out.print_String(" n"+Hex::u2(successor)+" ");
                    }                    
                }
                System::out.println_String("};");
                System::out.println_String("\tn"+Hex::u2(label)+" -> n"+Hex::u2(bb.getPrimarySuccessor())+" [label=\"primary\"];");
            }
        }
        System::out.println_String("}");
    }
}
