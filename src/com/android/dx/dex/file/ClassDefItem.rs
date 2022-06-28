use crate::helper;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::dex::file::EncodedArrayItem;
use crate::com::android::dx::dex::file::ClassDataItem;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::AnnotationsDirectoryItem;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::util::Writers;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::file::OffsettedItem;
use crate::com::android::dx::dex::file::TypeListItem;
use crate::com::android::dx::rop::type::Type;

struct ClassDefItem{
    pub thisClass: CstType,
    pub accessFlags: i32,
    pub superclass: CstType,
    pub interfaces: TypeListItem,
    pub sourceFile: CstString,
    pub classData: ClassDataItem,
    pub staticValuesItem: EncodedArrayItem,
    pub annotationsDirectory: AnnotationsDirectoryItem,
}
impl ClassDefItem{
    pub fn new(&self, thisClass: &CstType, accessFlags: i32, superclass: &CstType, interfaces: &TypeList, sourceFile: &CstString)    {
        if thisClass==None        {
            throw NullPointerException::new("thisClass == null");
        }        
        if interfaces==None        {
            throw NullPointerException::new("interfaces == null");
        }        
        self->thisClass=thisClass;
        self->accessFlags=accessFlags;
        self->superclass=superclass;
        self->interfaces=if (interfaces.size()==0) { None } else { TypeListItem::new(&interfaces) };
                self->sourceFile=sourceFile;
                self->classData=ClassDataItem::new(&thisClass);
                self->staticValuesItem=None;
                self->annotationsDirectory=AnnotationsDirectoryItem::new();
            }
            pub fn itemType(&self) -> ItemType            {
                return ItemType::TYPE_CLASS_DEF_ITEM;
            }
            pub fn writeSize(&self) -> i32            {
                return SizeOf::CLASS_DEF_ITEM;
            }
            pub fn addContents(&self, file: &DexFile)            {
                let typeIds: TypeIdsSection = file.getTypeIds();
                let byteData: MixedItemSection = file.getByteData();
                let wordData: MixedItemSection = file.getWordData();
                let typeLists: MixedItemSection = file.getTypeLists();
                let stringIds: StringIdsSection = file.getStringIds();
                typeIds.intern_CstType(&self.thisClass);
                if !self.classData.isEmpty()                {
                    let classDataSection: MixedItemSection = file.getClassData();
                    classDataSection.add(&self.classData);
                    let staticValues: CstArray = self.classData.getStaticValuesConstant();
                    if staticValues!=None                    {
                        self.staticValuesItem=byteData.intern(EncodedArrayItem::new(&staticValues));
                    }                    
                }                
                if self.superclass!=None                {
                    typeIds.intern_CstType(&self.superclass);
                }                
                if self.interfaces!=None                {
                    self.interfaces=typeLists.intern(&self.interfaces);
                }                
                if self.sourceFile!=None                {
                    stringIds.intern_CstString(&self.sourceFile);
                }                
                if !self.annotationsDirectory.isEmpty()                {
                    if self.annotationsDirectory.isInternable()                    {
                        self.annotationsDirectory=wordData.intern(&self.annotationsDirectory);
                    }                    else                     {
                        wordData.add(&self.annotationsDirectory);
                    }
                }                
            }
            pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)            {
                let annotates: boolean = out.annotates();
                let typeIds: TypeIdsSection = file.getTypeIds();
                let classIdx: i32 = typeIds.indexOf_CstType(&self.thisClass);
                let superIdx: i32 = if (self.superclass==None) { -1 } else { typeIds.indexOf_CstType(&self.superclass) };
                        let interOff: i32 = OffsettedItem::getAbsoluteOffsetOr0(&self.interfaces);
                        let annoOff: i32 = if self.annotationsDirectory.isEmpty() { 0 } else { self.annotationsDirectory.getAbsoluteOffset() };
                                let sourceFileIdx: i32 = if (self.sourceFile==None) { -1 } else { file.getStringIds().indexOf(&self.sourceFile) };
                                        let dataOff: i32 = if self.classData.isEmpty() { 0 } else { self.classData.getAbsoluteOffset() };
                                                let staticValuesOff: i32 = OffsettedItem::getAbsoluteOffsetOr0(&self.staticValuesItem);
                                                if annotates                                                {
                                                    out.annotate_int_String(0, indexString()+' '+self.thisClass.toHuman());
                                                    out.annotate_int_String(4, "  class_idx:           "+Hex::u4(classIdx));
                                                    out.annotate_int_String(4, "  access_flags:        "+AccessFlags::classString(self.accessFlags));
                                                    out.annotate_int_String(4, "  superclass_idx:      "+Hex::u4(superIdx)+" // "+(if (self.superclass==None) { "<none>" } else { self.superclass.toHuman() }));
                                                            out.annotate_int_String(4, "  interfaces_off:      "+Hex::u4(interOff));
                                                            if interOff!=0                                                            {
                                                                let list: TypeList = self.interfaces.getList();
                                                                let sz: i32 = list.size();
                                                                for(                                                                let i: i32 = 0;;i<szi += 1)                                                                {
                                                                    out.annotate_int_String(0, "    "+list.getType(i).toHuman());
                                                                }
                                                            }                                                            
                                                            out.annotate_int_String(4, "  source_file_idx:     "+Hex::u4(sourceFileIdx)+" // "+(if (self.sourceFile==None) { "<none>" } else { self.sourceFile.toHuman() }));
                                                                    out.annotate_int_String(4, "  annotations_off:     "+Hex::u4(annoOff));
                                                                    out.annotate_int_String(4, "  class_data_off:      "+Hex::u4(dataOff));
                                                                    out.annotate_int_String(4, "  static_values_off:   "+Hex::u4(staticValuesOff));
                                                                }                                                                
                                                                out.writeInt(classIdx);
                                                                out.writeInt(self.accessFlags);
                                                                out.writeInt(superIdx);
                                                                out.writeInt(interOff);
                                                                out.writeInt(sourceFileIdx);
                                                                out.writeInt(annoOff);
                                                                out.writeInt(dataOff);
                                                                out.writeInt(staticValuesOff);
                                                            }
                                                            pub fn getThisClass(&self) -> CstType                                                            {
                                                                return self.thisClass;
                                                            }
                                                            pub fn getAccessFlags(&self) -> i32                                                            {
                                                                return self.accessFlags;
                                                            }
                                                            pub fn getSuperclass(&self) -> CstType                                                            {
                                                                return self.superclass;
                                                            }
                                                            pub fn getInterfaces(&self) -> TypeList                                                            {
                                                                if self.interfaces==None                                                                {
                                                                    return StdTypeList::EMPTY;
                                                                }                                                                
                                                                return self.interfaces.getList();
                                                            }
                                                            pub fn getSourceFile(&self) -> CstString                                                            {
                                                                return self.sourceFile;
                                                            }
                                                            pub fn addStaticField(&self, field: &EncodedField, value: &Constant)                                                            {
                                                                self.classData.addStaticField(&field, &value);
                                                            }
                                                            pub fn addInstanceField(&self, field: &EncodedField)                                                            {
                                                                self.classData.addInstanceField(&field);
                                                            }
                                                            pub fn addDirectMethod(&self, method: &EncodedMethod)                                                            {
                                                                self.classData.addDirectMethod(&method);
                                                            }
                                                            pub fn addVirtualMethod(&self, method: &EncodedMethod)                                                            {
                                                                self.classData.addVirtualMethod(&method);
                                                            }
                                                            pub fn getMethods(&self) -> ArrayList<EncodedMethod>                                                            {
                                                                return self.classData.getMethods();
                                                            }
                                                            pub fn setClassAnnotations(&self, annotations: &Annotations, dexFile: &DexFile)                                                            {
                                                                self.annotationsDirectory.setClassAnnotations(&annotations, &dexFile);
                                                            }
                                                            pub fn addFieldAnnotations(&self, field: &CstFieldRef, annotations: &Annotations, dexFile: &DexFile)                                                            {
                                                                self.annotationsDirectory.addFieldAnnotations(&field, &annotations, &dexFile);
                                                            }
                                                            pub fn addMethodAnnotations(&self, method: &CstMethodRef, annotations: &Annotations, dexFile: &DexFile)                                                            {
                                                                self.annotationsDirectory.addMethodAnnotations(&method, &annotations, &dexFile);
                                                            }
                                                            pub fn addParameterAnnotations(&self, method: &CstMethodRef, list: &AnnotationsList, dexFile: &DexFile)                                                            {
                                                                self.annotationsDirectory.addParameterAnnotations(&method, &list, &dexFile);
                                                            }
                                                            pub fn getMethodAnnotations(&self, method: &CstMethodRef) -> Annotations                                                            {
                                                                return self.annotationsDirectory.getMethodAnnotations(&method);
                                                            }
                                                            pub fn getParameterAnnotations(&self, method: &CstMethodRef) -> AnnotationsList                                                            {
                                                                return self.annotationsDirectory.getParameterAnnotations(&method);
                                                            }
                                                            pub fn debugPrint(&self, out: &Writer, verbose: boolean)                                                            {
                                                                let pw: PrintWriter = Writers::printWriterFor(&out);
                                                                pw.println_String(getClass().getName()+" {");
                                                                pw.println_String("  accessFlags: "+Hex::u2(self.accessFlags));
                                                                pw.println_String("  superclass: "+self.superclass);
                                                                pw.println_String("  interfaces: "+(if (self.interfaces==None) { "<none>" } else { self.interfaces }));
                                                                        pw.println_String("  sourceFile: "+(if (self.sourceFile==None) { "<none>" } else { self.sourceFile.toQuoted() }));
                                                                                self.classData.debugPrint(&out, verbose);
                                                                                self.annotationsDirectory.debugPrint(&pw);
                                                                                pw.println_String("}");
                                                                            }
}
