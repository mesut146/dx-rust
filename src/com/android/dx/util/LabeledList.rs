use crate::helper;
use crate::com::android::dx::util::LabeledItem;
use crate::com::android::dx::util::LabeledList;
use crate::com::android::dx::util::IntList;

struct LabeledList{
    pub labelToIndex: IntList,
}
impl LabeledList{
    pub fn new(&self, size: i32)    {
        super(size);

        self.labelToIndex=IntList::new(size);
    }
    pub fn new(&self, old: &LabeledList)    {
        super(old.size());

        self.labelToIndex=.mutableCopy();
        let sz: i32 = old.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Object = old.get0(i);
            if one!=None            {
                set0(i, &one);
            }            
        }
    }
    pub fn getMaxLabel(&self) -> i32    {
        let sz: i32 = self.labelToIndex.size();
        let i: i32;
        for(i=sz-1;(i>=0)&&(self.labelToIndex.get(i)<0)i -= 1)
        let newSize: i32 = i+1;
        self.labelToIndex.shrink(newSize);
        return newSize;
    }
    pub fn removeLabel(&self, oldLabel: i32)    {
        self.labelToIndex.set(oldLabel, -1);
    }
    pub fn addLabelIndex(&self, label: i32, index: i32)    {
        let origSz: i32 = self.labelToIndex.size();
        for(        let i: i32 = 0;;i<=(label-origSz)i += 1)        {
            self.labelToIndex.add(-1);
        }
        self.labelToIndex.set(label, index);
    }
    pub fn indexOfLabel(&self, label: i32) -> i32    {
        if label>=self.labelToIndex.size()        {
            return -1;
        }        else         {
            return self.labelToIndex.get(label);
        }
    }
    pub fn getLabelsInOrder(&self) -> Vec<i32>    {
        let sz: i32 = size();
        let result: Vec<i32> = new int[sz];
        for(        let i: i32 = 0;;i<szi += 1)        {
            let li: LabeledItem = (LabeledItem*)get0(i);
            if li==None            {
                throw NullPointerException::new("null at index "+i);
            }            
            result[i]=li.getLabel();
        }
        Arrays::sort_int[](&result);
        return result;
    }
    pub fn shrinkToFit(&self)    {
        super.shrinkToFit();
        rebuildLabelToIndex();
    }
    pub fn rebuildLabelToIndex(&self)    {
        let szItems: i32 = size();
        for(        let i: i32 = 0;;i<szItemsi += 1)        {
            let li: LabeledItem = (LabeledItem*)get0(i);
            if li!=None            {
                self.labelToIndex.set(li.getLabel(), i);
            }            
        }
    }
    pub fn set(&self, n: i32, item: &LabeledItem)    {
        let old: LabeledItem = (LabeledItem*)getOrNull0(n);
        set0(n, &item);
        if old!=None        {
            removeLabel(old.getLabel());
        }        
        if item!=None        {
            addLabelIndex(item.getLabel(), n);
        }        
    }
}
