use crate::helper;
use crate::com::android::dx::util::ListIntSet;
use crate::com::android::dx::util::BitIntSet;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::util::IntIterator;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::util::Bits;

struct ListIntSet{
    pub ints: IntList,
}
impl ListIntSet{
    pub fn new(&self)    {
        self.ints=IntList::new();
        self.ints.sort();
    }
    pub fn add(&self, value: i32)    {
        let index: i32 = self.ints.binarysearch(value);
        if index<0        {
            self.ints.insert(-(index+1), value);
        }        
    }
    pub fn remove(&self, value: i32)    {
        let index: i32 = self.ints.indexOf(value);
        if index>=0        {
            self.ints.removeIndex(index);
        }        
    }
    pub fn has(&self, value: i32) -> boolean    {
        return self.ints.indexOf(value)>=0;
    }
    pub fn merge(&self, other: &IntSet)    {
        if //other instanceof ListIntSet        {
            let o: ListIntSet = (ListIntSet*)other;
            let szThis: i32 = self.ints.size();
            let szOther: i32 = .size();
            let i: i32 = 0;
            let j: i32 = 0;
            while j<szOther&&i<szThis            {
                while j<szOther&&.get(j)<self.ints.get(i)                {
                    add(.get(j += 1));
                }
                if j==szOther                {
                    break;
                }                
                while i<szThis&&.get(j)>=self.ints.get(i)                {
                    i += 1;
                }
            }
            while j<szOther            {
                add(.get(j += 1));
            }
            self.ints.sort();
        }        else         if //other instanceof BitIntSet        {
            let o: BitIntSet = (BitIntSet*)other;
            for(            let i: i32 = 0;;i>=0i=Bits::findFirst_int[]_int(&, i+1))            {
                self.ints.add(i);
            }
            self.ints.sort();
        }        else         {
            let iter: IntIterator = other.iterator();
            while iter.hasNext()            {
                add(iter.next());
            }
        }
    }
    pub fn elements(&self) -> i32    {
        return self.ints.size();
    }
    pub fn iterator(&self) -> IntIterator    {
        return /*new IntIterator(){
  private int idx=0;
  /** 
 * {@inheritDoc} 
 */
  @Override public boolean hasNext(){
    return idx < ints.size();
  }
  /** 
 * {@inheritDoc} 
 */
  @Override public int next(){
    if (!hasNext()) {
      throw new NoSuchElementException();
    }
    return ints.get(idx++);
  }
}
*/;
    }
    pub fn toString(&self) -> String    {
        return self.ints.toString();
    }
}
