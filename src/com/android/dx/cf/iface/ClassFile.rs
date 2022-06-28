use crate::helper;

trait ClassFile{
    pub fn getMagic(&self) -> i32;
    pub fn getMinorVersion(&self) -> i32;
    pub fn getMajorVersion(&self) -> i32;
    pub fn getAccessFlags(&self) -> i32;
    pub fn getThisClass(&self) -> CstType;
    pub fn getSuperclass(&self) -> CstType;
    pub fn getConstantPool(&self) -> ConstantPool;
    pub fn getInterfaces(&self) -> TypeList;
    pub fn getFields(&self) -> FieldList;
    pub fn getMethods(&self) -> MethodList;
    pub fn getAttributes(&self) -> AttributeList;
    pub fn getBootstrapMethods(&self) -> BootstrapMethodsList;
    pub fn getSourceFile(&self) -> CstString;
}
