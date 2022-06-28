use crate::helper;
use crate::com::android::dx::rop::code::DexTranslationAdvice;
use crate::com::android::dx::cf::direct::StdAttributeFactory;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::cf::direct::CodeObserver;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::cf::code::ByteCatchList;
use crate::com::android::dx::cf::code::ByteBlock;
use crate::com::android::dx::command::dump::Args;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::cf::code::BytecodeArray;
use crate::com::android::dx::cf::iface::Member;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::command::dump::BlockDumper;
use crate::com::android::dx::cf::code::Ropper;
use crate::com::android::dx::cf::code::ByteBlockList;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::cf::code::ByteCatchList::Item;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::cf::code::BasicBlocker;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct BlockDumper{
    pub rop: boolean,
    pub classFile: DirectClassFile,
    pub suppressDump: boolean,
    pub first: boolean,
    pub optimize: boolean,
}
impl BlockDumper{
    pub fn dump(bytes: &Vec<i8>, out: &PrintStream, filePath: &String, rop: boolean, args: &Args)    {
        let bd: BlockDumper = BlockDumper::new(&bytes, &out, &filePath, rop, &args);
        bd.dump();
    }
    pub fn new(&self, bytes: &Vec<i8>, out: &PrintStream, filePath: &String, rop: boolean, args: &Args)    {
        super(bytes,out,filePath,args);

        self->rop=rop;
        self->classFile=None;
        self->suppressDump=true;
        self->first=true;
        self->optimize=;
    }
    pub fn dump(&self)    {
        let bytes: Vec<i8> = getBytes();
        let ba: ByteArray = ByteArray::new(&bytes);
        self.classFile=DirectClassFile::new(&ba, getFilePath(), getStrictParse());
        self.classFile.setAttributeFactory(&StdAttributeFactory::THE_ONE);
        self.classFile.getMagic();
        let liveCf: DirectClassFile = DirectClassFile::new(&ba, getFilePath(), getStrictParse());
        liveCf.setAttributeFactory(&StdAttributeFactory::THE_ONE);
        liveCf.setObserver(self);
        liveCf.getMagic();
    }
    pub fn changeIndent(&self, indentDelta: i32)    {
        if !self.suppressDump        {
            super.changeIndent(indentDelta);
        }        
    }
    pub fn parsed(&self, bytes: &ByteArray, offset: i32, len: i32, human: &String)    {
        if !self.suppressDump        {
            super.parsed(bytes,offset,len,human);
        }        
    }
    pub fn shouldDumpMethod(&self, name: &String) -> boolean    {
        return ==None||.equals(&name);
    }
    pub fn startParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String)    {
        if descriptor.indexOf_int('(')<0        {
            return;
        }        
        if !shouldDumpMethod(&name)        {
            return;
        }        
        self.suppressDump=false;
        if self.first        {
            self.first=false;
        }        else         {
            parsed(&bytes, offset, 0, "\n");
        }
        parsed(&bytes, offset, 0, "method "+name+" "+descriptor);
        self.suppressDump=true;
    }
    pub fn endParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String, member: &Member)    {
        if !(//member instanceof Method)        {
            return;
        }        
        if !shouldDumpMethod(&name)        {
            return;
        }        
        if (member.getAccessFlags()&(AccessFlags::ACC_ABSTRACT|AccessFlags::ACC_NATIVE))!=0        {
            return;
        }        
        let meth: ConcreteMethod = ConcreteMethod::new((Method*)member, &self.classFile, true, true);
        if self.rop        {
            ropDump(&meth);
        }        else         {
            regularDump(&meth);
        }
    }
    pub fn regularDump(&self, meth: &ConcreteMethod)    {
        let code: BytecodeArray = meth.getCode();
        let bytes: ByteArray = code.getBytes();
        let list: ByteBlockList = BasicBlocker::identifyBlocks(&meth);
        let sz: i32 = list.size();
        let codeObserver: CodeObserver = CodeObserver::new(&bytes, self.BlockDumper);
        self.suppressDump=false;
        let byteAt: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let bb: ByteBlock = list.get(i);
            let start: i32 = bb.getStart();
            let end: i32 = bb.getEnd();
            if byteAt<start            {
                parsed(&bytes, byteAt, start-byteAt, "dead code "+Hex::u2(byteAt)+".."+Hex::u2(start));
            }            
            parsed(&bytes, start, 0, "block "+Hex::u2(bb.getLabel())+": "+Hex::u2(start)+".."+Hex::u2(end));
            changeIndent(1);
            let len: i32;
            for(            let j: i32 = start;;j<endj+=len)            {
                len=code.parseInstruction(j, &codeObserver);
                codeObserver.setPreviousOffset(j);
            }
            let successors: IntList = bb.getSuccessors();
            let ssz: i32 = successors.size();
            if ssz==0            {
                parsed(&bytes, end, 0, "returns");
            }            else             {
                for(                let j: i32 = 0;;j<sszj += 1)                {
                    let succ: i32 = successors.get(j);
                    parsed(&bytes, end, 0, "next "+Hex::u2(succ));
                }
            }
            let catches: ByteCatchList = bb.getCatches();
            let csz: i32 = catches.size();
            for(            let j: i32 = 0;;j<cszj += 1)            {
                let one: Item = catches.get(j);
                let exceptionClass: CstType = one.getExceptionClass();
                parsed(&bytes, end, 0, "catch "+(if (exceptionClass==CstType::OBJECT) { "<any>" } else { exceptionClass.toHuman() })+" -> "+Hex::u2(one.getHandlerPc()));
                    }
                    changeIndent(-1);
                    byteAt=end;
                }
                let end: i32 = bytes.size();
                if byteAt<end                {
                    parsed(&bytes, byteAt, end-byteAt, "dead code "+Hex::u2(byteAt)+".."+Hex::u2(end));
                }                
                self.suppressDump=true;
            }
            pub fn ropDump(&self, meth: &ConcreteMethod)            {
                let advice: TranslationAdvice = DexTranslationAdvice::THE_ONE;
                let code: BytecodeArray = meth.getCode();
                let bytes: ByteArray = code.getBytes();
                let rmeth: RopMethod = Ropper::convert(&meth, &advice, self.classFile.getMethods(), &self.super_.dexOptions);
                let sb: StringBuilder = StringBuilder::new(2000);
                if self.optimize                {
                    let isStatic: boolean = AccessFlags::isStatic(meth.getAccessFlags());
                    let paramWidth: i32 = BaseDumper::computeParamWidth(&meth, isStatic);
                    rmeth=Optimizer::optimize_RopMethod_int_boolean_boolean_TranslationAdvice(&rmeth, paramWidth, isStatic, true, &advice);
                }                
                let blocks: BasicBlockList = rmeth.getBlocks();
                let order: Vec<i32> = blocks.getLabelsInOrder();
                sb.append_String("first "+Hex::u2(rmeth.getFirstLabel())+"\n");
                for label in order                {
                    let bb: BasicBlock = blocks.get(blocks.indexOfLabel(label));
                    sb.append_String("block ");
                    sb.append_String(Hex::u2(label));
                    sb.append_String("\n");
                    let preds: IntList = rmeth.labelToPredecessors(label);
                    let psz: i32 = preds.size();
                    for(                    let i: i32 = 0;;i<pszi += 1)                    {
                        sb.append_String("  pred ");
                        sb.append_String(Hex::u2(preds.get(i)));
                        sb.append_String("\n");
                    }
                    let il: InsnList = bb.getInsns();
                    let ilsz: i32 = il.size();
                    for(                    let i: i32 = 0;;i<ilszi += 1)                    {
                        let one: Insn = il.get(i);
                        sb.append_String("  ");
                        sb.append_String(il.get(i).toHuman());
                        sb.append_String("\n");
                    }
                    let successors: IntList = bb.getSuccessors();
                    let ssz: i32 = successors.size();
                    if ssz==0                    {
                        sb.append_String("  returns\n");
                    }                    else                     {
                        let primary: i32 = bb.getPrimarySuccessor();
                        for(                        let i: i32 = 0;;i<sszi += 1)                        {
                            let succ: i32 = successors.get(i);
                            sb.append_String("  next ");
                            sb.append_String(Hex::u2(succ));
                            if (ssz!=1)&&(succ==primary)                            {
                                sb.append_String(" *");
                            }                            
                            sb.append_String("\n");
                        }
                    }
                }
                self.suppressDump=false;
                parsed(&bytes, 0, bytes.size(), sb.toString());
                self.suppressDump=true;
            }
}
