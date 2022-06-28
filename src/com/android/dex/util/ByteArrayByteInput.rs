use crate::helper;

struct ByteArrayByteInput{
    pub bytes: Vec<i8>,
    pub position: i32,
}
impl ByteArrayByteInput{
    pub fn new(&self, bytes: i8)    {
        self->bytes=bytes;
    }
    pub fn readByte(&self) -> i8    {
        return self.bytes[self.position += 1];
    }
}
