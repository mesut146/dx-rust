use crate::helper;

struct AddressMap{
    pub map: HashMap<Integer,Integer>,
}
impl AddressMap{
    pub fn new(&self)    {
        self.map=HashMap<Integer,Integer>::new();
    }
    pub fn get(&self, keyAddress: i32) -> i32    {
        let value: Integer = self.map.get(keyAddress);
        return if (value==None) { -1 } else { value };
            }
            pub fn put(&self, keyAddress: i32, valueAddress: i32)            {
                self.map.put(keyAddress, valueAddress);
            }
}
