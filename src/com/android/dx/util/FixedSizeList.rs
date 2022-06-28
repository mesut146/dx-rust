use crate::helper;
use crate::com::android::dx::util::ToHuman;
use crate::com::android::dx::util::FixedSizeList;

struct FixedSizeList{
    pub arr: Vec<Object>,
}
impl FixedSizeList{
    pub fn new(&self, size: i32)    {
        super(size != 0);

        try        {
            self.arr=new Object[size];
        }        catch(        let ex: NegativeArraySizeException)        {
            throw IllegalArgumentException::new("size < 0");
        }
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if self==other        {
            return true;
        }        
        if (other==None)||(getClass()!=other.getClass())        {
            return false;
        }        
        let list: FixedSizeList = (FixedSizeList*)other;
        return Arrays::equals_Object[]_Object[](&self.arr, &);
    }
    pub fn hashCode(&self) -> i32    {
        return Arrays::hashCode_Object[](&self.arr);
    }
    pub fn toString(&self) -> String    {
        let name: String = getClass().getName();
        return toString0(name.substring_int(name.lastIndexOf_int('.')+1)+'{', ", ", "}", false);
    }
    pub fn toHuman(&self) -> String    {
        let name: String = getClass().getName();
        return toString0(name.substring_int(name.lastIndexOf_int('.')+1)+'{', ", ", "}", true);
    }
    pub fn toString(&self, prefix: &String, separator: &String, suffix: &String) -> String    {
        return toString0(&prefix, &separator, &suffix, false);
    }
    pub fn toHuman(&self, prefix: &String, separator: &String, suffix: &String) -> String    {
        return toString0(&prefix, &separator, &suffix, true);
    }
    pub fn size(&self) -> i32    {
        return self.arr.len();
    }
    pub fn shrinkToFit(&self)    {
        let sz: i32 = self.arr.len();
        let newSz: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            if self.arr[i]!=None            {
                newSz += 1;
            }            
        }
        if sz==newSz        {
            return;
        }        
        throwIfImmutable();
        let newa: Vec<Object> = new Object[newSz];
        let at: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let one: Object = self.arr[i];
            if one!=None            {
                newa[at]=one;
                at += 1;
            }            
        }
        self.arr=newa;
        if newSz==0        {
            setImmutable();
        }        
    }
    pub fn get0(&self, n: i32) -> Object    {
        try        {
            let result: Object = self.arr[n];
            if result==None            {
                throw NullPointerException::new("unset: "+n);
            }            
            return result;
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            return throwIndex(n);
        }
    }
    pub fn getOrNull0(&self, n: i32) -> Object    {
        return self.arr[n];
    }
    pub fn set0(&self, n: i32, obj: &Object)    {
        throwIfImmutable();
        try        {
            self.arr[n]=obj;
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
            throwIndex(n);
        }
    }
    pub fn throwIndex(&self, n: i32) -> Object    {
        if n<0        {
            throw IndexOutOfBoundsException::new("n < 0");
        }        
        throw IndexOutOfBoundsException::new("n >= size()");
    }
    pub fn toString0(&self, prefix: &String, separator: &String, suffix: &String, human: boolean) -> String    {
        let len: i32 = self.arr.len();
        let sb: StringBuilder = StringBuilder::new(len*10+10);
        if prefix!=None        {
            sb.append_String(&prefix);
        }        
        for(        let i: i32 = 0;;i<leni += 1)        {
            if (i!=0)&&(separator!=None)            {
                sb.append_String(&separator);
            }            
            if human            {
                sb.append_String(((ToHuman*)self.arr[i]).toHuman());
            }            else             {
                sb.append_Object(self.arr[i]);
            }
        }
        if suffix!=None        {
            sb.append_String(&suffix);
        }        
        return sb.toString();
    }
}
