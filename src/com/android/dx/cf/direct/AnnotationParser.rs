use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::CstAnnotation;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::rop::annotation::Annotation;
use crate::com::android::dx::rop::annotation::AnnotationVisibility;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::rop::cst::CstEnumRef;
use crate::com::android::dx::rop::annotation::NameValuePair;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstArray::List;
use crate::com::android::dx::cf::iface::ParseObserver;
use crate::com::android::dx::rop::cst::CstChar;
use crate::com::android::dx::rop::annotation::AnnotationsList;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::cst::CstShort;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstArray;
use crate::com::android::dx::rop::cst::CstBoolean;
use crate::com::android::dx::cf::iface::ParseException;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::util::ByteArray::MyDataInputStream;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::rop::cst::CstDouble;
use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::rop::cst::CstByte;
use crate::com::android::dx::rop::cst::CstNat;

struct AnnotationParser{
    pub cf: DirectClassFile,
    pub pool: ConstantPool,
    pub bytes: ByteArray,
    pub observer: ParseObserver,
    pub input: ByteArray.MyDataInputStream,
    pub parseCursor: i32,
}
impl AnnotationParser{
    pub fn new(&self, cf: &DirectClassFile, offset: i32, length: i32, observer: &ParseObserver)    {
        if cf==None        {
            throw NullPointerException::new("cf == null");
        }        
        self->cf=cf;
        self->pool=cf.getConstantPool();
        self->observer=observer;
        self->bytes=cf.getBytes().slice(offset, offset+length);
        self->input=self.bytes.makeDataInputStream();
        self->parseCursor=0;
    }
    pub fn parseValueAttribute(&self) -> Constant    {
        let result: Constant;
        try        {
            result=parseValue();
            if self.input.available()!=0            {
                throw ParseException::new("extra data in attribute");
            }            
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new("shouldn't happen", &ex);
        }
        return result;
    }
    pub fn parseParameterAttribute(&self, visibility: &AnnotationVisibility) -> AnnotationsList    {
        let result: AnnotationsList;
        try        {
            result=parseAnnotationsList(&visibility);
            if self.input.available()!=0            {
                throw ParseException::new("extra data in attribute");
            }            
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new("shouldn't happen", &ex);
        }
        return result;
    }
    pub fn parseAnnotationAttribute(&self, visibility: &AnnotationVisibility) -> Annotations    {
        let result: Annotations;
        try        {
            result=parseAnnotations(&visibility);
            if self.input.available()!=0            {
                throw ParseException::new("extra data in attribute");
            }            
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new("shouldn't happen", &ex);
        }
        return result;
    }
    pub fn parseAnnotationsList(&self, visibility: &AnnotationVisibility) -> AnnotationsList    {
        let count: i32 = self.input.readUnsignedByte();
        if self.observer!=None        {
            parsed(1, "num_parameters: "+Hex::u1(count));
        }        
        let outerList: AnnotationsList = AnnotationsList::new(count);
        for(        let i: i32 = 0;;i<counti += 1)        {
            if self.observer!=None            {
                parsed(0, "parameter_annotations["+i+"]:");
                changeIndent(1);
            }            
            let annotations: Annotations = parseAnnotations(&visibility);
            outerList.set(i, &annotations);
            if self.observer!=None            {
                self.observer.changeIndent(-1);
            }            
        }
        outerList.setImmutable();
        return outerList;
    }
    pub fn parseAnnotations(&self, visibility: &AnnotationVisibility) -> Annotations    {
        let count: i32 = self.input.readUnsignedShort();
        if self.observer!=None        {
            parsed(2, "num_annotations: "+Hex::u2(count));
        }        
        let annotations: Annotations = Annotations::new();
        for(        let i: i32 = 0;;i<counti += 1)        {
            if self.observer!=None            {
                parsed(0, "annotations["+i+"]:");
                changeIndent(1);
            }            
            let annotation: Annotation = parseAnnotation(&visibility);
            annotations.add(&annotation);
            if self.observer!=None            {
                self.observer.changeIndent(-1);
            }            
        }
        annotations.setImmutable();
        return annotations;
    }
    pub fn parseAnnotation(&self, visibility: &AnnotationVisibility) -> Annotation    {
        requireLength(4);
        let typeIndex: i32 = self.input.readUnsignedShort();
        let numElements: i32 = self.input.readUnsignedShort();
        let typeString: CstString = (CstString*)self.pool.get(typeIndex);
        let type: CstType = CstType::new(Type::intern(typeString.getString()));
        if self.observer!=None        {
            parsed(2, "type: "+type_renamed.toHuman());
            parsed(2, "num_elements: "+numElements);
        }        
        let annotation: Annotation = Annotation::new(&type_renamed, &visibility);
        for(        let i: i32 = 0;;i<numElementsi += 1)        {
            if self.observer!=None            {
                parsed(0, "elements["+i+"]:");
                changeIndent(1);
            }            
            let element: NameValuePair = parseElement();
            annotation.add(&element);
            if self.observer!=None            {
                changeIndent(-1);
            }            
        }
        annotation.setImmutable();
        return annotation;
    }
    pub fn parseElement(&self) -> NameValuePair    {
        requireLength(5);
        let elementNameIndex: i32 = self.input.readUnsignedShort();
        let elementName: CstString = (CstString*)self.pool.get(elementNameIndex);
        if self.observer!=None        {
            parsed(2, "element_name: "+elementName.toHuman());
            parsed(0, "value: ");
            changeIndent(1);
        }        
        let value: Constant = parseValue();
        if self.observer!=None        {
            changeIndent(-1);
        }        
        return NameValuePair::new(&elementName, &value);
    }
    pub fn parseValue(&self) -> Constant    {
        let tag: i32 = self.input.readUnsignedByte();
        if self.observer!=None        {
            let humanTag: CstString = CstString::new(Character::toString_char(tag as char));
            parsed(1, "tag: "+humanTag.toQuoted());
        }        
        match tag{'B' =>             {
                let value: CstInteger = (CstInteger*)parseConstant();
                return CstByte::make_int(value.getValue());
            }'C' =>             {
                let value: CstInteger = (CstInteger*)parseConstant();
                let intValue: i32 = value.getValue();
                return CstChar::make_int(value.getValue());
            }'D' =>             {
                let value: CstDouble = (CstDouble*)parseConstant();
                return value;
            }'F' =>             {
                let value: CstFloat = (CstFloat*)parseConstant();
                return value;
            }'I' =>             {
                let value: CstInteger = (CstInteger*)parseConstant();
                return value;
            }'J' =>             {
                let value: CstLong = (CstLong*)parseConstant();
                return value;
            }'S' =>             {
                let value: CstInteger = (CstInteger*)parseConstant();
                return CstShort::make_int(value.getValue());
            }'Z' =>             {
                let value: CstInteger = (CstInteger*)parseConstant();
                return CstBoolean::make_int(value.getValue());
            }'c' =>             {
                let classInfoIndex: i32 = self.input.readUnsignedShort();
                let value: CstString = (CstString*)self.pool.get(classInfoIndex);
                let type: Type = Type::internReturnType(value.getString());
                if self.observer!=None                {
                    parsed(2, "class_info: "+type_renamed.toHuman());
                }                
                return CstType::new(&type_renamed);
            }'s' =>             {
                return parseConstant();
            }'e' =>             {
                requireLength(4);
                let typeNameIndex: i32 = self.input.readUnsignedShort();
                let constNameIndex: i32 = self.input.readUnsignedShort();
                let typeName: CstString = (CstString*)self.pool.get(typeNameIndex);
                let constName: CstString = (CstString*)self.pool.get(constNameIndex);
                if self.observer!=None                {
                    parsed(2, "type_name: "+typeName.toHuman());
                    parsed(2, "const_name: "+constName.toHuman());
                }                
                return CstEnumRef::new(CstNat::new(&constName, &typeName));
            }'@' =>             {
                let annotation: Annotation = parseAnnotation(&AnnotationVisibility::EMBEDDED);
                return CstAnnotation::new(&annotation);
            }'[' =>             {
                requireLength(2);
                let numValues: i32 = self.input.readUnsignedShort();
                let list: List = CstArray.List::new(numValues);
                if self.observer!=None                {
                    parsed(2, "num_values: "+numValues);
                    changeIndent(1);
                }                
                for(                let i: i32 = 0;;i<numValuesi += 1)                {
                    if self.observer!=None                    {
                        changeIndent(-1);
                        parsed(0, "element_value["+i+"]:");
                        changeIndent(1);
                    }                    
                    list.set(i, parseValue());
                }
                if self.observer!=None                {
                    changeIndent(-1);
                }                
                list.setImmutable();
                return CstArray::new(&list);
            }        _ => {}        {
            throw ParseException::new("unknown annotation tag: "+Hex::u1(tag));
        }    }
}
pub fn parseConstant(&self) -> Constant{
    let constValueIndex: i32 = self.input.readUnsignedShort();
    let value: Constant = (Constant*)self.pool.get(constValueIndex);
    if self.observer!=None    {
        let human: String = if (//value instanceof CstString) { ((CstString*)value).toQuoted() } else { value.toHuman() };
                parsed(2, "constant_value: "+human);
            }            
            return value;
        }
        pub fn requireLength(&self, requiredLength: i32)        {
            if self.input.available()<requiredLength            {
                throw ParseException::new("truncated annotation attribute");
            }            
        }
        pub fn parsed(&self, length: i32, message: &String)        {
            self.observer.parsed(&self.bytes, self.parseCursor, length, &message);
            self.parseCursor+=length;
        }
        pub fn changeIndent(&self, indent: i32)        {
            self.observer.changeIndent(indent);
        }
}
