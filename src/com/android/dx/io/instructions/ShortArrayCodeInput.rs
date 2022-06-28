use crate::helper;

struct ShortArrayCodeInput{
    pub array: Vec<i16>,
}
impl ShortArrayCodeInput{
    pub fn new(&self, array: &Vec<i16>)    {
        if array==None        {
            throw NullPointerException::new("array == null");
        }        
        self->array=array;
    }
    pub fn hasMore(&self) -> boolean    {
        return cursor()<self.array.len();
    }
    pub fn read(&self) -> i32    {
        try        {
            let value: i32 = self.array[cursor()];
            advance(1);
            return value&0xffff;
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            throw EOFException::new();
        }
    }
    pub fn readInt(&self) -> i32    {
        let short0: i32 = read();
        let short1: i32 = read();
        return short0|(short1<<16);
    }
    pub fn readLong(&self) -> i64    {
        let short0: i64 = read();
        let short1: i64 = read();
        let short2: i64 = read();
        let short3: i64 = read();
        return short0|(short1<<16)|(short2<<32)|(short3<<48);
    }
}
