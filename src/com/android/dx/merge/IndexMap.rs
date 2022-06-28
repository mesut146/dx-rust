use crate::helper;
use crate::com::android::dex::CallSiteId;
use crate::com::android::dx::merge::SortableType;
use crate::com::android::dex::FieldId;
use crate::com::android::dex::util::ByteOutput;
use crate::com::android::dex::MethodHandle;
use crate::com::android::dex::TypeList;
use crate::com::android::dex::Annotation;
use crate::com::android::dx::merge::IndexMap::EncodedValueTransformer;
use crate::com::android::dex::EncodedValueReader;
use crate::com::android::dex::ProtoId;
use crate::com::android::dex::MethodHandle::MethodHandleType;
use crate::com::android::dex::Dex;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dex::EncodedValueCodec;
use crate::com::android::dex::MethodId;
use crate::com::android::dex::Leb128;
use crate::com::android::dex::ClassDef;
use crate::com::android::dex::EncodedValue;
use crate::com::android::dex::DexException;
use crate::com::android::dex::TableOfContents::Section;

struct IndexMap{
    pub target: Dex,
    pub stringIds: Vec<i32>,
    pub typeIds: Vec<i16>,
    pub protoIds: Vec<i16>,
    pub fieldIds: Vec<i16>,
    pub methodIds: Vec<i16>,
    pub callSiteIds: Vec<i32>,
    pub methodHandleIds: HashMap<Integer,Integer>,
    pub typeListOffsets: HashMap<Integer,Integer>,
    pub annotationOffsets: HashMap<Integer,Integer>,
    pub annotationSetOffsets: HashMap<Integer,Integer>,
    pub annotationSetRefListOffsets: HashMap<Integer,Integer>,
    pub annotationDirectoryOffsets: HashMap<Integer,Integer>,
    pub encodedArrayValueOffset: HashMap<Integer,Integer>,
}
impl IndexMap{
    pub fn new(&self, target: &Dex, tableOfContents: &TableOfContents)    {
        self->target=target;
        self->stringIds=new int[tableOfContents.stringIds.size];
        self->typeIds=new short[tableOfContents.typeIds.size];
        self->protoIds=new short[tableOfContents.protoIds.size];
        self->fieldIds=new short[tableOfContents.fieldIds.size];
        self->methodIds=new short[tableOfContents.methodIds.size];
        self->callSiteIds=new int[tableOfContents.callSiteIds.size];
        self->methodHandleIds=HashMap<Integer,Integer>::new();
        self->typeListOffsets=HashMap<Integer,Integer>::new();
        self->annotationOffsets=HashMap<Integer,Integer>::new();
        self->annotationSetOffsets=HashMap<Integer,Integer>::new();
        self->annotationSetRefListOffsets=HashMap<Integer,Integer>::new();
        self->annotationDirectoryOffsets=HashMap<Integer,Integer>::new();
        self->encodedArrayValueOffset=HashMap<Integer,Integer>::new();
        self->typeListOffsets.put(0, 0);
        self->annotationSetOffsets.put(0, 0);
        self->annotationDirectoryOffsets.put(0, 0);
        self->encodedArrayValueOffset.put(0, 0);
    }
    pub fn putTypeListOffset(&self, oldOffset: i32, newOffset: i32)    {
        if oldOffset<=0||newOffset<=0        {
            throw IllegalArgumentException::new();
        }        
        self.typeListOffsets.put(oldOffset, newOffset);
    }
    pub fn putAnnotationOffset(&self, oldOffset: i32, newOffset: i32)    {
        if oldOffset<=0||newOffset<=0        {
            throw IllegalArgumentException::new();
        }        
        self.annotationOffsets.put(oldOffset, newOffset);
    }
    pub fn putAnnotationSetOffset(&self, oldOffset: i32, newOffset: i32)    {
        if oldOffset<=0||newOffset<=0        {
            throw IllegalArgumentException::new();
        }        
        self.annotationSetOffsets.put(oldOffset, newOffset);
    }
    pub fn putAnnotationSetRefListOffset(&self, oldOffset: i32, newOffset: i32)    {
        if oldOffset<=0||newOffset<=0        {
            throw IllegalArgumentException::new();
        }        
        self.annotationSetRefListOffsets.put(oldOffset, newOffset);
    }
    pub fn putAnnotationDirectoryOffset(&self, oldOffset: i32, newOffset: i32)    {
        if oldOffset<=0||newOffset<=0        {
            throw IllegalArgumentException::new();
        }        
        self.annotationDirectoryOffsets.put(oldOffset, newOffset);
    }
    pub fn putEncodedArrayValueOffset(&self, oldOffset: i32, newOffset: i32)    {
        if oldOffset<=0||newOffset<=0        {
            throw IllegalArgumentException::new();
        }        
        self.encodedArrayValueOffset.put(oldOffset, newOffset);
    }
    pub fn adjustString(&self, stringIndex: i32) -> i32    {
        return if stringIndex==ClassDef::NO_INDEX { ClassDef::NO_INDEX } else { self.stringIds[stringIndex] };
            }
            pub fn adjustType(&self, typeIndex: i32) -> i32            {
                return if (typeIndex==ClassDef::NO_INDEX) { ClassDef::NO_INDEX } else { (self.typeIds[typeIndex]&0xffff) };
                    }
                    pub fn adjustTypeList(&self, typeList: &TypeList) -> TypeList                    {
                        if typeList==TypeList::EMPTY                        {
                            return typeList;
                        }                        
                        let types: Vec<i16> = typeList.getTypes().clone();
                        for(                        let i: i32 = 0;;i<types.len()i += 1)                        {
                            types[i]=adjustType(types[i]) as i16;
                        }
                        return TypeList::new(&self.target, &types);
                    }
                    pub fn adjustProto(&self, protoIndex: i32) -> i32                    {
                        return self.protoIds[protoIndex]&0xffff;
                    }
                    pub fn adjustField(&self, fieldIndex: i32) -> i32                    {
                        return self.fieldIds[fieldIndex]&0xffff;
                    }
                    pub fn adjustMethod(&self, methodIndex: i32) -> i32                    {
                        return self.methodIds[methodIndex]&0xffff;
                    }
                    pub fn adjustTypeListOffset(&self, typeListOffset: i32) -> i32                    {
                        return self.typeListOffsets.get(typeListOffset);
                    }
                    pub fn adjustAnnotation(&self, annotationOffset: i32) -> i32                    {
                        return self.annotationOffsets.get(annotationOffset);
                    }
                    pub fn adjustAnnotationSet(&self, annotationSetOffset: i32) -> i32                    {
                        return self.annotationSetOffsets.get(annotationSetOffset);
                    }
                    pub fn adjustAnnotationSetRefList(&self, annotationSetRefListOffset: i32) -> i32                    {
                        return self.annotationSetRefListOffsets.get(annotationSetRefListOffset);
                    }
                    pub fn adjustAnnotationDirectory(&self, annotationDirectoryOffset: i32) -> i32                    {
                        return self.annotationDirectoryOffsets.get(annotationDirectoryOffset);
                    }
                    pub fn adjustEncodedArray(&self, encodedArrayAttribute: i32) -> i32                    {
                        return self.encodedArrayValueOffset.get(encodedArrayAttribute);
                    }
                    pub fn adjustCallSite(&self, callSiteIndex: i32) -> i32                    {
                        return self.callSiteIds[callSiteIndex];
                    }
                    pub fn adjustMethodHandle(&self, methodHandleIndex: i32) -> i32                    {
                        return self.methodHandleIds.get(methodHandleIndex);
                    }
                    pub fn adjust(&self, methodId: &MethodId) -> MethodId                    {
                        return MethodId::new(&self.target, adjustType(methodId.getDeclaringClassIndex()), adjustProto(methodId.getProtoIndex()), adjustString(methodId.getNameIndex()));
                    }
                    pub fn adjust(&self, callSiteId: &CallSiteId) -> CallSiteId                    {
                        return CallSiteId::new(&self.target, adjustEncodedArray_int(callSiteId.getCallSiteOffset()));
                    }
                    pub fn adjust(&self, methodHandle: &MethodHandle) -> MethodHandle                    {
                        return MethodHandle::new(&self.target, methodHandle.getMethodHandleType(), methodHandle.getUnused1(), if methodHandle.getMethodHandleType().isField() { adjustField(methodHandle.getFieldOrMethodId()) } else { adjustMethod(methodHandle.getFieldOrMethodId()) }, methodHandle.getUnused2());
                            }
                            pub fn adjust(&self, fieldId: &FieldId) -> FieldId                            {
                                return FieldId::new(&self.target, adjustType(fieldId.getDeclaringClassIndex()), adjustType(fieldId.getTypeIndex()), adjustString(fieldId.getNameIndex()));
                            }
                            pub fn adjust(&self, protoId: &ProtoId) -> ProtoId                            {
                                return ProtoId::new(&self.target, adjustString(protoId.getShortyIndex()), adjustType(protoId.getReturnTypeIndex()), adjustTypeListOffset(protoId.getParametersOffset()));
                            }
                            pub fn adjust(&self, classDef: &ClassDef) -> ClassDef                            {
                                return ClassDef::new(&self.target, classDef.getOffset(), adjustType(classDef.getTypeIndex()), classDef.getAccessFlags(), adjustType(classDef.getSupertypeIndex()), adjustTypeListOffset(classDef.getInterfacesOffset()), classDef.getSourceFileIndex(), classDef.getAnnotationsOffset(), classDef.getClassDataOffset(), classDef.getStaticValuesOffset());
                            }
                            pub fn adjust(&self, sortableType: &SortableType) -> SortableType                            {
                                return SortableType::new(sortableType.getDex(), sortableType.getIndexMap(), adjust_ClassDef(sortableType.getClassDef()));
                            }
                            pub fn adjustEncodedValue(&self, encodedValue: &EncodedValue) -> EncodedValue                            {
                                let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new(32);
                                EncodedValueTransformer::new(&out, self).transform(EncodedValueReader::new(&encodedValue));
                                return EncodedValue::new(out.toByteArray());
                            }
                            pub fn adjustEncodedArray(&self, encodedArray: &EncodedValue) -> EncodedValue                            {
                                let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new(32);
                                EncodedValueTransformer::new(&out, self).transformArray(EncodedValueReader::new(&encodedArray, EncodedValueReader::ENCODED_ARRAY));
                                return EncodedValue::new(out.toByteArray());
                            }
                            pub fn adjust(&self, annotation: &Annotation) -> Annotation                            {
                                let out: ByteArrayAnnotatedOutput = ByteArrayAnnotatedOutput::new(32);
                                EncodedValueTransformer::new(&out, self).transformAnnotation(annotation.getReader());
                                return Annotation::new(&self.target, annotation.getVisibility(), EncodedValue::new(out.toByteArray()));
                            }
}
