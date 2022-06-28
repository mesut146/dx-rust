use crate::helper;
use crate::com::android::dx::cf::iface::ClassFile;
use crate::com::android::dx::cf::attrib::AttLocalVariableTable;
use crate::com::android::dx::cf::iface::AttributeList;
use crate::com::android::dx::cf::attrib::AttLocalVariableTypeTable;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::cf::code::LineNumberList;
use crate::com::android::dx::cf::code::LocalVariableList;
use crate::com::android::dx::cf::attrib::AttLineNumberTable;
use crate::com::android::dx::cf::attrib::AttCode;
use crate::com::android::dx::rop::cst::CstNat;

struct ConcreteMethod{
    pub method: Method,
    pub classFile: ClassFile,
    pub attCode: AttCode,
    pub lineNumbers: LineNumberList,
    pub localVariables: LocalVariableList,
}
impl ConcreteMethod{
    pub fn new(&self, method: &Method, classFile: &ClassFile, keepLines: boolean, keepLocals: boolean)    {
        self->method=method;
        self->classFile=classFile;
        let attribs: AttributeList = method.getAttributes();
        self->attCode=(AttCode*)attribs.findFirst(&AttCode::ATTRIBUTE_NAME);
        let codeAttribs: AttributeList = self.attCode.getAttributes();
        let lnl: LineNumberList = LineNumberList::EMPTY;
        if keepLines        {
            for(            let lnt: AttLineNumberTable = (AttLineNumberTable*)codeAttribs.findFirst(&AttLineNumberTable::ATTRIBUTE_NAME);;lnt!=Nonelnt=(AttLineNumberTable*)codeAttribs.findNext(&lnt))            {
                lnl=LineNumberList::concat(&lnl, lnt.getLineNumbers());
            }
        }        
        self->lineNumbers=lnl;
        let lvl: LocalVariableList = LocalVariableList::EMPTY;
        if keepLocals        {
            for(            let lvt: AttLocalVariableTable = (AttLocalVariableTable*)codeAttribs.findFirst(&AttLocalVariableTable::ATTRIBUTE_NAME);;lvt!=Nonelvt=(AttLocalVariableTable*)codeAttribs.findNext(&lvt))            {
                lvl=LocalVariableList::concat(&lvl, lvt.getLocalVariables());
            }
            let typeList: LocalVariableList = LocalVariableList::EMPTY;
            for(            let lvtt: AttLocalVariableTypeTable = (AttLocalVariableTypeTable*)codeAttribs.findFirst(&AttLocalVariableTypeTable::ATTRIBUTE_NAME);;lvtt!=Nonelvtt=(AttLocalVariableTypeTable*)codeAttribs.findNext(&lvtt))            {
                typeList=LocalVariableList::concat(&typeList, lvtt.getLocalVariables());
            }
            if typeList.size()!=0            {
                lvl=LocalVariableList::mergeDescriptorsAndSignatures(&lvl, &typeList);
            }            
        }        
        self->localVariables=lvl;
    }
    pub fn getSourceFile(&self) -> CstString    {
        return self.classFile.getSourceFile();
    }
    pub fn isDefaultOrStaticInterfaceMethod(&self) -> boolean    {
        return (self.classFile.getAccessFlags()&AccessFlags::ACC_INTERFACE)!=0&&!getNat().isClassInit();
    }
    pub fn isStaticMethod(&self) -> boolean    {
        return (getAccessFlags()&AccessFlags::ACC_STATIC)!=0;
    }
    pub fn getNat(&self) -> CstNat    {
        return self.method.getNat();
    }
    pub fn getName(&self) -> CstString    {
        return self.method.getName();
    }
    pub fn getDescriptor(&self) -> CstString    {
        return self.method.getDescriptor();
    }
    pub fn getAccessFlags(&self) -> i32    {
        return self.method.getAccessFlags();
    }
    pub fn getAttributes(&self) -> AttributeList    {
        return self.method.getAttributes();
    }
    pub fn getDefiningClass(&self) -> CstType    {
        return self.method.getDefiningClass();
    }
    pub fn getEffectiveDescriptor(&self) -> Prototype    {
        return self.method.getEffectiveDescriptor();
    }
    pub fn getMaxStack(&self) -> i32    {
        return self.attCode.getMaxStack();
    }
    pub fn getMaxLocals(&self) -> i32    {
        return self.attCode.getMaxLocals();
    }
    pub fn getCode(&self) -> BytecodeArray    {
        return self.attCode.getCode();
    }
    pub fn getCatches(&self) -> ByteCatchList    {
        return self.attCode.getCatches();
    }
    pub fn getLineNumbers(&self) -> LineNumberList    {
        return self.lineNumbers;
    }
    pub fn getLocalVariables(&self) -> LocalVariableList    {
        return self.localVariables;
    }
    pub fn makeSourcePosistion(&self, offset: i32) -> SourcePosition    {
        return SourcePosition::new(getSourceFile(), offset, self.lineNumbers.pcToLine(offset));
    }
}
