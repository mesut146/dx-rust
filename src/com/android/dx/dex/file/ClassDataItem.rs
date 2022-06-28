use crate::helper;
use crate::com::android::dx::dex::file::EncodedField;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::util::Writers;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::rop::cst::CstArray::List;
use crate::;
use crate::com::android::dx::dex::file::EncodedMethod;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::rop::cst::CstArray;
use crate::com::android::dx::rop::cst::Zeroes;

struct ClassDataItem{
    pub thisClass: CstType,
    pub staticFields: ArrayList<EncodedField>,
    pub staticValues: HashMap<EncodedField,Constant>,
    pub instanceFields: ArrayList<EncodedField>,
    pub directMethods: ArrayList<EncodedMethod>,
    pub virtualMethods: ArrayList<EncodedMethod>,
    pub staticValuesConstant: CstArray,
    pub encodedForm: Vec<i8>,
}
impl ClassDataItem{
    pub fn new(&self, thisClass: &CstType)    {
        super(1,-1);

        if thisClass==None        {
            throw NullPointerException::new("thisClass == null");
        }        
        self->thisClass=thisClass;
        self->staticFields=ArrayList<EncodedField>::new(20);
        self->staticValues=HashMap<EncodedField,Constant>::new(40);
        self->instanceFields=ArrayList<EncodedField>::new(20);
        self->directMethods=ArrayList<EncodedMethod>::new(20);
        self->virtualMethods=ArrayList<EncodedMethod>::new(20);
        self->staticValuesConstant=None;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_CLASS_DATA_ITEM;
    }
    pub fn toHuman(&self) -> String    {
        return toString();
    }
    pub fn isEmpty(&self) -> boolean    {
        return self.staticFields.isEmpty()&&self.instanceFields.isEmpty()&&self.directMethods.isEmpty()&&self.virtualMethods.isEmpty();
    }
    pub fn addStaticField(&self, field: &EncodedField, value: &Constant)    {
        if field==None        {
            throw NullPointerException::new("field == null");
        }        
        if self.staticValuesConstant!=None        {
            throw UnsupportedOperationException::new("static fields already sorted");
        }        
        self.staticFields.add_EncodedField(&field);
        self.staticValues.put(&field, &value);
    }
    pub fn addInstanceField(&self, field: &EncodedField)    {
        if field==None        {
            throw NullPointerException::new("field == null");
        }        
        self.instanceFields.add_EncodedField(&field);
    }
    pub fn addDirectMethod(&self, method: &EncodedMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        self.directMethods.add_EncodedMethod(&method);
    }
    pub fn addVirtualMethod(&self, method: &EncodedMethod)    {
        if method==None        {
            throw NullPointerException::new("method == null");
        }        
        self.virtualMethods.add_EncodedMethod(&method);
    }
    pub fn getMethods(&self) -> ArrayList<EncodedMethod>    {
        let sz: i32 = self.directMethods.size()+self.virtualMethods.size();
        let result: ArrayList<EncodedMethod> = ArrayList<EncodedMethod>::new(sz);
        result.addAll_Collection<? extends EncodedMethod>(&self.directMethods);
        result.addAll_Collection<? extends EncodedMethod>(&self.virtualMethods);
        return result;
    }
    pub fn debugPrint(&self, out: &Writer, verbose: boolean)    {
        let pw: PrintWriter = Writers::printWriterFor(&out);
        let sz: i32 = self.staticFields.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            pw.println_String("  sfields["+i+"]: "+self.staticFields.get(i));
        }
        sz=self.instanceFields.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            pw.println_String("  ifields["+i+"]: "+self.instanceFields.get(i));
        }
        sz=self.directMethods.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            pw.println_String("  dmeths["+i+"]:");
            self.directMethods.get(i).debugPrint(&pw, verbose);
        }
        sz=self.virtualMethods.size();
        for(        let i: i32 = 0;;i<szi += 1)        {
            pw.println_String("  vmeths["+i+"]:");
            self.virtualMethods.get(i).debugPrint(&pw, verbose);
        }
    }
    pub fn addContents(&self, file: &DexFile)    {
        if !self.staticFields.isEmpty()        {
            getStaticValuesConstant();
            for field in self.staticFields            {
                field.addContents(&file);
            }
        }        
        if !self.instanceFields.isEmpty()        {
            Collections::sort_List<EncodedField>(&self.instanceFields);
            for field in self.instanceFields            {
                field.addContents(&file);
            }
        }        
        if !self.directMethods.isEmpty()        {
            Collections::sort_List<EncodedMethod>(&self.directMethods);
            for method in self.directMethods            {
                method.addContents(&file);
            }
        }        
        if !self.virtualMethods.isEmpty()        {
            Collections::sort_List<EncodedMethod>(&self.virtualMethods);
            for method in self.virtualMethods            {
                method.addContents(&file);
            }
        }        
    }
    pub fn getStaticValuesConstant(&self) -> CstArray    {
        if (self.staticValuesConstant==None)&&(self.staticFields.size()!=0)        {
            self.staticValuesConstant=makeStaticValuesConstant();
        }        
        return self.staticValuesConstant;
    }
    pub fn makeStaticValuesConstant(&self) -> CstArray    {
        Collections::sort_List<EncodedField>(&self.staticFields);
        let size: i32 = self.staticFields.size();
        while size>0        {
            let field: EncodedField = self.staticFields.get(size-1);
            let cst: Constant = self.staticValues.get(&field);
            if //cst instanceof CstLiteralBits            {
                if ((CstLiteralBits*)cst).getLongBits()!=0                {
                    break;
                }                
            }            else             if cst!=None            {
                break;
            }            
            size -= 1;
        }
        if size==0        {
            return None;
        }        
        let list: List = CstArray.List::new(size);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            let field: EncodedField = self.staticFields.get(i);
            let cst: Constant = self.staticValues.get(&field);
            if cst==None            {
                cst=Zeroes::zeroFor(field.getRef().getType());
            }            
            list.set(i, &cst);
        }
        list.setImmutable();
        return CstArray::new(&list);
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new();
        encodeOutput(addedTo.getFile(), &out);
        self.encodedForm=out.toByteArray();
        setWriteSize(self.encodedForm.len());
    }
    pub fn encodeOutput(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        if annotates        {
            out.annotate_int_String(0, offsetString()+" class data for "+self.thisClass.toHuman());
        }        
        ClassDataItem::encodeSize(&file, &out, "static_fields", self.staticFields.size());
        ClassDataItem::encodeSize(&file, &out, "instance_fields", self.instanceFields.size());
        ClassDataItem::encodeSize(&file, &out, "direct_methods", self.directMethods.size());
        ClassDataItem::encodeSize(&file, &out, "virtual_methods", self.virtualMethods.size());
        ClassDataItem::encodeList(&file, &out, "static_fields", &self.staticFields);
        ClassDataItem::encodeList(&file, &out, "instance_fields", &self.instanceFields);
        ClassDataItem::encodeList(&file, &out, "direct_methods", &self.directMethods);
        ClassDataItem::encodeList(&file, &out, "virtual_methods", &self.virtualMethods);
        if annotates        {
            out.endAnnotation();
        }        
    }
    pub fn encodeSize(file: &DexFile, out: &AnnotatedOutput, label: &String, size: i32)    {
        if out.annotates()        {
            out.annotate_String(String::format_String_Object[]("  %-21s %08x", label+"_size:", size));
        }        
        out.writeUleb128(size);
    }
    pub fn encodeList(file: &DexFile, out: &AnnotatedOutput, label: &String, list: &ArrayList<? extends EncodedMember>)    {
        let size: i32 = list.size();
        let lastIndex: i32 = 0;
        if size==0        {
            return;
        }        
        if out.annotates()        {
            out.annotate_int_String(0, "  "+label+":");
        }        
        for(        let i: i32 = 0;;i<sizei += 1)        {
            lastIndex=list.get(i).encode(&file, &out, lastIndex, i);
        }
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        if annotates        {
            encodeOutput(&file, &out);
        }        else         {
            out.write_byte[](&self.encodedForm);
        }
    }
}
