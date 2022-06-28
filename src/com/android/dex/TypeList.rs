use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dex::TypeList;
use crate::com::android::dex::util::Unsigned;

let static EMPTY: TypeList = TypeList::new(None, &Dex::EMPTY_SHORT_ARRAY);
struct TypeList{
    pub dex: Dex,
    pub types: Vec<i16>,
}
impl TypeList{
    pub fn new(&self, dex: &Dex, types: &Vec<i16>)    {
        self->dex=dex;
        self->types=types;
    }
    pub fn getTypes(&self) -> Vec<i16>    {
        return self.types;
    }
    pub fn compareTo(&self, other: &TypeList) -> i32    {
        for(        let i: i32 = 0;;i<self.types.len()&&i<.len()i += 1)        {
            if self.types[i]!=[i]            {
                return Unsigned::compare_short_short(self.types[i], [i]);
            }            
        }
        return Unsigned::compare_int_int(self.types.len(), .len());
    }
    pub fn toString(&self) -> String    {
        let result: StringBuilder = StringBuilder::new();
        result.append_String("(");
        for(        let i: i32 = 0;        let typesLength: i32 = self.types.len();;i<typesLengthi += 1)        {
            result.append_Object(if self.dex!=None { self.dex.typeNames().get(self.types[i]) } else { self.types[i] });
                }
                result.append_String(")");
                return result.toString();
            }
}
