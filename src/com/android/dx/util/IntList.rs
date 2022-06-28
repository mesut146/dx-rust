use crate::helper;
use crate::com::android::dx::util::IntList;

let static EMPTY: IntList = IntList::new(0);
struct IntList{
    pub values: Vec<i32>,
    pub size: i32,
    pub sorted: boolean,
}
impl IntList{
    pub fn makeImmutable(value: i32) -> IntList    {
        let result: IntList = IntList::new(1);
        result.add(value);
        result.setImmutable();
        return result;
    }
    pub fn makeImmutable(value0: i32, value1: i32) -> IntList    {
        let result: IntList = IntList::new(2);
        result.add(value0);
        result.add(value1);
        result.setImmutable();
        return result;
    }
    pub fn new(&self)    {
        this(4);

    }
    pub fn new(&self, initialCapacity: i32)    {
        super(true);

        try        {
            self.values=new int[initialCapacity];
        }        catch(        let ex: NegativeArraySizeException)        {
            throw IllegalArgumentException::new("size < 0");
        }
        self.size=0;
        self.sorted=true;
    }
    pub fn hashCode(&self) -> i32    {
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<self.sizei += 1)        {
            result=(result*31)+self.values[i];
        }
        return result;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if other==self        {
            return true;
        }        
        if !(//other instanceof IntList)        {
            return false;
        }        
        let otherList: IntList = (IntList*)other;
        if self.sorted!=        {
            return false;
        }        
        if self.size!=        {
            return false;
        }        
        for(        let i: i32 = 0;;i<self.sizei += 1)        {
            if self.values[i]!=[i]            {
                return false;
            }            
        }
        return true;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(self.size*5+10);
        sb.append_char('{');
        for(        let i: i32 = 0;;i<self.sizei += 1)        {
            if i!=0            {
                sb.append_String(", ");
            }            
            sb.append_int(self.values[i]);
        }
        sb.append_char('}');
        return sb.toString();
    }
    pub fn size(&self) -> i32    {
        return self.size;
    }
    pub fn get(&self, n: i32) -> i32    {
        if n>=self.size        {
            throw IndexOutOfBoundsException::new("n >= size()");
        }        
        try        {
            return self.values[n];
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            throw IndexOutOfBoundsException::new("n < 0");
        }
    }
    pub fn set(&self, n: i32, value: i32)    {
        throwIfImmutable();
        if n>=self.size        {
            throw IndexOutOfBoundsException::new("n >= size()");
        }        
        try        {
            self.values[n]=value;
            self.sorted=false;
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            if n<0            {
                throw IllegalArgumentException::new("n < 0");
            }            
        }
    }
    pub fn add(&self, value: i32)    {
        throwIfImmutable();
        growIfNeeded();
        self.values[self.size += 1]=value;
        if self.sorted&&(self.size>1)        {
            self.sorted=(value>=self.values[self.size-2]);
        }        
    }
    pub fn insert(&self, n: i32, value: i32)    {
        if n>self.size        {
            throw IndexOutOfBoundsException::new("n > size()");
        }        
        growIfNeeded();
        System::arraycopy(&self.values, n, &self.values, n+1, self.size-n);
        self.values[n]=value;
        self.size += 1;
        self.sorted=self.sorted&&(n==0||value>self.values[n-1])&&(n==(self.size-1)||value<self.values[n+1]);
    }
    pub fn removeIndex(&self, n: i32)    {
        if n>=self.size        {
            throw IndexOutOfBoundsException::new("n >= size()");
        }        
        System::arraycopy(&self.values, n+1, &self.values, n, self.size-n-1);
        self.size -= 1;
    }
    pub fn growIfNeeded(&self)    {
        if self.size==self.values.len()        {
            let newv: Vec<i32> = new int[size * 3 / 2 + 10];
            System::arraycopy(&self.values, 0, &newv, 0, self.size);
            self.values=newv;
        }        
    }
    pub fn top(&self) -> i32    {
        return get(self.size-1);
    }
    pub fn pop(&self) -> i32    {
        throwIfImmutable();
        let result: i32;
        result=get(self.size-1);
        self.size -= 1;
        return result;
    }
    pub fn pop(&self, n: i32)    {
        throwIfImmutable();
        self.size-=n;
    }
    pub fn shrink(&self, newSize: i32)    {
        if newSize<0        {
            throw IllegalArgumentException::new("newSize < 0");
        }        
        if newSize>self.size        {
            throw IllegalArgumentException::new("newSize > size");
        }        
        throwIfImmutable();
        self.size=newSize;
    }
    pub fn mutableCopy(&self) -> IntList    {
        let sz: i32 = self.size;
        let result: IntList = IntList::new(sz);
        for(        let i: i32 = 0;;i<szi += 1)        {
            result.add(self.values[i]);
        }
        return result;
    }
    pub fn sort(&self)    {
        throwIfImmutable();
        if !self.sorted        {
            Arrays::sort_int[]_int_int(&self.values, 0, self.size);
            self.sorted=true;
        }        
    }
    pub fn indexOf(&self, value: i32) -> i32    {
        let ret: i32 = binarysearch(value);
        return if ret>=0 { ret } else { -1 };
            }
            pub fn binarysearch(&self, value: i32) -> i32            {
                let sz: i32 = self.size;
                if !self.sorted                {
                    for(                    let i: i32 = 0;;i<szi += 1)                    {
                        if self.values[i]==value                        {
                            return i;
                        }                        
                    }
                    return -sz;
                }                
                let min: i32 = -1;
                let max: i32 = sz;
                while max>(min+1)                {
                    let guessIdx: i32 = min+((max-min)>>1);
                    let guess: i32 = self.values[guessIdx];
                    if value<=guess                    {
                        max=guessIdx;
                    }                    else                     {
                        min=guessIdx;
                    }
                }
                if (max!=sz)                {
                    return if (value==self.values[max]) { max } else { (-max-1) };
                        }                        else                         {
                            return -sz-1;
                        }
                    }
                    pub fn contains(&self, value: i32) -> boolean                    {
                        return indexOf(value)>=0;
                    }
}
