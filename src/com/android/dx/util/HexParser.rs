use crate::helper;
use crate::com::android::dx::util::Hex;

struct HexParser{
}
impl HexParser{
    pub fn new(&self)    {
    }
    pub fn parse(src: &String) -> Vec<i8>    {
        let len: i32 = src.length();
        let result: Vec<i8> = new byte[len / 2];
        let at: i32 = 0;
        let outAt: i32 = 0;
        while at<len        {
            let nlAt: i32 = src.indexOf_int_int('\n', at);
            if nlAt<0            {
                nlAt=len;
            }            
            let poundAt: i32 = src.indexOf_int_int('#', at);
            let line: String;
            if (poundAt>=0)&&(poundAt<nlAt)            {
                line=src.substring_int_int(at, poundAt);
            }            else             {
                line=src.substring_int_int(at, nlAt);
            }
            at=nlAt+1;
            let colonAt: i32 = line.indexOf_int(':');
            atCheck:
            let lineLen: i32 = line.length();
            let value: i32 = -1;
            let quoteMode: boolean = false;
            for(            let i: i32 = 0;;i<lineLeni += 1)            {
                let c: char = line.charAt(i);
                if quoteMode                {
                    if c=='\"'                    {
                        quoteMode=false;
                    }                    else                     {
                        result[outAt]=c as i8;
                        outAt += 1;
                    }
                    continue;
                }                
                if c<=' '                {
                    continue;
                }                
                if c=='\"'                {
                    if value!=-1                    {
                        throw RuntimeException::new("spare digit around "+"offset "+Hex::u4(outAt));
                    }                    
                    quoteMode=true;
                    continue;
                }                
                let digVal: i32 = Character::digit_char_int(c, 16);
                if digVal==-1                {
                    throw RuntimeException::new("bogus digit character: \""+c+"\"");
                }                
                if value==-1                {
                    value=digVal;
                }                else                 {
                    result[outAt]=((value<<4)|digVal) as i8;
                    outAt += 1;
                    value=-1;
                }
            }
            if value!=-1            {
                throw RuntimeException::new("spare digit around offset "+Hex::u4(outAt));
            }            
            if quoteMode            {
                throw RuntimeException::new("unterminated quote around "+"offset "+Hex::u4(outAt));
            }            
        }
        if outAt<result.len()        {
            let newr: Vec<i8> = new byte[outAt];
            System::arraycopy(&result, 0, &newr, 0, outAt);
            result=newr;
        }        
        return result;
    }
}
