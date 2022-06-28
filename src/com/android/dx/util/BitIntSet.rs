use crate::helper;
use crate::com::android::dx::util::ListIntSet;
use crate::com::android::dx::util::BitIntSet;
use crate::com::android::dx::util::IntSet;
use crate::com::android::dx::util::IntIterator;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::util::Bits;

struct BitIntSet{
    pub bits: Vec<i32>,
}
impl BitIntSet{
    pub fn new(&self, max: i32)    {
        self.bits=Bits::makeBitSet(max);
    }
    pub fn add(&self, value: i32)    {
        ensureCapacity(value);
        Bits::set_int[]_int_boolean(&self.bits, value, true);
    }
    pub fn ensureCapacity(&self, value: i32)    {
        if value>=Bits::getMax(&self.bits)        {
            let newBits: Vec<i32> = Bits::makeBitSet(Math::max_int_int(value+1, 2*Bits::getMax(&self.bits)));
            System::arraycopy(&self.bits, 0, &newBits, 0, self.bits.len());
            self.bits=newBits;
        }        
    }
    pub fn remove(&self, value: i32)    {
        if value<Bits::getMax(&self.bits)        {
            Bits::set_int[]_int_boolean(&self.bits, value, false);
        }        
    }
    pub fn has(&self, value: i32) -> boolean    {
        return (value<Bits::getMax(&self.bits))&&Bits::get(&self.bits, value);
    }
    pub fn merge(&self, other: &IntSet)    {
        if //other instanceof BitIntSet        {
            let o: BitIntSet = (BitIntSet*)other;
            ensureCapacity(Bits::getMax(&)+1);
            Bits::or(&self.bits, &);
        }        else         if //other instanceof ListIntSet        {
            let o: ListIntSet = (ListIntSet*)other;
            let sz: i32 = .size();
            if sz>0            {
                ensureCapacity(.get(sz-1));
            }            
            for(            let i: i32 = 0;;i<.size()i += 1)            {
                Bits::set_int[]_int_boolean(&self.bits, .get(i), true);
            }
        }        else         {
            let iter: IntIterator = other.iterator();
            while iter.hasNext()            {
                add(iter.next());
            }
        }
    }
    pub fn elements(&self) -> i32    {
        return Bits::bitCount(&self.bits);
    }
    pub fn iterator(&self) -> IntIterator    {
        return /*new IntIterator(){
  private int idx=Bits.findFirst(bits,0);
  /** 
 * {@inheritDoc} 
 */
  @Override public boolean hasNext(){
    return idx >= 0;
  }
  /** 
 * {@inheritDoc} 
 */
  @Override public int next(){
    if (!hasNext()) {
      throw new NoSuchElementException();
    }
    int ret=idx;
    idx=Bits.findFirst(bits,idx + 1);
    return ret;
  }
}
*/;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        sb.append_char('{');
        let first: boolean = true;
        for(        let i: i32 = Bits::findFirst_int[]_int(&self.bits, 0);;i>=0i=Bits::findFirst_int[]_int(&self.bits, i+1))        {
            if !first            {
                sb.append_String(", ");
            }            
            first=false;
            sb.append_int(i);
        }
        sb.append_char('}');
        return sb.toString();
    }
}
