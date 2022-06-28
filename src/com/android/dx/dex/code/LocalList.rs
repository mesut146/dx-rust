use crate::helper;
use crate::com::android::dx::dex::code::LocalList::MakeState;
use crate::com::android::dx::rop::code::RegisterSpecSet;
use crate::com::android::dx::dex::code::LocalList::Entry;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::LocalSnapshot;
use crate::com::android::dx::dex::code::LocalStart;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::code::LocalList::Disposition;
use crate::com::android::dx::dex::code::LocalList;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::rop::type::Type;

let static EMPTY: LocalList = LocalList::new(0);
struct LocalList{
}
impl LocalList{
    pub const DEBUG: boolean = false;
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Entry    {
        return (Entry*)get0(n);
    }
    pub fn set(&self, n: i32, entry: &Entry)    {
        set0(n, &entry);
    }
    pub fn debugPrint(&self, out: &PrintStream, prefix: &String)    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            out.print_String(&prefix);
            out.println_Object(get(i));
        }
    }
    pub fn make(insns: &DalvInsnList) -> LocalList    {
        let sz: i32 = insns.size();
        let state: MakeState = MakeState::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            let insn: DalvInsn = insns.get(i);
            if //insn instanceof LocalSnapshot            {
                let snapshot: RegisterSpecSet = ((LocalSnapshot*)insn).getLocals();
                state.snapshot(insn.getAddress(), &snapshot);
            }            else             if //insn instanceof LocalStart            {
                let local: RegisterSpec = ((LocalStart*)insn).getLocal();
                state.startLocal(insn.getAddress(), &local);
            }            
        }
        let result: LocalList = state.finish();
        if LocalList::DEBUG        {
            LocalList::debugVerify(&result);
        }        
        return result;
    }
    pub fn debugVerify(locals: &LocalList)    {
        try        {
            LocalList::debugVerify0(&locals);
        }        catch(        let ex: RuntimeException)        {
            let sz: i32 = locals.size();
            for(            let i: i32 = 0;;i<szi += 1)            {
                System::err.println_Object(locals.get(i));
            }
            throw ex;
        }
    }
    pub fn debugVerify0(locals: &LocalList)    {
        let sz: i32 = locals.size();
        let active: Vec<Entry> = new Entry[65536];
        for(        let i: i32 = 0;;i<szi += 1)        {
            let e: Entry = locals.get(i);
            let reg: i32 = e.getRegister();
            if e.isStart()            {
                let already: Entry = active[reg];
                if (already!=None)&&e.matches_Entry(&already)                {
                    throw RuntimeException::new("redundant start at "+Integer::toHexString(e.getAddress())+": got "+e+"; had "+already);
                }                
                active[reg]=e;
            }            else             {
                if active[reg]==None                {
                    throw RuntimeException::new("redundant end at "+Integer::toHexString(e.getAddress()));
                }                
                let addr: i32 = e.getAddress();
                let foundStart: boolean = false;
                for(                let j: i32 = i+1;;j<szj += 1)                {
                    let test: Entry = locals.get(j);
                    if test.getAddress()!=addr                    {
                        break;
                    }                    
                    if test.getRegisterSpec().getReg()==reg                    {
                        if test.isStart()                        {
                            if e.getDisposition()!=Disposition::END_REPLACED                            {
                                throw RuntimeException::new("improperly marked end at "+Integer::toHexString(addr));
                            }                            
                            foundStart=true;
                        }                        else                         {
                            throw RuntimeException::new("redundant end at "+Integer::toHexString(addr));
                        }
                    }                    
                }
                if !foundStart&&(e.getDisposition()==Disposition::END_REPLACED)                {
                    throw RuntimeException::new("improper end replacement claim at "+Integer::toHexString(addr));
                }                
                active[reg]=None;
            }
        }
    }
}
