use crate::helper;
use crate::;
use crate::com::android::dx::merge::InstructionTransformer;
use crate::com::android::dex::FieldId;
use crate::;
use crate::com::android::dex::MethodHandle;
use crate::com::android::dx::command::dexer::DxContext;
use crate::;
use crate::com::android::dex::TableOfContents;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::TypeList>;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::MethodHandle>;
use crate::com::android::dex::Code::CatchHandler;
use crate::;
use crate::com::android::dex::Code;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::MethodId>;
use crate::com::android::dx::merge::DexMerger::WriterSizes;
use crate::com::android::dex::ClassData::Method;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::FieldId>;
use crate::com::android::dx::merge::DexMerger::IdMerger<java::lang::Integer>;
use crate::com::android::dx::merge::IndexMap;
use crate::com::android::dex::MethodId;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::Annotation>;
use crate::com::android::dex::ClassData::Field;
use crate::com::android::dex::EncodedValue;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::CallSiteId>;
use crate::com::android::dex::TableOfContents::Section;
use crate::com::android::dex::ClassData;
use crate::T;
use crate::;
use crate::com::android::dex::Dex::Section;
use crate::;
use crate::com::android::dex::CallSiteId;
use crate::com::android::dx::merge::SortableType;
use crate::com::android::dx::merge::DexMerger;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::merge::CollisionPolicy;
use crate::com::android::dex::Annotation;
use crate::com::android::dx::merge::DexMerger::IdMerger<T>::UnsortedValue;
use crate::com::android::dex::Code::Try;
use crate::com::android::dx::merge::DexMerger::IdMerger<com::android::dex::ProtoId>;
use crate::com::android::dex::ProtoId;
use crate::com::android::dex::Dex;
use crate::;
use crate::;
use crate::com::android::dx::merge::DexMerger::IdMerger<java::lang::String>;
use crate::;
use crate::com::android::dex::ClassDef;
use crate::com::android::dex::DexIndexOverflowException;
use crate::com::android::dex::DexException;

struct DexMerger{
    pub dexes: Vec<Dex>,
    pub indexMaps: Vec<IndexMap>,
    pub collisionPolicy: CollisionPolicy,
    pub context: DxContext,
    pub writerSizes: WriterSizes,
    pub dexOut: Dex,
    pub headerOut: Dex.Section,
    pub idsDefsOut: Dex.Section,
    pub mapListOut: Dex.Section,
    pub typeListOut: Dex.Section,
    pub classDataOut: Dex.Section,
    pub codeOut: Dex.Section,
    pub stringDataOut: Dex.Section,
    pub debugInfoOut: Dex.Section,
    pub encodedArrayOut: Dex.Section,
    pub annotationsDirectoryOut: Dex.Section,
    pub annotationSetOut: Dex.Section,
    pub annotationSetRefListOut: Dex.Section,
    pub annotationOut: Dex.Section,
    pub contentsOut: TableOfContents,
    pub instructionTransformer: InstructionTransformer,
    pub compactWasteThreshold: i32,
}
impl DexMerger{
    pub const DBG_END_SEQUENCE: i8 = 0x00;
    pub const DBG_ADVANCE_PC: i8 = 0x01;
    pub const DBG_ADVANCE_LINE: i8 = 0x02;
    pub const DBG_START_LOCAL: i8 = 0x03;
    pub const DBG_START_LOCAL_EXTENDED: i8 = 0x04;
    pub const DBG_END_LOCAL: i8 = 0x05;
    pub const DBG_RESTART_LOCAL: i8 = 0x06;
    pub const DBG_SET_PROLOGUE_END: i8 = 0x07;
    pub const DBG_SET_EPILOGUE_BEGIN: i8 = 0x08;
    pub const DBG_SET_FILE: i8 = 0x09;
    pub fn new(&self, dexes: &Vec<Dex>, collisionPolicy: &CollisionPolicy, context: &DxContext)    {
        this(dexes,collisionPolicy,context,new WriterSizes(dexes));

    }
    pub fn new(&self, dexes: &Vec<Dex>, collisionPolicy: &CollisionPolicy, context: &DxContext, writerSizes: &WriterSizes)    {
        self->dexes=dexes;
        self->collisionPolicy=collisionPolicy;
        self->context=context;
        self->writerSizes=writerSizes;
        self.dexOut=Dex::new(writerSizes.size());
        self.indexMaps=new IndexMap[dexes.length];
        for(        let i: i32 = 0;;i<dexes.len()i += 1)        {
            self.indexMaps[i]=IndexMap::new(&self.dexOut, dexes[i].getTableOfContents());
        }
        self.instructionTransformer=InstructionTransformer::new();
        self.headerOut=self.dexOut.appendSection(, "header");
        self.idsDefsOut=self.dexOut.appendSection(, "ids defs");
        self.contentsOut=self.dexOut.getTableOfContents();
        =self.dexOut.getNextSectionStart();
        =self.dexOut.getNextSectionStart();
        =1;
        self.mapListOut=self.dexOut.appendSection(, "map list");
        =self.dexOut.getNextSectionStart();
        self.typeListOut=self.dexOut.appendSection(, "type list");
        =self.dexOut.getNextSectionStart();
        self.annotationSetRefListOut=self.dexOut.appendSection(, "annotation set ref list");
        =self.dexOut.getNextSectionStart();
        self.annotationSetOut=self.dexOut.appendSection(, "annotation sets");
        =self.dexOut.getNextSectionStart();
        self.classDataOut=self.dexOut.appendSection(, "class data");
        =self.dexOut.getNextSectionStart();
        self.codeOut=self.dexOut.appendSection(, "code");
        =self.dexOut.getNextSectionStart();
        self.stringDataOut=self.dexOut.appendSection(, "string data");
        =self.dexOut.getNextSectionStart();
        self.debugInfoOut=self.dexOut.appendSection(, "debug info");
        =self.dexOut.getNextSectionStart();
        self.annotationOut=self.dexOut.appendSection(, "annotation");
        =self.dexOut.getNextSectionStart();
        self.encodedArrayOut=self.dexOut.appendSection(, "encoded array");
        =self.dexOut.getNextSectionStart();
        self.annotationsDirectoryOut=self.dexOut.appendSection(, "annotations directory");
        =self.dexOut.getNextSectionStart()-;
    }
    pub fn setCompactWasteThreshold(&self, compactWasteThreshold: i32)    {
        self->compactWasteThreshold=compactWasteThreshold;
    }
    pub fn mergeDexes(&self) -> Dex    {
        mergeStringIds();
        mergeTypeIds();
        mergeTypeLists();
        mergeProtoIds();
        mergeFieldIds();
        mergeMethodIds();
        mergeMethodHandles();
        mergeAnnotations();
        unionAnnotationSetsAndDirectories();
        mergeCallSiteIds();
        mergeClassDefs();
        Arrays::sort_Object[](&);
        =0;
        =1;
        =self.dexOut.getLength();
        self.contentsOut.computeSizesFromOffsets();
        self.contentsOut.writeHeader(&self.headerOut, mergeApiLevels());
        self.contentsOut.writeMap(&self.mapListOut);
        self.dexOut.writeHashes();
        return self.dexOut;
    }
    pub fn merge(&self) -> Dex    {
        if self.dexes.len()==1        {
            return self.dexes[0];
        }        else         if self.dexes.len()==0        {
            return None;
        }        
        let start: i64 = System::nanoTime();
        let result: Dex = mergeDexes();
        let compactedSizes: WriterSizes = WriterSizes::new(self);
        let wastedByteCount: i32 = self.writerSizes.size()-compactedSizes.size();
        if wastedByteCount>+self.compactWasteThreshold        {
            let compacter: DexMerger = DexMerger::new(vec![self.dexOut, Dex::new(0)], &CollisionPolicy::FAIL, &self.context, &compactedSizes);
            result=compacter.mergeDexes();
            .printf_String_Object[]("Result compacted from %.1fKiB to %.1fKiB to save %.1fKiB%n", self.dexOut.getLength()/1024f, result.getLength()/1024f, wastedByteCount/1024f);
        }        
        let elapsed: i64 = System::nanoTime()-start;
        for(        let i: i32 = 0;;i<self.dexes.len()i += 1)        {
            .printf_String_Object[]("Merged dex #%d (%d defs/%.1fKiB)%n", i+1, self.dexes[i].getTableOfContents()->classDefs->size, self.dexes[i].getLength()/1024f);
        }
        .printf_String_Object[]("Result is %d defs/%.1fKiB. Took %.1fs%n", result.getTableOfContents()->classDefs->size, result.getLength()/1024f, elapsed/1000000000f);
        return result;
    }
    pub fn mergeApiLevels(&self) -> i32    {
        let maxApi: i32 = -1;
        for(        let i: i32 = 0;;i<self.dexes.len()i += 1)        {
            let dexMinApi: i32 = self.dexes[i].getTableOfContents()->apiLevel;
            if maxApi<dexMinApi            {
                maxApi=dexMinApi;
            }            
        }
        return maxApi;
    }
    pub fn mergeStringIds(&self)    {
        /*new IdMerger<String>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.stringIds;
  }
  @Override String read(  Dex.Section in,  IndexMap indexMap,  int index){
    return in.readString();
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    indexMap.stringIds[oldIndex]=newIndex;
  }
  @Override void write(  String value){
    contentsOut.stringDatas.size++;
    idsDefsOut.writeInt(stringDataOut.getPosition());
    stringDataOut.writeStringData(value);
  }
}
*/.mergeSorted();
    }
    pub fn mergeTypeIds(&self)    {
        /*new IdMerger<Integer>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.typeIds;
  }
  @Override Integer read(  Dex.Section in,  IndexMap indexMap,  int index){
    int stringIndex=in.readInt();
    return indexMap.adjustString(stringIndex);
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    if (newIndex < 0 || newIndex > 0xffff) {
      throw new DexIndexOverflowException("type ID not in [0, 0xffff]: " + newIndex);
    }
    indexMap.typeIds[oldIndex]=(short)newIndex;
  }
  @Override void write(  Integer value){
    idsDefsOut.writeInt(value);
  }
}
*/.mergeSorted();
    }
    pub fn mergeTypeLists(&self)    {
        /*new IdMerger<TypeList>(typeListOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.typeLists;
  }
  @Override TypeList read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjustTypeList(in.readTypeList());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    indexMap.putTypeListOffset(offset,typeListOut.getPosition());
  }
  @Override void write(  TypeList value){
    typeListOut.writeTypeList(value);
  }
}
*/.mergeUnsorted();
    }
    pub fn mergeProtoIds(&self)    {
        /*new IdMerger<ProtoId>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.protoIds;
  }
  @Override ProtoId read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjust(in.readProtoId());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    if (newIndex < 0 || newIndex > 0xffff) {
      throw new DexIndexOverflowException("proto ID not in [0, 0xffff]: " + newIndex);
    }
    indexMap.protoIds[oldIndex]=(short)newIndex;
  }
  @Override void write(  ProtoId value){
    value.writeTo(idsDefsOut);
  }
}
*/.mergeSorted();
    }
    pub fn mergeCallSiteIds(&self)    {
        /*new IdMerger<CallSiteId>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.callSiteIds;
  }
  @Override CallSiteId read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjust(in.readCallSiteId());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    indexMap.callSiteIds[oldIndex]=newIndex;
  }
  @Override void write(  CallSiteId value){
    value.writeTo(idsDefsOut);
  }
}
*/.mergeSorted();
    }
    pub fn mergeMethodHandles(&self)    {
        /*new IdMerger<MethodHandle>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.methodHandles;
  }
  @Override MethodHandle read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjust(in.readMethodHandle());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    indexMap.methodHandleIds.put(oldIndex,indexMap.methodHandleIds.size());
  }
  @Override void write(  MethodHandle value){
    value.writeTo(idsDefsOut);
  }
}
*/.mergeUnsorted();
    }
    pub fn mergeFieldIds(&self)    {
        /*new IdMerger<FieldId>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.fieldIds;
  }
  @Override FieldId read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjust(in.readFieldId());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    if (newIndex < 0 || newIndex > 0xffff) {
      throw new DexIndexOverflowException("field ID not in [0, 0xffff]: " + newIndex);
    }
    indexMap.fieldIds[oldIndex]=(short)newIndex;
  }
  @Override void write(  FieldId value){
    value.writeTo(idsDefsOut);
  }
}
*/.mergeSorted();
    }
    pub fn mergeMethodIds(&self)    {
        /*new IdMerger<MethodId>(idsDefsOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.methodIds;
  }
  @Override MethodId read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjust(in.readMethodId());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    if (newIndex < 0 || newIndex > 0xffff) {
      throw new DexIndexOverflowException("method ID not in [0, 0xffff]: " + newIndex);
    }
    indexMap.methodIds[oldIndex]=(short)newIndex;
  }
  @Override void write(  MethodId methodId){
    methodId.writeTo(idsDefsOut);
  }
}
*/.mergeSorted();
    }
    pub fn mergeAnnotations(&self)    {
        /*new IdMerger<Annotation>(annotationOut){
  @Override TableOfContents.Section getSection(  TableOfContents tableOfContents){
    return tableOfContents.annotations;
  }
  @Override Annotation read(  Dex.Section in,  IndexMap indexMap,  int index){
    return indexMap.adjust(in.readAnnotation());
  }
  @Override void updateIndex(  int offset,  IndexMap indexMap,  int oldIndex,  int newIndex){
    indexMap.putAnnotationOffset(offset,annotationOut.getPosition());
  }
  @Override void write(  Annotation value){
    value.writeTo(annotationOut);
  }
}
*/.mergeUnsorted();
    }
    pub fn mergeClassDefs(&self)    {
        let types: Vec<SortableType> = getSortedTypes();
        =self.idsDefsOut.getPosition();
        =types.len();
        for type in types        {
            let in: Dex = type_renamed.getDex();
            transformClassDef(&in_renamed, type_renamed.getClassDef(), type_renamed.getIndexMap());
        }
    }
    pub fn getSortedTypes(&self) -> Vec<SortableType>    {
        let sortableTypes: Vec<SortableType> = new SortableType[contentsOut.typeIds.size];
        for(        let i: i32 = 0;;i<self.dexes.len()i += 1)        {
            readSortableTypes(&sortableTypes, self.dexes[i], self.indexMaps[i]);
        }
        while true        {
            let allDone: boolean = true;
            for sortableType in sortableTypes            {
                if sortableType!=None&&!sortableType.isDepthAssigned()                {
                    allDone&=sortableType.tryAssignDepth(&sortableTypes);
                }                
            }
            if allDone            {
                break;
            }            
        }
        Arrays::sort_SortableType[]_Comparator<? super SortableType>(&sortableTypes, &SortableType::NULLS_LAST_ORDER);
        let firstNull: i32 = Arrays::asList(&sortableTypes).indexOf(None);
        return if firstNull!=-1 { Arrays::copyOfRange_SortableType[]_int_int(&sortableTypes, 0, firstNull) } else { sortableTypes };
            }
            pub fn readSortableTypes(&self, sortableTypes: &Vec<SortableType>, buffer: &Dex, indexMap: &IndexMap)            {
                for classDef in buffer.classDefs()                {
                    let sortableType: SortableType = indexMap.adjust_SortableType(SortableType::new(&buffer, &indexMap, &classDef));
                    let t: i32 = sortableType.getTypeIndex();
                    if sortableTypes[t]==None                    {
                        sortableTypes[t]=sortableType;
                    }                    else                     if self.collisionPolicy!=CollisionPolicy::KEEP_FIRST                    {
                        throw DexException::new("Multiple dex files define "+buffer.typeNames().get(classDef.getTypeIndex()));
                    }                    
                }
            }
            pub fn unionAnnotationSetsAndDirectories(&self)            {
                for(                let i: i32 = 0;;i<self.dexes.len()i += 1)                {
                    transformAnnotationSets(self.dexes[i], self.indexMaps[i]);
                }
                for(                let i: i32 = 0;;i<self.dexes.len()i += 1)                {
                    transformAnnotationSetRefLists(self.dexes[i], self.indexMaps[i]);
                }
                for(                let i: i32 = 0;;i<self.dexes.len()i += 1)                {
                    transformAnnotationDirectories(self.dexes[i], self.indexMaps[i]);
                }
                for(                let i: i32 = 0;;i<self.dexes.len()i += 1)                {
                    transformStaticValues_Dex_IndexMap(self.dexes[i], self.indexMaps[i]);
                }
            }
            pub fn transformAnnotationSets(&self, in: &Dex, indexMap: &IndexMap)            {
                let section: Section = in.getTableOfContents()->annotationSets;
                if section.exists()                {
                    let setIn: Section = in.open();
                    for(                    let i: i32 = 0;;i<i += 1)                    {
                        transformAnnotationSet(&indexMap, &setIn);
                    }
                }                
            }
            pub fn transformAnnotationSetRefLists(&self, in: &Dex, indexMap: &IndexMap)            {
                let section: Section = in.getTableOfContents()->annotationSetRefLists;
                if section.exists()                {
                    let setIn: Section = in.open();
                    for(                    let i: i32 = 0;;i<i += 1)                    {
                        transformAnnotationSetRefList(&indexMap, &setIn);
                    }
                }                
            }
            pub fn transformAnnotationDirectories(&self, in: &Dex, indexMap: &IndexMap)            {
                let section: Section = in.getTableOfContents()->annotationsDirectories;
                if section.exists()                {
                    let directoryIn: Section = in.open();
                    for(                    let i: i32 = 0;;i<i += 1)                    {
                        transformAnnotationDirectory(&directoryIn, &indexMap);
                    }
                }                
            }
            pub fn transformStaticValues(&self, in: &Dex, indexMap: &IndexMap)            {
                let section: Section = in.getTableOfContents()->encodedArrays;
                if section.exists()                {
                    let staticValuesIn: Section = in.open();
                    for(                    let i: i32 = 0;;i<i += 1)                    {
                        transformStaticValues_Section_IndexMap(&staticValuesIn, &indexMap);
                    }
                }                
            }
            pub fn transformClassDef(&self, in: &Dex, classDef: &ClassDef, indexMap: &IndexMap)            {
                self.idsDefsOut.assertFourByteAligned();
                self.idsDefsOut.writeInt(classDef.getTypeIndex());
                self.idsDefsOut.writeInt(classDef.getAccessFlags());
                self.idsDefsOut.writeInt(classDef.getSupertypeIndex());
                self.idsDefsOut.writeInt(classDef.getInterfacesOffset());
                let sourceFileIndex: i32 = indexMap.adjustString(classDef.getSourceFileIndex());
                self.idsDefsOut.writeInt(sourceFileIndex);
                let annotationsOff: i32 = classDef.getAnnotationsOffset();
                self.idsDefsOut.writeInt(indexMap.adjustAnnotationDirectory(annotationsOff));
                let classDataOff: i32 = classDef.getClassDataOffset();
                if classDataOff==0                {
                    self.idsDefsOut.writeInt(0);
                }                else                 {
                    self.idsDefsOut.writeInt(self.classDataOut.getPosition());
                    let classData: ClassData = in.readClassData(&classDef);
                    transformClassData(&in, &classData, &indexMap);
                }
                let staticValuesOff: i32 = classDef.getStaticValuesOffset();
                self.idsDefsOut.writeInt(indexMap.adjustEncodedArray_int(staticValuesOff));
            }
            pub fn transformAnnotationDirectory(&self, directoryIn: &Dex.Section, indexMap: &IndexMap)            {
                 += 1;
                self.annotationsDirectoryOut.assertFourByteAligned();
                indexMap.putAnnotationDirectoryOffset(directoryIn.getPosition(), self.annotationsDirectoryOut.getPosition());
                let classAnnotationsOffset: i32 = indexMap.adjustAnnotationSet(directoryIn.readInt());
                self.annotationsDirectoryOut.writeInt(classAnnotationsOffset);
                let fieldsSize: i32 = directoryIn.readInt();
                self.annotationsDirectoryOut.writeInt(fieldsSize);
                let methodsSize: i32 = directoryIn.readInt();
                self.annotationsDirectoryOut.writeInt(methodsSize);
                let parameterListSize: i32 = directoryIn.readInt();
                self.annotationsDirectoryOut.writeInt(parameterListSize);
                for(                let i: i32 = 0;;i<fieldsSizei += 1)                {
                    self.annotationsDirectoryOut.writeInt(indexMap.adjustField(directoryIn.readInt()));
                    self.annotationsDirectoryOut.writeInt(indexMap.adjustAnnotationSet(directoryIn.readInt()));
                }
                for(                let i: i32 = 0;;i<methodsSizei += 1)                {
                    self.annotationsDirectoryOut.writeInt(indexMap.adjustMethod(directoryIn.readInt()));
                    self.annotationsDirectoryOut.writeInt(indexMap.adjustAnnotationSet(directoryIn.readInt()));
                }
                for(                let i: i32 = 0;;i<parameterListSizei += 1)                {
                    self.annotationsDirectoryOut.writeInt(indexMap.adjustMethod(directoryIn.readInt()));
                    self.annotationsDirectoryOut.writeInt(indexMap.adjustAnnotationSetRefList(directoryIn.readInt()));
                }
            }
            pub fn transformAnnotationSet(&self, indexMap: &IndexMap, setIn: &Dex.Section)            {
                 += 1;
                self.annotationSetOut.assertFourByteAligned();
                indexMap.putAnnotationSetOffset(setIn.getPosition(), self.annotationSetOut.getPosition());
                let size: i32 = setIn.readInt();
                self.annotationSetOut.writeInt(size);
                for(                let j: i32 = 0;;j<sizej += 1)                {
                    self.annotationSetOut.writeInt(indexMap.adjustAnnotation(setIn.readInt()));
                }
            }
            pub fn transformAnnotationSetRefList(&self, indexMap: &IndexMap, refListIn: &Dex.Section)            {
                 += 1;
                self.annotationSetRefListOut.assertFourByteAligned();
                indexMap.putAnnotationSetRefListOffset(refListIn.getPosition(), self.annotationSetRefListOut.getPosition());
                let parameterCount: i32 = refListIn.readInt();
                self.annotationSetRefListOut.writeInt(parameterCount);
                for(                let p: i32 = 0;;p<parameterCountp += 1)                {
                    self.annotationSetRefListOut.writeInt(indexMap.adjustAnnotationSet(refListIn.readInt()));
                }
            }
            pub fn transformClassData(&self, in: &Dex, classData: &ClassData, indexMap: &IndexMap)            {
                 += 1;
                let staticFields: Vec<Field> = classData.getStaticFields();
                let instanceFields: Vec<Field> = classData.getInstanceFields();
                let directMethods: Vec<Method> = classData.getDirectMethods();
                let virtualMethods: Vec<Method> = classData.getVirtualMethods();
                self.classDataOut.writeUleb128(staticFields.len());
                self.classDataOut.writeUleb128(instanceFields.len());
                self.classDataOut.writeUleb128(directMethods.len());
                self.classDataOut.writeUleb128(virtualMethods.len());
                transformFields(&indexMap, &staticFields);
                transformFields(&indexMap, &instanceFields);
                transformMethods(&in, &indexMap, &directMethods);
                transformMethods(&in, &indexMap, &virtualMethods);
            }
            pub fn transformFields(&self, indexMap: &IndexMap, fields: &Vec<ClassData.Field>)            {
                let lastOutFieldIndex: i32 = 0;
                for field in fields                {
                    let outFieldIndex: i32 = indexMap.adjustField(field.getFieldIndex());
                    self.classDataOut.writeUleb128(outFieldIndex-lastOutFieldIndex);
                    lastOutFieldIndex=outFieldIndex;
                    self.classDataOut.writeUleb128(field.getAccessFlags());
                }
            }
            pub fn transformMethods(&self, in: &Dex, indexMap: &IndexMap, methods: &Vec<ClassData.Method>)            {
                let lastOutMethodIndex: i32 = 0;
                for method in methods                {
                    let outMethodIndex: i32 = indexMap.adjustMethod(method.getMethodIndex());
                    self.classDataOut.writeUleb128(outMethodIndex-lastOutMethodIndex);
                    lastOutMethodIndex=outMethodIndex;
                    self.classDataOut.writeUleb128(method.getAccessFlags());
                    if method.getCodeOffset()==0                    {
                        self.classDataOut.writeUleb128(0);
                    }                    else                     {
                        self.codeOut.alignToFourBytesWithZeroFill();
                        self.classDataOut.writeUleb128(self.codeOut.getPosition());
                        transformCode(&in, in.readCode(&method), &indexMap);
                    }
                }
            }
            pub fn transformCode(&self, in: &Dex, code: &Code, indexMap: &IndexMap)            {
                 += 1;
                self.codeOut.assertFourByteAligned();
                self.codeOut.writeUnsignedShort(code.getRegistersSize());
                self.codeOut.writeUnsignedShort(code.getInsSize());
                self.codeOut.writeUnsignedShort(code.getOutsSize());
                let tries: Vec<Try> = code.getTries();
                let catchHandlers: Vec<CatchHandler> = code.getCatchHandlers();
                self.codeOut.writeUnsignedShort(tries.len());
                let debugInfoOffset: i32 = code.getDebugInfoOffset();
                if debugInfoOffset!=0                {
                    self.codeOut.writeInt(self.debugInfoOut.getPosition());
                    transformDebugInfoItem(in.open(debugInfoOffset), &indexMap);
                }                else                 {
                    self.codeOut.writeInt(0);
                }
                let instructions: Vec<i16> = code.getInstructions();
                let newInstructions: Vec<i16> = self.instructionTransformer.transform(&indexMap, &instructions);
                self.codeOut.writeInt(newInstructions.len());
                self.codeOut.write_short[](&newInstructions);
                if tries.len()>0                {
                    if newInstructions.len()%2==1                    {
                        self.codeOut.writeShort(0 as i16);
                    }                    
                    let triesSection: Section = self.dexOut.open(self.codeOut.getPosition());
                    self.codeOut.skip(tries.len()*SizeOf::TRY_ITEM);
                    let offsets: Vec<i32> = transformCatchHandlers(&indexMap, &catchHandlers);
                    transformTries(&triesSection, &tries, &offsets);
                }                
            }
            pub fn transformCatchHandlers(&self, indexMap: &IndexMap, catchHandlers: &Vec<Code.CatchHandler>) -> Vec<i32>            {
                let baseOffset: i32 = self.codeOut.getPosition();
                self.codeOut.writeUleb128(catchHandlers.len());
                let offsets: Vec<i32> = new int[catchHandlers.length];
                for(                let i: i32 = 0;;i<catchHandlers.len()i += 1)                {
                    offsets[i]=self.codeOut.getPosition()-baseOffset;
                    transformEncodedCatchHandler(catchHandlers[i], &indexMap);
                }
                return offsets;
            }
            pub fn transformTries(&self, out: &Dex.Section, tries: &Vec<Code.Try>, catchHandlerOffsets: &Vec<i32>)            {
                for tryItem in tries                {
                    out.writeInt(tryItem.getStartAddress());
                    out.writeUnsignedShort(tryItem.getInstructionCount());
                    out.writeUnsignedShort(catchHandlerOffsets[tryItem.getCatchHandlerIndex()]);
                }
            }
            pub fn transformDebugInfoItem(&self, in: &Dex.Section, indexMap: &IndexMap)            {
                 += 1;
                let lineStart: i32 = in.readUleb128();
                self.debugInfoOut.writeUleb128(lineStart);
                let parametersSize: i32 = in.readUleb128();
                self.debugInfoOut.writeUleb128(parametersSize);
                for(                let p: i32 = 0;;p<parametersSizep += 1)                {
                    let parameterName: i32 = in.readUleb128p1();
                    self.debugInfoOut.writeUleb128p1(indexMap.adjustString(parameterName));
                }
                let addrDiff: i32;
                let lineDiff: i32;
                let registerNum: i32;
                let nameIndex: i32;
                let typeIndex: i32;
                let sigIndex: i32;
                while true                {
                    let opcode: i32 = in.readByte();
                    self.debugInfoOut.writeByte(opcode);
                    match opcode{DexMerger::DBG_END_SEQUENCE =>                         return;DexMerger::DBG_ADVANCE_PC =>                         addrDiff=in.readUleb128();                        self.debugInfoOut.writeUleb128(addrDiff);                        break;DexMerger::DBG_ADVANCE_LINE =>                         lineDiff=in.readSleb128();                        self.debugInfoOut.writeSleb128(lineDiff);                        break;DexMerger::DBG_START_LOCAL => DexMerger::DBG_START_LOCAL_EXTENDED =>                         registerNum=in.readUleb128();                        self.debugInfoOut.writeUleb128(registerNum);                        nameIndex=in.readUleb128p1();                        self.debugInfoOut.writeUleb128p1(indexMap.adjustString(nameIndex));                        typeIndex=in.readUleb128p1();                        self.debugInfoOut.writeUleb128p1(indexMap.adjustType(typeIndex));                        if opcode==DexMerger::DBG_START_LOCAL_EXTENDED                        {
                            sigIndex=in.readUleb128p1();
                            self.debugInfoOut.writeUleb128p1(indexMap.adjustString(sigIndex));
                        }                                                break;DexMerger::DBG_END_LOCAL => DexMerger::DBG_RESTART_LOCAL =>                         registerNum=in.readUleb128();                        self.debugInfoOut.writeUleb128(registerNum);                        break;DexMerger::DBG_SET_FILE =>                         nameIndex=in.readUleb128p1();                        self.debugInfoOut.writeUleb128p1(indexMap.adjustString(nameIndex));                        break;DexMerger::DBG_SET_PROLOGUE_END => DexMerger::DBG_SET_EPILOGUE_BEGIN =>                     _ => {}                    break;                }
            }
        }
        pub fn transformEncodedCatchHandler(&self, catchHandler: &Code.CatchHandler, indexMap: &IndexMap)        {
            let catchAllAddress: i32 = catchHandler.getCatchAllAddress();
            let typeIndexes: Vec<i32> = catchHandler.getTypeIndexes();
            let addresses: Vec<i32> = catchHandler.getAddresses();
            if catchAllAddress!=-1            {
                self.codeOut.writeSleb128(-typeIndexes.len());
            }            else             {
                self.codeOut.writeSleb128(typeIndexes.len());
            }
            for(            let i: i32 = 0;;i<typeIndexes.len()i += 1)            {
                self.codeOut.writeUleb128(indexMap.adjustType(typeIndexes[i]));
                self.codeOut.writeUleb128(addresses[i]);
            }
            if catchAllAddress!=-1            {
                self.codeOut.writeUleb128(catchAllAddress);
            }            
        }
        pub fn transformStaticValues(&self, in: &Dex.Section, indexMap: &IndexMap)        {
             += 1;
            indexMap.putEncodedArrayValueOffset(in.getPosition(), self.encodedArrayOut.getPosition());
            indexMap.adjustEncodedArray_EncodedValue(in.readEncodedArray()).writeTo(&self.encodedArrayOut);
        }
        pub fn main(args: &Vec<String>)        {
            if args.len()<2            {
                DexMerger::printUsage();
                return;
            }            
            let dexes: Vec<Dex> = new Dex[args.length - 1];
            for(            let i: i32 = 1;;i<args.len()i += 1)            {
                dexes[i-1]=Dex::new(File::new(args[i]));
            }
            let merged: Dex = DexMerger::new(&dexes, &CollisionPolicy::KEEP_FIRST, DxContext::new()).merge();
            merged.writeTo_File(File::new(args[0]));
        }
        pub fn printUsage()        {
            System::out.println_String("Usage: DexMerger <out.dex> <a.dex> <b.dex> ...");
            System::out.println();
            System::out.println_String("If a class is defined in several dex, the class found in the first dex will be used.");
        }
}
