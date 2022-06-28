use crate::helper;

struct ShortArrayCodeOutput{
    pub array: Vec<i16>,
}
impl ShortArrayCodeOutput{
    pub fn new(&self, maxSize: i32)    {
        if maxSize<0        {
            throw IllegalArgumentException::new("maxSize < 0");
        }        
        self->array=new short[maxSize];
    }
    pub fn getArray(&self) -> Vec<i16>    {
        let cursor: i32 = cursor();
        if cursor==self.array.len()        {
            return self.array;
        }        
        let result: Vec<i16> = new short[cursor];
        System::arraycopy(&self.array, 0, &result, 0, cursor);
        return result;
    }
    pub fn write(&self, codeUnit: i16)    {
        self.array[cursor()]=codeUnit;
        advance(1);
    }
    pub fn write(&self, u0: i16, u1: i16)    {
        write_short(u0);
        write_short(u1);
    }
    pub fn write(&self, u0: i16, u1: i16, u2: i16)    {
        write_short(u0);
        write_short(u1);
        write_short(u2);
    }
    pub fn write(&self, u0: i16, u1: i16, u2: i16, u3: i16)    {
        write_short(u0);
        write_short(u1);
        write_short(u2);
        write_short(u3);
    }
    pub fn write(&self, u0: i16, u1: i16, u2: i16, u3: i16, u4: i16)    {
        write_short(u0);
        write_short(u1);
        write_short(u2);
        write_short(u3);
        write_short(u4);
    }
    pub fn writeInt(&self, value: i32)    {
        write_short(value as i16);
        write_short((value>>16) as i16);
    }
    pub fn writeLong(&self, value: i64)    {
        write_short(value as i16);
        write_short((value>>16) as i16);
        write_short((value>>32) as i16);
        write_short((value>>48) as i16);
    }
    pub fn write(&self, data: &Vec<i8>)    {
        let value: i32 = 0;
        let even: boolean = true;
        for b in data        {
            if even            {
                value=b&0xff;
                even=false;
            }            else             {
                value|=b<<8;
                write_short(value as i16);
                even=true;
            }
        }
        if !even        {
            write_short(value as i16);
        }        
    }
    pub fn write(&self, data: &Vec<i16>)    {
        for unit in data        {
            write_short(unit);
        }
    }
    pub fn write(&self, data: &Vec<i32>)    {
        for i in data        {
            writeInt(i);
        }
    }
    pub fn write(&self, data: &Vec<i64>)    {
        for l in data        {
            writeLong(l);
        }
    }
}
