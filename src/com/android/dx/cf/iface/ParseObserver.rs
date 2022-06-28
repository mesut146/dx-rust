use crate::helper;

trait ParseObserver{
    pub fn changeIndent(&self, indentDelta: i32);
    pub fn startParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String);
    pub fn endParsingMember(&self, bytes: &ByteArray, offset: i32, name: &String, descriptor: &String, member: &Member);
    pub fn parsed(&self, bytes: &ByteArray, offset: i32, len: i32, human: &String);
}
