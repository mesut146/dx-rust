use crate::helper;
use crate::com::android::dx::cf::code::LineNumberList;
use crate::com::android::dx::cf::code::LineNumberList::Item;

let static EMPTY: LineNumberList = LineNumberList::new(0);
struct LineNumberList{
}
impl LineNumberList{
    pub fn concat(list1: &LineNumberList, list2: &LineNumberList) -> LineNumberList    {
        if list1==LineNumberList::EMPTY        {
            return list2;
        }        
        let sz1: i32 = list1.size();
        let sz2: i32 = list2.size();
        let result: LineNumberList = LineNumberList::new(sz1+sz2);
        for(        let i: i32 = 0;;i<sz1i += 1)        {
            result.set_int_Item(i, list1.get(i));
        }
        for(        let i: i32 = 0;;i<sz2i += 1)        {
            result.set_int_Item(sz1+i, list2.get(i));
        }
        return result;
    }
    pub fn new(&self, count: i32)    {
        super(count);

    }
    pub fn get(&self, n: i32) -> Item    {
        return (Item*)get0(n);
    }
    pub fn set(&self, n: i32, item: &Item)    {
        if item==None        {
            throw NullPointerException::new("item == null");
        }        
        set0(n, &item);
    }
    pub fn set(&self, n: i32, startPc: i32, lineNumber: i32)    {
        set0(n, Item::new(startPc, lineNumber));
    }
    pub fn pcToLine(&self, pc: i32) -> i32    {
        let sz: i32 = size();
        let bestPc: i32 = -1;
        let bestLine: i32 = -1;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Item = get(i);
            let onePc: i32 = one.getStartPc();
            if (onePc<=pc)&&(onePc>bestPc)            {
                bestPc=onePc;
                bestLine=one.getLineNumber();
                if bestPc==pc                {
                    break;
                }                
            }            
        }
        return bestLine;
    }
}
