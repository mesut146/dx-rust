use crate::helper;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::InsnList;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::rop::code::RopMethod;
use crate::com::android::dx::rop::code::BasicBlock;

struct BlockAddresses{
    pub starts: Vec<CodeAddress>,
    pub lasts: Vec<CodeAddress>,
    pub ends: Vec<CodeAddress>,
}
impl BlockAddresses{
    pub fn new(&self, method: &RopMethod)    {
        let blocks: BasicBlockList = method.getBlocks();
        let maxLabel: i32 = blocks.getMaxLabel();
        self->starts=new CodeAddress[maxLabel];
        self->lasts=new CodeAddress[maxLabel];
        self->ends=new CodeAddress[maxLabel];
        setupArrays(&method);
    }
    pub fn getStart(&self, block: &BasicBlock) -> CodeAddress    {
        return self.starts[block.getLabel()];
    }
    pub fn getStart(&self, label: i32) -> CodeAddress    {
        return self.starts[label];
    }
    pub fn getLast(&self, block: &BasicBlock) -> CodeAddress    {
        return self.lasts[block.getLabel()];
    }
    pub fn getLast(&self, label: i32) -> CodeAddress    {
        return self.lasts[label];
    }
    pub fn getEnd(&self, block: &BasicBlock) -> CodeAddress    {
        return self.ends[block.getLabel()];
    }
    pub fn getEnd(&self, label: i32) -> CodeAddress    {
        return self.ends[label];
    }
    pub fn setupArrays(&self, method: &RopMethod)    {
        let blocks: BasicBlockList = method.getBlocks();
        let sz: i32 = blocks.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: BasicBlock = blocks.get(i);
            let label: i32 = one.getLabel();
            let insn: Insn = one.getInsns().get(0);
            self.starts[label]=CodeAddress::new(insn.getPosition());
            let pos: SourcePosition = one.getLastInsn().getPosition();
            self.lasts[label]=CodeAddress::new(&pos);
            self.ends[label]=CodeAddress::new(&pos);
        }
    }
}
