use crate::helper;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::type::Type;

let static TYPE_ArithmeticException: Type = Type::intern("Ljava/lang/ArithmeticException;");
let static TYPE_ArrayIndexOutOfBoundsException: Type = Type::intern("Ljava/lang/ArrayIndexOutOfBoundsException;");
let static TYPE_ArrayStoreException: Type = Type::intern("Ljava/lang/ArrayStoreException;");
let static TYPE_ClassCastException: Type = Type::intern("Ljava/lang/ClassCastException;");
let static TYPE_Error: Type = Type::intern("Ljava/lang/Error;");
let static TYPE_IllegalMonitorStateException: Type = Type::intern("Ljava/lang/IllegalMonitorStateException;");
let static TYPE_NegativeArraySizeException: Type = Type::intern("Ljava/lang/NegativeArraySizeException;");
let static TYPE_NullPointerException: Type = Type::intern("Ljava/lang/NullPointerException;");
let static LIST_Error: StdTypeList = StdTypeList::make_Type(&Exceptions::TYPE_Error);
let static LIST_Error_ArithmeticException: StdTypeList = StdTypeList::make_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_ArithmeticException);
let static LIST_Error_ClassCastException: StdTypeList = StdTypeList::make_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_ClassCastException);
let static LIST_Error_NegativeArraySizeException: StdTypeList = StdTypeList::make_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_NegativeArraySizeException);
let static LIST_Error_NullPointerException: StdTypeList = StdTypeList::make_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_NullPointerException);
let static LIST_Error_Null_ArrayIndexOutOfBounds: StdTypeList = StdTypeList::make_Type_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_NullPointerException, &Exceptions::TYPE_ArrayIndexOutOfBoundsException);
let static LIST_Error_Null_ArrayIndex_ArrayStore: StdTypeList = StdTypeList::make_Type_Type_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_NullPointerException, &Exceptions::TYPE_ArrayIndexOutOfBoundsException, &Exceptions::TYPE_ArrayStoreException);
let static LIST_Error_Null_IllegalMonitorStateException: StdTypeList = StdTypeList::make_Type_Type_Type(&Exceptions::TYPE_Error, &Exceptions::TYPE_NullPointerException, &Exceptions::TYPE_IllegalMonitorStateException);
struct Exceptions{
}
impl Exceptions{
    pub fn new(&self)    {
    }
}
