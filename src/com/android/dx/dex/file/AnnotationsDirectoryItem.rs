use crate::helper;
use crate::com::android::dx::dex::file::ParameterAnnotationStruct;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::file::MethodAnnotationStruct;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::AnnotationsDirectoryItem;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::AnnotationSetItem;
use crate::com::android::dx::dex::file::FieldAnnotationStruct;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::file::OffsettedItem;

struct AnnotationsDirectoryItem{
    pub classAnnotations: AnnotationSetItem,
    pub fieldAnnotations: ArrayList<FieldAnnotationStruct>,
    pub methodAnnotations: ArrayList<MethodAnnotationStruct>,
    pub parameterAnnotations: ArrayList<ParameterAnnotationStruct>,
}
impl AnnotationsDirectoryItem{
    pub const ALIGNMENT: i32 = 4;
    pub const HEADER_SIZE: i32 = 16;
    pub const ELEMENT_SIZE: i32 = 8;
    pub fn new(&self)    {
        super(ALIGNMENT,-1);

        self.classAnnotations=None;
        self.fieldAnnotations=None;
        self.methodAnnotations=None;
        self.parameterAnnotations=None;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_ANNOTATIONS_DIRECTORY_ITEM;
    }
    pub fn isEmpty(&self) -> boolean    {
        return (self.classAnnotations==None)&&(self.fieldAnnotations==None)&&(self.methodAnnotations==None)&&(self.parameterAnnotations==None);
    }
    pub fn isInternable(&self) -> boolean    {
        return (self.classAnnotations!=None)&&(self.fieldAnnotations==None)&&(self.methodAnnotations==None)&&(self.parameterAnnotations==None);
    }
    pub fn hashCode(&self) -> i32    {
        if self.classAnnotations==None        {
            return 0;
        }        
        return self.classAnnotations.hashCode();
    }
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        if !isInternable()        {
            throw UnsupportedOperationException::new("uninternable instance");
        }        
        let otherDirectory: AnnotationsDirectoryItem = (AnnotationsDirectoryItem*)other;
        return self.classAnnotations.compareTo(&);
    }
    pub fn setClassAnnotations(&self, annotations: &Annotations, dexFile: &DexFile)    {
        if annotations==None        {
            throw NullPointerException::new("annotations == null");
        }        
        if self.classAnnotations!=None        {
            throw UnsupportedOperationException::new("class annotations already set");
        }        
        self.classAnnotations=AnnotationSetItem::new(&annotations, &dexFile);
    }
    pub fn addFieldAnnotations(&self, field: &CstFieldRef, annotations: &Annotations, dexFile: &DexFile)    {
        if self.fieldAnnotations==None        {
            self.fieldAnnotations=ArrayList<FieldAnnotationStruct>::new();
        }        
        self.fieldAnnotations.add_FieldAnnotationStruct(FieldAnnotationStruct::new(&field, AnnotationSetItem::new(&annotations, &dexFile)));
    }
    pub fn addMethodAnnotations(&self, method: &CstMethodRef, annotations: &Annotations, dexFile: &DexFile)    {
        if self.methodAnnotations==None        {
            self.methodAnnotations=ArrayList<MethodAnnotationStruct>::new();
        }        
        self.methodAnnotations.add_MethodAnnotationStruct(MethodAnnotationStruct::new(&method, AnnotationSetItem::new(&annotations, &dexFile)));
    }
    pub fn addParameterAnnotations(&self, method: &CstMethodRef, list: &AnnotationsList, dexFile: &DexFile)    {
        if self.parameterAnnotations==None        {
            self.parameterAnnotations=ArrayList<ParameterAnnotationStruct>::new();
        }        
        self.parameterAnnotations.add_ParameterAnnotationStruct(ParameterAnnotationStruct::new(&method, &list, &dexFile));
    }
    pub fn getMethodAnnotations(&self, method: &CstMethodRef) -> Annotations    {
        if self.methodAnnotations==None        {
            return None;
        }        
        for item in self.methodAnnotations        {
            if item.getMethod().equals(&method)            {
                return item.getAnnotations();
            }            
        }
        return None;
    }
    pub fn getParameterAnnotations(&self, method: &CstMethodRef) -> AnnotationsList    {
        if self.parameterAnnotations==None        {
            return None;
        }        
        for item in self.parameterAnnotations        {
            if item.getMethod().equals(&method)            {
                return item.getAnnotationsList();
            }            
        }
        return None;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let wordData: MixedItemSection = file.getWordData();
        if self.classAnnotations!=None        {
            self.classAnnotations=wordData.intern(&self.classAnnotations);
        }        
        if self.fieldAnnotations!=None        {
            for item in self.fieldAnnotations            {
                item.addContents(&file);
            }
        }        
        if self.methodAnnotations!=None        {
            for item in self.methodAnnotations            {
                item.addContents(&file);
            }
        }        
        if self.parameterAnnotations!=None        {
            for item in self.parameterAnnotations            {
                item.addContents(&file);
            }
        }        
    }
    pub fn toHuman(&self) -> String    {
        throw RuntimeException::new("unsupported");
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        let elementCount: i32 = AnnotationsDirectoryItem::listSize(&self.fieldAnnotations)+AnnotationsDirectoryItem::listSize(&self.methodAnnotations)+AnnotationsDirectoryItem::listSize(&self.parameterAnnotations);
        setWriteSize(AnnotationsDirectoryItem::HEADER_SIZE+(elementCount*AnnotationsDirectoryItem::ELEMENT_SIZE));
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        let classOff: i32 = OffsettedItem::getAbsoluteOffsetOr0(&self.classAnnotations);
        let fieldsSize: i32 = AnnotationsDirectoryItem::listSize(&self.fieldAnnotations);
        let methodsSize: i32 = AnnotationsDirectoryItem::listSize(&self.methodAnnotations);
        let parametersSize: i32 = AnnotationsDirectoryItem::listSize(&self.parameterAnnotations);
        if annotates        {
            out.annotate_int_String(0, offsetString()+" annotations directory");
            out.annotate_int_String(4, "  class_annotations_off: "+Hex::u4(classOff));
            out.annotate_int_String(4, "  fields_size:           "+Hex::u4(fieldsSize));
            out.annotate_int_String(4, "  methods_size:          "+Hex::u4(methodsSize));
            out.annotate_int_String(4, "  parameters_size:       "+Hex::u4(parametersSize));
        }        
        out.writeInt(classOff);
        out.writeInt(fieldsSize);
        out.writeInt(methodsSize);
        out.writeInt(parametersSize);
        if fieldsSize!=0        {
            Collections::sort_List<FieldAnnotationStruct>(&self.fieldAnnotations);
            if annotates            {
                out.annotate_int_String(0, "  fields:");
            }            
            for item in self.fieldAnnotations            {
                item.writeTo(&file, &out);
            }
        }        
        if methodsSize!=0        {
            Collections::sort_List<MethodAnnotationStruct>(&self.methodAnnotations);
            if annotates            {
                out.annotate_int_String(0, "  methods:");
            }            
            for item in self.methodAnnotations            {
                item.writeTo(&file, &out);
            }
        }        
        if parametersSize!=0        {
            Collections::sort_List<ParameterAnnotationStruct>(&self.parameterAnnotations);
            if annotates            {
                out.annotate_int_String(0, "  parameters:");
            }            
            for item in self.parameterAnnotations            {
                item.writeTo(&file, &out);
            }
        }        
    }
    pub fn listSize(list: &ArrayList<?>) -> i32    {
        if list==None        {
            return 0;
        }        
        return list.size();
    }
    pub fn debugPrint(&self, out: &PrintWriter)    {
        if self.classAnnotations!=None        {
            out.println_String("  class annotations: "+self.classAnnotations);
        }        
        if self.fieldAnnotations!=None        {
            out.println_String("  field annotations:");
            for item in self.fieldAnnotations            {
                out.println_String("    "+item.toHuman());
            }
        }        
        if self.methodAnnotations!=None        {
            out.println_String("  method annotations:");
            for item in self.methodAnnotations            {
                out.println_String("    "+item.toHuman());
            }
        }        
        if self.parameterAnnotations!=None        {
            out.println_String("  parameter annotations:");
            for item in self.parameterAnnotations            {
                out.println_String("    "+item.toHuman());
            }
        }        
    }
}
