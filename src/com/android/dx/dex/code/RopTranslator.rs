use crate::helper;
use crate::com::android::dx::dex::code::RopTranslator::LocalVariableAwareTranslationVisitor;
use crate::com::android::dx::dex::code::RopTranslator::TranslationVisitor;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::dex::code::RopTranslator;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::dex::code::StdCatchBuilder;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::dex::code::MultiCstInsn;
use crate::com::android::dx::rop::code::LocalVariableInfo;
use crate::com::android::dx::dex::DexOptions;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::rop::code::Insn::BaseVisitor;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dx::dex::code::RopToDop;
use crate::com::android::dx::rop::code::ThrowingInsn;
use crate::com::android::dx::dex::code::BlockAddresses;
use crate::com::android::dx::dex::code::ArrayData;
use crate::com::android::dx::rop::code::BasicBlock;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::dex::code::OutputCollector;
use crate::com::android::dx::rop::code::FillArrayDataInsn;
use crate::com::android::dx::rop::code::PlainCstInsn;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::dex::code::SimpleInsn;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::dex::code::LocalSnapshot;
use crate::com::android::dx::dex::code::TargetInsn;
use crate::com::android::dx::dex::code::OddSpacer;
use crate::com::android::dx::dex::code::LocalStart;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::code::SwitchData;
use crate::com::android::dx::rop::code::InvokePolymorphicInsn;
use crate::com::android::dx::rop::code::SwitchInsn;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::util::Bits;
use crate::com::android::dx::rop::code::PlainInsn;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::dex::code::Dops;

struct RopTranslator{
    pub dexOptions: DexOptions,
    pub method: RopMethod,
    pub positionInfo: i32,
    pub locals: LocalVariableInfo,
    pub addresses: BlockAddresses,
    pub output: OutputCollector,
    pub translationVisitor: TranslationVisitor,
    pub regCount: i32,
    pub order: Vec<i32>,
    pub paramSize: i32,
    pub paramsAreInOrder: boolean,
}
impl RopTranslator{
    pub fn translate(method: &RopMethod, positionInfo: i32, locals: &LocalVariableInfo, paramSize: i32, dexOptions: &DexOptions) -> DalvCode    {
        let translator: RopTranslator = RopTranslator::new(&method, positionInfo, &locals, paramSize, &dexOptions);
        return translator.translateAndGetResult();
    }
    pub fn new(&self, method: &RopMethod, positionInfo: i32, locals: &LocalVariableInfo, paramSize: i32, dexOptions: &DexOptions)    {
        self->dexOptions=dexOptions;
        self->method=method;
        self->positionInfo=positionInfo;
        self->locals=locals;
        self->addresses=BlockAddresses::new(&method);
        self->paramSize=paramSize;
        self->order=None;
        self->paramsAreInOrder=RopTranslator::calculateParamsAreInOrder(&method, paramSize);
        let blocks: BasicBlockList = method.getBlocks();
        let bsz: i32 = blocks.size();
        let maxInsns: i32 = (bsz*3)+blocks.getInstructionCount();
        if locals!=None        {
            maxInsns+=bsz+locals.getAssignmentCount();
        }        
        self->regCount=blocks.getRegCount()+(if self.paramsAreInOrder { 0 } else { self->paramSize });
                self->output=OutputCollector::new(&dexOptions, maxInsns, bsz*3, self.regCount, paramSize);
                if locals!=None                {
                    self->translationVisitor=LocalVariableAwareTranslationVisitor::new(&self.output, &locals, self);
                }                else                 {
                    self->translationVisitor=TranslationVisitor::new(&self.output, self);
                }
            }
            pub fn calculateParamsAreInOrder(method: &RopMethod, paramSize: i32) -> boolean            {
                let paramsAreInOrder: Vec<boolean> = vec![true];
                let initialRegCount: i32 = method.getBlocks().getRegCount();
                method.getBlocks().forEachInsn(/*new Insn.BaseVisitor(){
  @Override public void visitPlainCstInsn(  PlainCstInsn insn){
    if (insn.getOpcode().getOpcode() == RegOps.MOVE_PARAM) {
      int param=((CstInteger)insn.getConstant()).getValue();
      paramsAreInOrder[0]=paramsAreInOrder[0] && ((initialRegCount - paramSize + param) == insn.getResult().getReg());
    }
  }
}
*/);
                return paramsAreInOrder[0];
            }
            pub fn translateAndGetResult(&self) -> DalvCode            {
                pickOrder();
                outputInstructions();
                let catches: StdCatchBuilder = StdCatchBuilder::new(&self.method, &self.order, &self.addresses);
                return DalvCode::new(self.positionInfo, self.output.getFinisher(), &catches);
            }
            pub fn outputInstructions(&self)            {
                let blocks: BasicBlockList = self.method.getBlocks();
                let order: Vec<i32> = self->order;
                let len: i32 = order.len();
                for(                let i: i32 = 0;;i<leni += 1)                {
                    let nextI: i32 = i+1;
                    let nextLabel: i32 = if (nextI==order.len()) { -1 } else { order[nextI] };
                            outputBlock(blocks.labelToBlock(order[i]), nextLabel);
                        }
                    }
                    pub fn outputBlock(&self, block: &BasicBlock, nextLabel: i32)                    {
                        let startAddress: CodeAddress = self.addresses.getStart_BasicBlock(&block);
                        self.output.add(&startAddress);
                        if self.locals!=None                        {
                            let starts: RegisterSpecSet = self.locals.getStarts_BasicBlock(&block);
                            self.output.add(LocalSnapshot::new(startAddress.getPosition(), &starts));
                        }                        
                        self.translationVisitor.setBlock(&block, self.addresses.getLast_BasicBlock(&block));
                        block.getInsns().forEach(&self.translationVisitor);
                        self.output.add(self.addresses.getEnd_BasicBlock(&block));
                        let succ: i32 = block.getPrimarySuccessor();
                        let lastInsn: Insn = block.getLastInsn();
                        if (succ>=0)&&(succ!=nextLabel)                        {
                            let lastRop: Rop = lastInsn.getOpcode();
                            if (lastRop.getBranchingness()==Rop::BRANCH_IF)&&(block.getSecondarySuccessor()==nextLabel)                            {
                                self.output.reverseBranch(1, self.addresses.getStart_int(succ));
                            }                            else                             {
                                let insn: TargetInsn = TargetInsn::new(&Dops::GOTO, lastInsn.getPosition(), &RegisterSpecList::EMPTY, self.addresses.getStart_int(succ));
                                self.output.add(&insn);
                            }
                        }                        
                    }
                    pub fn pickOrder(&self)                    {
                        let blocks: BasicBlockList = self.method.getBlocks();
                        let sz: i32 = blocks.size();
                        let maxLabel: i32 = blocks.getMaxLabel();
                        let workSet: Vec<i32> = Bits::makeBitSet(maxLabel);
                        let tracebackSet: Vec<i32> = Bits::makeBitSet(maxLabel);
                        for(                        let i: i32 = 0;;i<szi += 1)                        {
                            let one: BasicBlock = blocks.get(i);
                            Bits::set_int[]_int(&workSet, one.getLabel());
                        }
                        let order: Vec<i32> = new int[sz];
                        let at: i32 = 0;
                        for(                        let label: i32 = self.method.getFirstLabel();;label!=-1label=Bits::findFirst_int[]_int(&workSet, 0))                        {
                            traceBack:
                            while label!=-1                            {
                                Bits::clear(&workSet, label);
                                Bits::clear(&tracebackSet, label);
                                order[at]=label;
                                at += 1;
                                let one: BasicBlock = blocks.labelToBlock(label);
                                let preferredBlock: BasicBlock = blocks.preferredSuccessorOf(&one);
                                if preferredBlock==None                                {
                                    break;
                                }                                
                                let preferred: i32 = preferredBlock.getLabel();
                                let primary: i32 = one.getPrimarySuccessor();
                                if Bits::get(&workSet, preferred)                                {
                                    label=preferred;
                                }                                else                                 if (primary!=preferred)&&(primary>=0)&&Bits::get(&workSet, primary)                                {
                                    label=primary;
                                }                                else                                 {
                                    let successors: IntList = one.getSuccessors();
                                    let ssz: i32 = successors.size();
                                    label=-1;
                                    for(                                    let i: i32 = 0;;i<sszi += 1)                                    {
                                        let candidate: i32 = successors.get(i);
                                        if Bits::get(&workSet, candidate)                                        {
                                            label=candidate;
                                            break;
                                        }                                        
                                    }
                                }
                            }
                        }
                        if at!=sz                        {
                            throw RuntimeException::new("shouldn't happen");
                        }                        
                        self->order=order;
                    }
                    pub fn getRegs(insn: &Insn) -> RegisterSpecList                    {
                        return RopTranslator::getRegs_Insn_RegisterSpec(&insn, insn.getResult());
                    }
                    pub fn getRegs(insn: &Insn, resultReg: &RegisterSpec) -> RegisterSpecList                    {
                        let regs: RegisterSpecList = insn.getSources();
                        if insn.getOpcode().isCommutative()&&(regs.size()==2)&&(resultReg.getReg()==regs.get(1).getReg())                        {
                            regs=RegisterSpecList::make_RegisterSpec_RegisterSpec(regs.get(1), regs.get(0));
                        }                        
                        if resultReg==None                        {
                            return regs;
                        }                        
                        return regs.withFirst(&resultReg);
                    }
}
