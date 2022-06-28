use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dex::ClassData::Method;
use crate::com::android::dex::FieldId;
use crate::com::android::dx::io::CodeReader;
use crate::com::android::dx::io::CodeReader::Visitor;
use crate::com::android::dex::MethodId;
use crate::com::android::dex::ClassData::Field;
use crate::com::android::dx::io::instructions::DecodedInstruction;
use crate::com::android::dex::ClassDef;
use crate::com::android::dx::io::OpcodeInfo;
use crate::com::android::dex::ClassData;
use crate::com::android::dex::Code;

struct FindUsages{
    pub dex: Dex,
    pub methodIds: Set<Integer>,
    pub fieldIds: Set<Integer>,
    pub codeReader: CodeReader,
    pub out: PrintWriter,
    pub currentClass: ClassDef,
    pub currentMethod: ClassData.Method,
}
impl FindUsages{
    pub fn new(&self, dex: &Dex, declaredBy: &String, memberName: &String, out: &PrintWriter)    {
        self->dex=dex;
        self->out=out;
        let typeStringIndexes: Set<Integer> = HashSet<Integer>::new();
        let memberNameIndexes: Set<Integer> = HashSet<Integer>::new();
        let declaredByPattern: Pattern = Pattern::compile_String(&declaredBy);
        let memberNamePattern: Pattern = Pattern::compile_String(&memberName);
        let strings: List<String> = dex.strings();
        for(        let i: i32 = 0;;i<strings.size()++i)        {
            let string: String = strings.get(i);
            if declaredByPattern.matcher(&string).matches()            {
                typeStringIndexes.add(i);
            }            
            if memberNamePattern.matcher(&string).matches()            {
                memberNameIndexes.add(i);
            }            
        }
        if typeStringIndexes.isEmpty()||memberNameIndexes.isEmpty()        {
            self.methodIds=self.fieldIds=None;
            return;
        }        
        self.methodIds=HashSet<Integer>::new();
        self.fieldIds=HashSet<Integer>::new();
        for typeStringIndex in typeStringIndexes        {
            let typeIndex: i32 = Collections::binarySearch_List<? extends Comparable<? super Integer>>_Integer(dex.typeIds(), typeStringIndex);
            if typeIndex<0            {
                continue;
            }            
            self.methodIds.addAll(getMethodIds(&dex, &memberNameIndexes, typeIndex));
            self.fieldIds.addAll(getFieldIds(&dex, &memberNameIndexes, typeIndex));
        }
        self.codeReader.setFieldVisitor(/*new CodeReader.Visitor(){
  @Override public void visit(  DecodedInstruction[] all,  DecodedInstruction one){
    int fieldId=one.getIndex();
    if (fieldIds.contains(fieldId)) {
      out.println(location() + ": field reference " + dex.fieldIds().get(fieldId)+ " ("+ OpcodeInfo.getName(one.getOpcode())+ ")");
    }
  }
}
*/);
        self.codeReader.setMethodVisitor(/*new CodeReader.Visitor(){
  @Override public void visit(  DecodedInstruction[] all,  DecodedInstruction one){
    int methodId=one.getIndex();
    if (methodIds.contains(methodId)) {
      out.println(location() + ": method reference " + dex.methodIds().get(methodId)+ " ("+ OpcodeInfo.getName(one.getOpcode())+ ")");
    }
  }
}
*/);
    }
    pub fn location(&self) -> String    {
        let className: String = self.dex.typeNames().get(self.currentClass.getTypeIndex());
        if self.currentMethod!=None        {
            let methodId: MethodId = self.dex.methodIds().get(self.currentMethod.getMethodIndex());
            return className+"."+self.dex.strings().get(methodId.getNameIndex());
        }        else         {
            return className;
        }
    }
    pub fn findUsages(&self)    {
        if self.fieldIds==None||self.methodIds==None        {
            return;
        }        
        for classDef in self.dex.classDefs()        {
            self.currentClass=classDef;
            self.currentMethod=None;
            if classDef.getClassDataOffset()==0            {
                continue;
            }            
            let classData: ClassData = self.dex.readClassData(&classDef);
            for field in classData.allFields()            {
                let fieldIndex: i32 = field.getFieldIndex();
                if self.fieldIds.contains(fieldIndex)                {
                    self.out.println_String(location()+" field declared "+self.dex.fieldIds().get(fieldIndex));
                }                
            }
            for method in classData.allMethods()            {
                self.currentMethod=method;
                let methodIndex: i32 = method.getMethodIndex();
                if self.methodIds.contains(methodIndex)                {
                    self.out.println_String(location()+" method declared "+self.dex.methodIds().get(methodIndex));
                }                
                if method.getCodeOffset()!=0                {
                    self.codeReader.visitAll_short[](self.dex.readCode(&method).getInstructions());
                }                
            }
        }
        self.currentClass=None;
        self.currentMethod=None;
    }
    pub fn getFieldIds(&self, dex: &Dex, memberNameIndexes: &Set<Integer>, declaringType: i32) -> Set<Integer>    {
        let fields: Set<Integer> = HashSet<Integer>::new();
        let fieldIndex: i32 = 0;
        for fieldId in dex.fieldIds()        {
            if memberNameIndexes.contains(fieldId.getNameIndex())&&declaringType==fieldId.getDeclaringClassIndex()            {
                fields.add(fieldIndex);
            }            
            fieldIndex += 1;
        }
        return fields;
    }
    pub fn getMethodIds(&self, dex: &Dex, memberNameIndexes: &Set<Integer>, declaringType: i32) -> Set<Integer>    {
        let subtypes: Set<Integer> = findAssignableTypes(&dex, declaringType);
        let methods: Set<Integer> = HashSet<Integer>::new();
        let methodIndex: i32 = 0;
        for method in dex.methodIds()        {
            if memberNameIndexes.contains(method.getNameIndex())&&subtypes.contains(method.getDeclaringClassIndex())            {
                methods.add(methodIndex);
            }            
            methodIndex += 1;
        }
        return methods;
    }
    pub fn findAssignableTypes(&self, dex: &Dex, typeIndex: i32) -> Set<Integer>    {
        let assignableTypes: Set<Integer> = HashSet<Integer>::new();
        assignableTypes.add(typeIndex);
        for classDef in dex.classDefs()        {
            if assignableTypes.contains(classDef.getSupertypeIndex())            {
                assignableTypes.add(classDef.getTypeIndex());
                continue;
            }            
            for implemented in classDef.getInterfaces()            {
                if assignableTypes.contains(implemented)                {
                    assignableTypes.add(classDef.getTypeIndex());
                    break;
                }                
            }
        }
        return assignableTypes;
    }
}
