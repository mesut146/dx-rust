use crate::helper;
use crate::com::android::dex::Dex;
use crate::com::android::dex::ClassData::Method;
use crate::com::android::dx::io::CodeReader;
use crate::com::android::dx::io::CodeReader::Visitor;
use crate::com::android::dex::MethodId;
use crate::com::android::dx::io::instructions::DecodedInstruction;
use crate::com::android::dex::ClassDef;
use crate::com::android::dex::EncodedValueReader;
use crate::com::android::dex::ClassData;
use crate::com::android::dex::Code;

struct Grep{
    pub dex: Dex,
    pub codeReader: CodeReader,
    pub stringIds: Set<Integer>,
    pub out: PrintWriter,
    pub count: i32,
    pub currentClass: ClassDef,
    pub currentMethod: ClassData.Method,
}
impl Grep{
    pub fn new(&self, dex: &Dex, pattern: &Pattern, out: &PrintWriter)    {
        self->dex=dex;
        self->out=out;
        self.stringIds=getStringIds(&dex, &pattern);
        self.codeReader.setStringVisitor(/*new CodeReader.Visitor(){
  @Override public void visit(  DecodedInstruction[] all,  DecodedInstruction one){
    encounterString(one.getIndex());
  }
}
*/);
    }
    pub fn readArray(&self, reader: &EncodedValueReader)    {
        for(        let i: i32 = 0;        let size: i32 = reader.readArray();;i<sizei += 1)        {
            match reader.peek(){EncodedValueReader::ENCODED_STRING =>                 encounterString(reader.readString());                break;EncodedValueReader::ENCODED_ARRAY =>                 readArray(&reader);                break;            }
        }
    }
    pub fn encounterString(&self, index: i32)    {
        if self.stringIds.contains(index)        {
            self.out.println_String(location()+" "+self.dex.strings().get(index));
            self.count += 1;
        }        
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
    pub fn grep(&self) -> i32    {
        for classDef in self.dex.classDefs()        {
            self.currentClass=classDef;
            self.currentMethod=None;
            if classDef.getClassDataOffset()==0            {
                continue;
            }            
            let classData: ClassData = self.dex.readClassData(&classDef);
            let staticValuesOffset: i32 = classDef.getStaticValuesOffset();
            if staticValuesOffset!=0            {
                readArray(EncodedValueReader::new(self.dex.open(staticValuesOffset)));
            }            
            for method in classData.allMethods()            {
                self.currentMethod=method;
                if method.getCodeOffset()!=0                {
                    self.codeReader.visitAll_short[](self.dex.readCode(&method).getInstructions());
                }                
            }
        }
        self.currentClass=None;
        self.currentMethod=None;
        return self.count;
    }
    pub fn getStringIds(&self, dex: &Dex, pattern: &Pattern) -> Set<Integer>    {
        let stringIds: Set<Integer> = HashSet<Integer>::new();
        let stringIndex: i32 = 0;
        for s in dex.strings()        {
            if pattern.matcher(&s).find()            {
                stringIds.add(stringIndex);
            }            
            stringIndex += 1;
        }
        return stringIds;
    }
}
