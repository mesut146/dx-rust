use crate::helper;
use crate::com::android::dx::dex::code::CatchTable;
use crate::com::android::dx::dex::code::CatchHandlerList;
use crate::com::android::dx::dex::code::CatchTable::Entry;

let static EMPTY: CatchTable = CatchTable::new(0);
struct CatchTable{
}
impl CatchTable{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Entry    {
        return (Entry*)get0(n);
    }
    pub fn set(&self, n: i32, entry: &Entry)    {
        set0(n, &entry);
    }
    pub fn compareTo(&self, other: &CatchTable) -> i32    {
        if self==other        {
            return 0;
        }        
        let thisSize: i32 = size();
        let otherSize: i32 = other.size();
        let checkSize: i32 = Math::min_int_int(thisSize, otherSize);
        for(        let i: i32 = 0;;i<checkSizei += 1)        {
            let thisEntry: Entry = get(i);
            let otherEntry: Entry = other.get(i);
            let compare: i32 = thisEntry.compareTo(&otherEntry);
            if compare!=0            {
                return compare;
            }            
        }
        if thisSize<otherSize        {
            return -1;
        }        else         if thisSize>otherSize        {
            return 1;
        }        
        return 0;
    }
}
