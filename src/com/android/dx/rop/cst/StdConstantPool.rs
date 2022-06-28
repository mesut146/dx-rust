use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::util::Hex;

struct StdConstantPool{
    pub entries: Vec<Constant>,
}
impl StdConstantPool{
    pub fn new(&self, size: i32)    {
        super(size > 1);

        if size<1        {
            throw IllegalArgumentException::new("size < 1");
        }        
        self.entries=new Constant[size];
    }
    pub fn size(&self) -> i32    {
        return self.entries.len();
    }
    pub fn getOrNull(&self, n: i32) -> Constant    {
        try        {
            return self.entries[n];
        }        catch(        let ex: IndexOutOfBoundsException)        {
            return StdConstantPool::throwInvalid(n);
        }
    }
    pub fn get0Ok(&self, n: i32) -> Constant    {
        if n==0        {
            return None;
        }        
        return get(n);
    }
    pub fn get(&self, n: i32) -> Constant    {
        try        {
            let result: Constant = self.entries[n];
            if result==None            {
                StdConstantPool::throwInvalid(n);
            }            
            return result;
        }        catch(        let ex: IndexOutOfBoundsException)        {
            return StdConstantPool::throwInvalid(n);
        }
    }
    pub fn getEntries(&self) -> Vec<Constant>    {
        return self.entries;
    }
    pub fn set(&self, n: i32, cst: &Constant)    {
        throwIfImmutable();
        let cat2: boolean = (cst!=None)&&cst.isCategory2();
        if n<1        {
            throw IllegalArgumentException::new("n < 1");
        }        
        if cat2        {
            if n==(self.entries.len()-1)            {
                throw IllegalArgumentException::new("(n == size - 1) && "+"cst.isCategory2()");
            }            
            self.entries[n+1]=None;
        }        
        if (cst!=None)&&(self.entries[n]==None)        {
            let prev: Constant = self.entries[n-1];
            if (prev!=None)&&prev.isCategory2()            {
                self.entries[n-1]=None;
            }            
        }        
        self.entries[n]=cst;
    }
    pub fn throwInvalid(idx: i32) -> Constant    {
        throw ExceptionWithContext::new("invalid constant pool index "+Hex::u2(idx));
    }
}
