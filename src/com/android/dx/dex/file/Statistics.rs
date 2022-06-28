use crate::helper;
use crate::com::android::dx::dex::file::Item;
use crate::com::android::dx::dex::file::Statistics::Data;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::util::AnnotatedOutput;

struct Statistics{
    pub dataMap: HashMap<String,Data>,
}
impl Statistics{
    pub fn new(&self)    {
        self.dataMap=HashMap<String,Data>::new(50);
    }
    pub fn add(&self, item: &Item)    {
        let typeName: String = item.typeName();
        let data: Data = self.dataMap.get(&typeName);
        if data==None        {
            self.dataMap.put(&typeName, Data::new(&item, &typeName));
        }        else         {
            data.add(&item);
        }
    }
    pub fn addAll(&self, list: &Section)    {
        let items: Collection<? extends Item> = list.items();
        for item in items        {
            add(&item);
        }
    }
    pub fn writeAnnotation(&self, out: &AnnotatedOutput)    {
        if self.dataMap.size()==0        {
            return;
        }        
        out.annotate_int_String(0, "\nstatistics:\n");
        let sortedData: TreeMap<String,Data> = TreeMap<String,Data>::new();
        for data in self.dataMap.values()        {
            sortedData.put(&, &data);
        }
        for data in sortedData.values()        {
            data.writeAnnotation(&out);
        }
    }
    pub fn toHuman(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new();
        sb.append_String("Statistics:\n");
        let sortedData: TreeMap<String,Data> = TreeMap<String,Data>::new();
        for data in self.dataMap.values()        {
            sortedData.put(&, &data);
        }
        for data in sortedData.values()        {
            sb.append_String(data.toHuman());
        }
        return sb.toString();
    }
}
