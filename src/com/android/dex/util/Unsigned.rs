use crate::helper;

struct Unsigned{
}
impl Unsigned{
    pub fn new(&self)    {
    }
    pub fn compare(ushortA: i16, ushortB: i16) -> i32    {
        if ushortA==ushortB        {
            return 0;
        }        
        let a: i32 = ushortA&0xFFFF;
        let b: i32 = ushortB&0xFFFF;
        return if a<b { -1 } else { 1 };
            }
            pub fn compare(uintA: i32, uintB: i32) -> i32            {
                if uintA==uintB                {
                    return 0;
                }                
                let a: i64 = uintA&0xFFFFFFFFL;
                let b: i64 = uintB&0xFFFFFFFFL;
                return if a<b { -1 } else { 1 };
                    }
}
