use crate::helper;
use crate::com::android::dx::dex::code::CatchHandlerList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::code::CatchHandlerList::Entry;
use crate::com::android::dx::util::Hex;

let static EMPTY: CatchHandlerList = CatchHandlerList::new(0);
struct CatchHandlerList{
}
impl CatchHandlerList{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> Entry    {
        return (Entry*)get0(n);
    }
    pub fn toHuman(&self) -> String    {
        return toHuman_String_String("", "");
    }
    pub fn toHuman(&self, prefix: &String, header: &String) -> String    {
        let sb: StringBuilder = StringBuilder::new(100);
        let size: i32 = size();
        sb.append_String(&prefix);
        sb.append_String(&header);
        sb.append_String("catch ");
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let entry: Entry = get(i);
            if i!=0            {
                sb.append_String(",\n");
                sb.append_String(&prefix);
                sb.append_String("  ");
            }            
            if (i==(size-1))&&catchesAll()            {
                sb.append_String("<any>");
            }            else             {
                sb.append_String(entry.getExceptionType().toHuman());
            }
            sb.append_String(" -> ");
            sb.append_String(Hex::u2or4(entry.getHandler()));
        }
        return sb.toString();
    }
    pub fn catchesAll(&self) -> boolean    {
        let size: i32 = size();
        if size==0        {
            return false;
        }        
        let last: Entry = get(size-1);
        return last.getExceptionType().equals(&CstType::OBJECT);
    }
    pub fn set(&self, n: i32, exceptionType: &CstType, handler: i32)    {
        set0(n, Entry::new(&exceptionType, handler));
    }
    pub fn set(&self, n: i32, entry: &Entry)    {
        set0(n, &entry);
    }
    pub fn compareTo(&self, other: &CatchHandlerList) -> i32    {
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
