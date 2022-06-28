use crate::helper;
use crate::com::android::dx::cf::code::ByteCatchList::Item;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::util::IntList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::cf::code::ByteCatchList;

let static EMPTY: ByteCatchList = ByteCatchList::new(0);
struct ByteCatchList{
}
impl ByteCatchList{
    pub fn new(&self, count: i32)    {
        super(count);

    }
    pub fn byteLength(&self) -> i32    {
        return 2+size()*8;
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
    pub fn set(&self, n: i32, startPc: i32, endPc: i32, handlerPc: i32, exceptionClass: &CstType)    {
        set0(n, Item::new(startPc, endPc, handlerPc, &exceptionClass));
    }
    pub fn listFor(&self, pc: i32) -> ByteCatchList    {
        let sz: i32 = size();
        let resultArr: Vec<Item> = new Item[sz];
        let resultSz: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Item = get(i);
            if one.covers(pc)&&ByteCatchList::typeNotFound(&one, &resultArr, resultSz)            {
                resultArr[resultSz]=one;
                resultSz += 1;
            }            
        }
        if resultSz==0        {
            return ByteCatchList::EMPTY;
        }        
        let result: ByteCatchList = ByteCatchList::new(resultSz);
        for(        let i: i32 = 0;;i<resultSzi += 1)        {
            result.set_int_Item(i, resultArr[i]);
        }
        result.setImmutable();
        return result;
    }
    pub fn typeNotFound(item: &Item, arr: &Vec<Item>, count: i32) -> boolean    {
        let type: CstType = item.getExceptionClass();
        for(        let i: i32 = 0;;i<counti += 1)        {
            let one: CstType = arr[i].getExceptionClass();
            if (one==type_renamed)||(one==CstType::OBJECT)            {
                return false;
            }            
        }
        return true;
    }
    pub fn toTargetList(&self, noException: i32) -> IntList    {
        if noException<-1        {
            throw IllegalArgumentException::new("noException < -1");
        }        
        let hasDefault: boolean = (noException>=0);
        let sz: i32 = size();
        if sz==0        {
            if hasDefault            {
                return IntList::makeImmutable_int(noException);
            }            
            return IntList::EMPTY;
        }        
        let result: IntList = IntList::new(sz+(if hasDefault { 1 } else { 0 }));
                for(                let i: i32 = 0;;i<szi += 1)                {
                    result.add(get(i).getHandlerPc());
                }
                if hasDefault                {
                    result.add(noException);
                }                
                result.setImmutable();
                return result;
            }
            pub fn toRopCatchList(&self) -> TypeList            {
                let sz: i32 = size();
                if sz==0                {
                    return StdTypeList::EMPTY;
                }                
                let result: StdTypeList = StdTypeList::new(sz);
                for(                let i: i32 = 0;;i<szi += 1)                {
                    result.set(i, get(i).getExceptionClass().getClassType());
                }
                result.setImmutable();
                return result;
            }
}
