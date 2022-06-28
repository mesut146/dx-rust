use crate::helper;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::cf::code::ByteCatchList;

struct ByteBlock{
    pub label: i32,
    pub start: i32,
    pub end: i32,
    pub successors: IntList,
    pub catches: ByteCatchList,
}
impl ByteBlock{
    pub fn new(&self, label: i32, start: i32, end: i32, successors: &IntList, catches: &ByteCatchList)    {
        if label<0        {
            throw IllegalArgumentException::new("label < 0");
        }        
        if start<0        {
            throw IllegalArgumentException::new("start < 0");
        }        
        if end<=start        {
            throw IllegalArgumentException::new("end <= start");
        }        
        if successors==None        {
            throw NullPointerException::new("targets == null");
        }        
        let sz: i32 = successors.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            if successors.get(i)<0            {
                throw IllegalArgumentException::new("successors["+i+"] == "+successors.get(i));
            }            
        }
        if catches==None        {
            throw NullPointerException::new("catches == null");
        }        
        self->label=label;
        self->start=start;
        self->end=end;
        self->successors=successors;
        self->catches=catches;
    }
    pub fn toString(&self) -> String    {
        return '{'+Hex::u2(self.label)+": "+Hex::u2(self.start)+".."+Hex::u2(self.end)+'}';
    }
    pub fn getLabel(&self) -> i32    {
        return self.label;
    }
    pub fn getStart(&self) -> i32    {
        return self.start;
    }
    pub fn getEnd(&self) -> i32    {
        return self.end;
    }
    pub fn getSuccessors(&self) -> IntList    {
        return self.successors;
    }
    pub fn getCatches(&self) -> ByteCatchList    {
        return self.catches;
    }
}
