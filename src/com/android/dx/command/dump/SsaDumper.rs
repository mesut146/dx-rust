use crate::helper;
use crate::com::android::dx::rop::code::DexTranslationAdvice;
use crate::com::android::dx::cf::code::Ropper;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::ssa::SsaBasicBlock;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::ssa::SsaMethod;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::command::dump::Args;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::cf::iface::Member;
use crate::com::android::dx::ssa::SsaInsn;
use crate::com::android::dx::command::dump::SsaDumper;
use crate::com::android::dx::cf::direct::DirectClassFile;

struct SsaDumper{
}
impl SsaDumper{
    pub fn dump(bytes: &Vec<i8>, out: &PrintStream, filePath: &String, args: &Args)    {
        let sd: SsaDumper = SsaDumper::new(&bytes, &out, &filePath, &args);
        sd.dump();
    }
    pub fn new(&self, bytes: &Vec<i8>, out: &PrintStream, filePath: &String, args: &Args)    {
        super(bytes,out,filePath,true,args);

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
        let meth: ConcreteMethod = ConcreteMethod::new((Method*)member, &self.super_.classFile, true, true);
        let advice: TranslationAdvice = DexTranslationAdvice::THE_ONE;
        let rmeth: RopMethod = Ropper::convert(&meth, &advice, self.super_.classFile.getMethods(), &self.super_.dexOptions);
        let ssaMeth: SsaMethod = None;
        let isStatic: boolean = AccessFlags::isStatic(meth.getAccessFlags());
        let paramWidth: i32 = BaseDumper::computeParamWidth(&meth, isStatic);
        if ==None        {
            ssaMeth=Optimizer::debugNoRegisterAllocation(&rmeth, paramWidth, isStatic, true, &advice, EnumSet::allOf(Optimizer.OptionalStep.classOptimizer::OptionalStep));
        }        else         if "edge-split".equals(&)        {
            ssaMeth=Optimizer::debugEdgeSplit(&rmeth, paramWidth, isStatic, true, &advice);
        }        else         if "phi-placement".equals(&)        {
            ssaMeth=Optimizer::debugPhiPlacement(&rmeth, paramWidth, isStatic, true, &advice);
        }        else         if "renaming".equals(&)        {
            ssaMeth=Optimizer::debugRenaming(&rmeth, paramWidth, isStatic, true, &advice);
        }        else         if "dead-code".equals(&)        {
            ssaMeth=Optimizer::debugDeadCodeRemover(&rmeth, paramWidth, isStatic, true, &advice);
        }        
        let sb: StringBuilder = StringBuilder::new(2000);
        sb.append_String("first ");
        sb.append_String(Hex::u2(ssaMeth.blockIndexToRopLabel(ssaMeth.getEntryBlockIndex())));
        sb.append_char('\n');
        let blocks: ArrayList<SsaBasicBlock> = ssaMeth.getBlocks();
        let sortedBlocks: ArrayList<SsaBasicBlock> = (ArrayList<SsaBasicBlock>*)blocks.clone();
        Collections::sort_List<SsaBasicBlock>_Comparator<? super SsaBasicBlock>(&sortedBlocks, &SsaBasicBlock::LABEL_COMPARATOR);
        for block in sortedBlocks        {
            sb.append_String("block ").append_String(Hex::u2(block.getRopLabel())).append_char('\n');
            let preds: BitSet = block.getPredecessors();
            for(            let i: i32 = preds.nextSetBit_int(0);;i>=0i=preds.nextSetBit_int(i+1))            {
                sb.append_String("  pred ");
                sb.append_String(Hex::u2(ssaMeth.blockIndexToRopLabel(i)));
                sb.append_char('\n');
            }
            sb.append_String("  live in:"+block.getLiveInRegs());
            sb.append_String("\n");
            for insn in block.getInsns()            {
                sb.append_String("  ");
                sb.append_String(insn.toHuman());
                sb.append_char('\n');
            }
            if block.getSuccessors().cardinality()==0            {
                sb.append_String("  returns\n");
            }            else             {
                let primary: i32 = block.getPrimarySuccessorRopLabel();
                let succLabelList: IntList = block.getRopLabelSuccessorList();
                let szSuccLabels: i32 = succLabelList.size();
                for(                let i: i32 = 0;;i<szSuccLabelsi += 1)                {
                    sb.append_String("  next ");
                    sb.append_String(Hex::u2(succLabelList.get(i)));
                    if szSuccLabels!=1&&primary==succLabelList.get(i)                    {
                        sb.append_String(" *");
                    }                    
                    sb.append_char('\n');
                }
            }
            sb.append_String("  live out:"+block.getLiveOutRegs());
            sb.append_String("\n");
        }
        self.super_.suppressDump=false;
        parsed(&bytes, 0, bytes.size(), sb.toString());
        self.super_.suppressDump=true;
    }
}
