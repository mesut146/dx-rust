use crate::helper;

trait CodeOutput{
    pub fn write(&self, codeUnit: i16);
    pub fn write(&self, u0: i16, u1: i16);
    pub fn write(&self, u0: i16, u1: i16, u2: i16);
    pub fn write(&self, u0: i16, u1: i16, u2: i16, u3: i16);
    pub fn write(&self, u0: i16, u1: i16, u2: i16, u3: i16, u4: i16);
    pub fn writeInt(&self, value: i32);
    pub fn writeLong(&self, value: i64);
    pub fn write(&self, data: &Vec<i8>);
    pub fn write(&self, data: &Vec<i16>);
    pub fn write(&self, data: &Vec<i32>);
    pub fn write(&self, data: &Vec<i64>);
}
