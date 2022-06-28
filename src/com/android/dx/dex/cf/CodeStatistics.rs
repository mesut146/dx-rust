use crate::helper;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dx::rop::code::BasicBlockList;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::rop::code::RopMethod;

struct CodeStatistics{
    pub runningDeltaRegisters: i32,
    pub runningDeltaInsns: i32,
    pub runningTotalInsns: i32,
    pub dexRunningDeltaRegisters: i32,
    pub dexRunningDeltaInsns: i32,
    pub dexRunningTotalInsns: i32,
    pub runningOriginalBytes: i32,
}
impl CodeStatistics{
    pub const DEBUG: boolean = false;
    pub fn updateOriginalByteCount(&self, count: i32)    {
        self.runningOriginalBytes+=count;
    }
    pub fn updateDexStatistics(&self, nonOptCode: &DalvCode, code: &DalvCode)    {
        if CodeStatistics::DEBUG        {
            System::err.println_String("dex insns (old/new) "+nonOptCode.getInsns().codeSize()+"/"+code.getInsns().codeSize()+" regs (o/n) "+nonOptCode.getInsns().getRegistersSize()+"/"+code.getInsns().getRegistersSize());
        }        
        self.dexRunningDeltaInsns+=(code.getInsns().codeSize()-nonOptCode.getInsns().codeSize());
        self.dexRunningDeltaRegisters+=(code.getInsns().getRegistersSize()-nonOptCode.getInsns().getRegistersSize());
        self.dexRunningTotalInsns+=code.getInsns().codeSize();
    }
    pub fn updateRopStatistics(&self, nonOptRmeth: &RopMethod, rmeth: &RopMethod)    {
        let oldCountInsns: i32 = nonOptRmeth.getBlocks().getEffectiveInstructionCount();
        let oldCountRegs: i32 = nonOptRmeth.getBlocks().getRegCount();
        if CodeStatistics::DEBUG        {
            System::err.println_String("insns (old/new): "+oldCountInsns+"/"+rmeth.getBlocks().getEffectiveInstructionCount()+" regs (o/n):"+oldCountRegs+"/"+rmeth.getBlocks().getRegCount());
        }        
        let newCountInsns: i32 = rmeth.getBlocks().getEffectiveInstructionCount();
        self.runningDeltaInsns+=(newCountInsns-oldCountInsns);
        self.runningDeltaRegisters+=(rmeth.getBlocks().getRegCount()-oldCountRegs);
        self.runningTotalInsns+=newCountInsns;
    }
    pub fn dumpStatistics(&self, out: &PrintStream)    {
        out.printf_String_Object[]("Optimizer Delta Rop Insns: %d total: %d "+"(%.2f%%) Delta Registers: %d\n", self.runningDeltaInsns, self.runningTotalInsns, (100.0*((self.runningDeltaInsns as f32)/(self.runningTotalInsns+Math::abs_int(self.runningDeltaInsns)))), self.runningDeltaRegisters);
        out.printf_String_Object[]("Optimizer Delta Dex Insns: Insns: %d total: %d "+"(%.2f%%) Delta Registers: %d\n", self.dexRunningDeltaInsns, self.dexRunningTotalInsns, (100.0*((self.dexRunningDeltaInsns as f32)/(self.dexRunningTotalInsns+Math::abs_int(self.dexRunningDeltaInsns)))), self.dexRunningDeltaRegisters);
        out.printf_String_Object[]("Original bytecode byte count: %d\n", self.runningOriginalBytes);
    }
}
