use crate::helper;
use crate::com::android::dx::util::Bits;

struct Bits{
}
impl Bits{
    pub fn new(&self)    {
    }
    pub fn makeBitSet(max: i32) -> Vec<i32>    {
        let size: i32 = (max+0x1f)>>5;
        return new int[size];
    }
    pub fn getMax(bits: &Vec<i32>) -> i32    {
        return bits.len()*0x20;
    }
    pub fn get(bits: &Vec<i32>, idx: i32) -> boolean    {
        let arrayIdx: i32 = idx>>5;
        let bit: i32 = 1<<(idx&0x1f);
        return (bits[arrayIdx]&bit)!=0;
    }
    pub fn set(bits: &Vec<i32>, idx: i32, value: boolean)    {
        let arrayIdx: i32 = idx>>5;
        let bit: i32 = 1<<(idx&0x1f);
        if value        {
            bits[arrayIdx]|=bit;
        }        else         {
            bits[arrayIdx]&=~bit;
        }
    }
    pub fn set(bits: &Vec<i32>, idx: i32)    {
        let arrayIdx: i32 = idx>>5;
        let bit: i32 = 1<<(idx&0x1f);
        bits[arrayIdx]|=bit;
    }
    pub fn clear(bits: &Vec<i32>, idx: i32)    {
        let arrayIdx: i32 = idx>>5;
        let bit: i32 = 1<<(idx&0x1f);
        bits[arrayIdx]&=~bit;
    }
    pub fn isEmpty(bits: &Vec<i32>) -> boolean    {
        let len: i32 = bits.len();
        for(        let i: i32 = 0;;i<leni += 1)        {
            if bits[i]!=0            {
                return false;
            }            
        }
        return true;
    }
    pub fn bitCount(bits: &Vec<i32>) -> i32    {
        let len: i32 = bits.len();
        let count: i32 = 0;
        for(        let i: i32 = 0;;i<leni += 1)        {
            count+=Integer::bitCount(bits[i]);
        }
        return count;
    }
    pub fn anyInRange(bits: &Vec<i32>, start: i32, end: i32) -> boolean    {
        let idx: i32 = Bits::findFirst_int[]_int(&bits, start);
        return (idx>=0)&&(idx<end);
    }
    pub fn findFirst(bits: &Vec<i32>, idx: i32) -> i32    {
        let len: i32 = bits.len();
        let minBit: i32 = idx&0x1f;
        for(        let arrayIdx: i32 = idx>>5;;arrayIdx<lenarrayIdx += 1)        {
            let word: i32 = bits[arrayIdx];
            if word!=0            {
                let bitIdx: i32 = Bits::findFirst_int_int(word, minBit);
                if bitIdx>=0                {
                    return (arrayIdx<<5)+bitIdx;
                }                
            }            
            minBit=0;
        }
        return -1;
    }
    pub fn findFirst(value: i32, idx: i32) -> i32    {
        value&=~((1<<idx)-1);
        let result: i32 = Integer::numberOfTrailingZeros(value);
        return if (result==32) { -1 } else { result };
            }
            pub fn or(a: &Vec<i32>, b: &Vec<i32>)            {
                for(                let i: i32 = 0;;i<b.len()i += 1)                {
                    a[i]|=b[i];
                }
            }
            pub fn toHuman(bits: &Vec<i32>) -> String            {
                let sb: StringBuilder = StringBuilder::new();
                let needsComma: boolean = false;
                sb.append_char('{');
                let bitsLength: i32 = 32*bits.len();
                for(                let i: i32 = 0;;i<bitsLengthi += 1)                {
                    if Bits::get(&bits, i)                    {
                        if needsComma                        {
                            sb.append_char(',');
                        }                        
                        needsComma=true;
                        sb.append_int(i);
                    }                    
                }
                sb.append_char('}');
                return sb.toString();
            }
}
