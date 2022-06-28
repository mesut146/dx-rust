use crate::helper;
use crate::com::android::dx::dex::code::PositionList::Entry;
use crate::com::android::dx::dex::code::CodeAddress;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::dex::code::PositionList;

let static EMPTY: PositionList = PositionList::new(0);
struct PositionList{
}
impl PositionList{
    pub const NONE: i32 = 1;
    pub const LINES: i32 = 2;
    pub const IMPORTANT: i32 = 3;
    pub fn make(insns: &DalvInsnList, howMuch: i32) -> PositionList    {
        match howMuch{PositionList::NONE =>             {
                return PositionList::EMPTY;
            }PositionList::LINES => PositionList::IMPORTANT =>             {
                break;
            }        _ => {}        {
            throw IllegalArgumentException::new("bogus howMuch");
        }    }
    let noInfo: SourcePosition = SourcePosition::NO_INFO;
    let cur: SourcePosition = noInfo;
    let sz: i32 = insns.size();
    let arr: Vec<Entry> = new PositionList.Entry[sz];
    let lastWasTarget: boolean = false;
    let at: i32 = 0;
    for(    let i: i32 = 0;;i<szi += 1)    {
        let insn: DalvInsn = insns.get(i);
        if //insn instanceof CodeAddress        {
            lastWasTarget=true;

            continue;
        }        
        let pos: SourcePosition = insn.getPosition();
        if pos.equals(&noInfo)||pos.sameLine(&cur)        {
            continue;
        }        
        if (howMuch==PositionList::IMPORTANT)&&!lastWasTarget        {
            continue;
        }        
        cur=pos;
        arr[at]=PositionList.Entry::new(insn.getAddress(), &pos);
        at += 1;
        lastWasTarget=false;
    }
    let result: PositionList = PositionList::new(at);
    for(    let i: i32 = 0;;i<ati += 1)    {
        result.set(i, arr[i]);
    }
    result.setImmutable();
    return result;
}
pub fn new(&self, size: i32){
    super(size);

}
pub fn get(&self, n: i32) -> Entry{
    return (Entry*)get0(n);
}
pub fn set(&self, n: i32, entry: &Entry){
    set0(n, &entry);
}
}
