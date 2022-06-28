use crate::helper;
use crate::com::android::dx::util::Hex;

struct Hex{
}
impl Hex{
    pub fn new(&self)    {
    }
    pub fn u8(v: i64) -> String    {
        let result: Vec<char> = new char[16];
        for(        let i: i32 = 0;;i<16i += 1)        {
            result[15-i]=Character::forDigit(v as i32&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn u4(v: i32) -> String    {
        let result: Vec<char> = new char[8];
        for(        let i: i32 = 0;;i<8i += 1)        {
            result[7-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn u3(v: i32) -> String    {
        let result: Vec<char> = new char[6];
        for(        let i: i32 = 0;;i<6i += 1)        {
            result[5-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn u2(v: i32) -> String    {
        let result: Vec<char> = new char[4];
        for(        let i: i32 = 0;;i<4i += 1)        {
            result[3-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn u2or4(v: i32) -> String    {
        if v==v as char        {
            return Hex::u2(v);
        }        else         {
            return Hex::u4(v);
        }
    }
    pub fn u1(v: i32) -> String    {
        let result: Vec<char> = new char[2];
        for(        let i: i32 = 0;;i<2i += 1)        {
            result[1-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn uNibble(v: i32) -> String    {
        let result: Vec<char> = new char[1];
        result[0]=Character::forDigit(v&0x0f, 16);
        return String::new(&result);
    }
    pub fn s8(v: i64) -> String    {
        let result: Vec<char> = new char[17];
        if v<0        {
            result[0]='-';
            v=-v;
        }        else         {
            result[0]='+';
        }
        for(        let i: i32 = 0;;i<16i += 1)        {
            result[16-i]=Character::forDigit(v as i32&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn s4(v: i32) -> String    {
        let result: Vec<char> = new char[9];
        if v<0        {
            result[0]='-';
            v=-v;
        }        else         {
            result[0]='+';
        }
        for(        let i: i32 = 0;;i<8i += 1)        {
            result[8-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn s2(v: i32) -> String    {
        let result: Vec<char> = new char[5];
        if v<0        {
            result[0]='-';
            v=-v;
        }        else         {
            result[0]='+';
        }
        for(        let i: i32 = 0;;i<4i += 1)        {
            result[4-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn s1(v: i32) -> String    {
        let result: Vec<char> = new char[3];
        if v<0        {
            result[0]='-';
            v=-v;
        }        else         {
            result[0]='+';
        }
        for(        let i: i32 = 0;;i<2i += 1)        {
            result[2-i]=Character::forDigit(v&0x0f, 16);
            v>>=4;
        }
        return String::new(&result);
    }
    pub fn dump(arr: &Vec<i8>, offset: i32, length: i32, outOffset: i32, bpl: i32, addressLength: i32) -> String    {
        let end: i32 = offset+length;
        if ((offset|length|end)<0)||(end>arr.len())        {
            throw IndexOutOfBoundsException::new("arr.length "+arr.len()+"; "+offset+"..!"+end);
        }        
        if outOffset<0        {
            throw IllegalArgumentException::new("outOffset < 0");
        }        
        if length==0        {
            return "";
        }        
        let sb: StringBuilder = StringBuilder::new(length*4+6);
        let bol: boolean = true;
        let col: i32 = 0;
        while length>0        {
            if col==0            {
                let astr: String;
                match addressLength{2 =>                     astr=Hex::u1(outOffset);                    break;4 =>                     astr=Hex::u2(outOffset);                    break;6 =>                     astr=Hex::u3(outOffset);                    break;                _ => {}                astr=Hex::u4(outOffset);                break;            }
            sb.append_String(&astr);
            sb.append_String(": ");
        }        else         if (col&1)==0        {
            sb.append_char(' ');
        }        
        sb.append_String(Hex::u1(arr[offset]));
        outOffset += 1;
        offset += 1;
        col += 1;
        if col==bpl        {
            sb.append_char('\n');
            col=0;
        }        
        length -= 1;
    }
    if col!=0    {
        sb.append_char('\n');
    }    
    return sb.toString();
}
}
