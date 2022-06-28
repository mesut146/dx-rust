use crate::helper;
use crate::com::android::dx::rop::code::LocalItem;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::cf::code::LocalVariableList;
use crate::com::android::dx::cf::code::LocalVariableList::Item;
use crate::com::android::dx::rop::type::Type;

let static EMPTY: LocalVariableList = LocalVariableList::new(0);
struct LocalVariableList{
}
impl LocalVariableList{
    pub fn concat(list1: &LocalVariableList, list2: &LocalVariableList) -> LocalVariableList    {
        if list1==LocalVariableList::EMPTY        {
            return list2;
        }        
        let sz1: i32 = list1.size();
        let sz2: i32 = list2.size();
        let result: LocalVariableList = LocalVariableList::new(sz1+sz2);
        for(        let i: i32 = 0;;i<sz1i += 1)        {
            result.set_int_Item(i, list1.get(i));
        }
        for(        let i: i32 = 0;;i<sz2i += 1)        {
            result.set_int_Item(sz1+i, list2.get(i));
        }
        result.setImmutable();
        return result;
    }
    pub fn mergeDescriptorsAndSignatures(descriptorList: &LocalVariableList, signatureList: &LocalVariableList) -> LocalVariableList    {
        let descriptorSize: i32 = descriptorList.size();
        let result: LocalVariableList = LocalVariableList::new(descriptorSize);
        for(        let i: i32 = 0;;i<descriptorSizei += 1)        {
            let item: Item = descriptorList.get(i);
            let signatureItem: Item = signatureList.itemToLocal(&item);
            if signatureItem!=None            {
                let signature: CstString = signatureItem.getSignature();
                item=item.withSignature(&signature);
            }            
            result.set_int_Item(i, &item);
        }
        result.setImmutable();
        return result;
    }
    pub fn new(&self, count: i32)    {
        super(count);

    }
    pub fn get(&self, n: i32) -> Item    {
        return (Item*)get0(n);
    }
    pub fn set(&self, n: i32, item: &Item)    {
        if item==None        {
            throw NullPointerException::new("item == null");
        }        
        set0(n, &item);
    }
    pub fn set(&self, n: i32, startPc: i32, length: i32, name: &CstString, descriptor: &CstString, signature: &CstString, index: i32)    {
        set0(n, Item::new(startPc, length, &name, &descriptor, &signature, index));
    }
    pub fn itemToLocal(&self, item: &Item) -> Item    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Item = (Item*)get0(i);
            if (one!=None)&&one.matchesAllButType(&item)            {
                return one;
            }            
        }
        return None;
    }
    pub fn pcAndIndexToLocal(&self, pc: i32, index: i32) -> Item    {
        let sz: i32 = size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Item = (Item*)get0(i);
            if (one!=None)&&one.matchesPcAndIndex(pc, index)            {
                return one;
            }            
        }
        return None;
    }
}
