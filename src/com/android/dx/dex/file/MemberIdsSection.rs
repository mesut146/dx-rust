use crate::helper;
use crate::com::android::dex::DexFormat;
use crate::com::android::dex::DexIndexOverflowException;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::dex::file::MemberIdItem;

struct MemberIdsSection{
}
impl MemberIdsSection{
    pub fn new(&self, name: &String, file: &DexFile)    {
        super(name,file,4);

    }
    pub fn orderItems(&self)    {
        let idx: i32 = 0;
        if items().size()>DexFormat::MAX_MEMBER_IDX+1        {
            throw DexIndexOverflowException::new(getTooManyMembersMessage());
        }        
        for i in items()        {
            ((MemberIdItem*)i).setIndex(idx);
            idx += 1;
        }
    }
    pub fn getTooManyMembersMessage(&self) -> String    {
        let membersByPackage: Map<String,AtomicInteger> = TreeMap<String,AtomicInteger>::new();
        for member in items()        {
            let packageName: String = ((MemberIdItem*)member).getDefiningClass().getPackageName();
            let count: AtomicInteger = membersByPackage.get(&packageName);
            if count==None            {
                count=AtomicInteger::new();
                membersByPackage.put(&packageName, &count);
            }            
            count.incrementAndGet();
        }
        let formatter: Formatter = Formatter::new();
        try        {
            let memberType: String = if //this instanceof MethodIdsSection { "method" } else { "field" };
                    formatter.format_String_Object[]("Too many %1$s references to fit in one dex file: %2$d; max is %3$d.%n"+"You may try using multi-dex. If multi-dex is enabled then the list of "+"classes for the main dex list is too large.%n"+"References by package:", &memberType, items().size(), DexFormat::MAX_MEMBER_IDX+1);
                    for entry in membersByPackage.entrySet()                    {
                        formatter.format_String_Object[]("%n%6d %s", entry.getValue().get(), entry.getKey());
                    }
                    return formatter.toString();
                }                finally                {
                    formatter.close();
                }
            }
}
