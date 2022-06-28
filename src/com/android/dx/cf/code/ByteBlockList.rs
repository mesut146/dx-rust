use crate::helper;
use crate::com::android::dx::cf::code::ByteBlock;
use crate::com::android::dx::util::Hex;

struct ByteBlockList{
}
impl ByteBlockList{
    pub fn new(&self, size: i32)    {
        super(size);

    }
    pub fn get(&self, n: i32) -> ByteBlock    {
        return (ByteBlock*)get0(n);
    }
    pub fn labelToBlock(&self, label: i32) -> ByteBlock    {
        let idx: i32 = indexOfLabel(label);
        if idx<0        {
            throw IllegalArgumentException::new("no such label: "+Hex::u2(label));
        }        
        return get(idx);
    }
    pub fn set(&self, n: i32, bb: &ByteBlock)    {
        super.set(n,bb);
    }
}
