use crate::helper;
use crate::com::android::dx::rop::code::DexTranslationAdvice;
use crate::com::android::dx::dex::cf::AttributeTranslator;
use crate::com::android::dx::cf::iface::MethodList;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::dex::cf::OptimizerOptions;
use crate::com::android::dx::command::dexer::DxContext;
use crate::com::android::dx::dex::cf::CodeStatistics;
use crate::com::android::dx::cf::iface::Method;
use crate::com::android::dx::dex::file::MethodHandlesSection;
use crate::com::android::dx::cf::code::BootstrapMethodsList;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::cf::code::BytecodeArray;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dx::rop::cst::CstCallSite;
use crate::com::android::dx::rop::cst::CstFieldRef;
use crate::com::android::dx::cf::code::Ropper;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::cf::code::ConcreteMethod;
use crate::com::android::dx::rop::code::AccessFlags;
use crate::com::android::dx::dex::cf::CfOptions;
use crate::com::android::dx::dex::file::CallSiteIdsSection;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::cf::iface::Field;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::annotation::Annotations;
use crate::com::android::dx::cf::direct::DirectClassFile;
use crate::com::android::dx::rop::cst::CstByte;
use crate::com::android::dx::dex::code::DalvCode::AssignIndicesCallback;
use crate::com::android::dx::cf::iface::FieldList;
use crate::com::android::dx::dex::code::RopTranslator;
use crate::com::android::dx::dex::file::EncodedField;
use crate::com::android::dx::rop::cst::CstInterfaceMethodRef;
use crate::com::android::dx::rop::cst::CstEnumRef;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::ssa::Optimizer;
use crate::com::android::dx::cf::code::BootstrapMethodsList::Item;
use crate::com::android::dx::dex::file::EncodedMethod;
use crate::com::android::dx::rop::cst::CstChar;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::rop::cst::CstShort;
use crate::com::android::dx::rop::annotation::AnnotationsList;
use crate::com::android::dx::dex::file::ClassDefItem;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::rop::cst::CstBoolean;
use crate::com::android::dx::rop::cst::TypedConstant;
use crate::com::android::dx::dex::code::PositionList;
use crate::com::android::dx::rop::code::LocalVariableExtractor;

struct CfTranslator{
}
impl CfTranslator{
    pub const DEBUG: boolean = false;
    pub fn new(&self)    {
    }
    pub fn translate(context: &DxContext, cf: &DirectClassFile, bytes: &Vec<i8>, cfOptions: &CfOptions, dexOptions: &DexOptions, dexFile: &DexFile) -> ClassDefItem    {
        try        {
            return CfTranslator::translate0(&context, &cf, &bytes, &cfOptions, &dexOptions, &dexFile);
        }        catch(        let ex: RuntimeException)        {
            let msg: String = "...while processing "+cf.getFilePath();
            throw ExceptionWithContext::withContext(&ex, &msg);
        }
    }
    pub fn translate0(context: &DxContext, cf: &DirectClassFile, bytes: &Vec<i8>, cfOptions: &CfOptions, dexOptions: &DexOptions, dexFile: &DexFile) -> ClassDefItem    {
        .loadOptimizeLists(&, &);
        let thisClass: CstType = cf.getThisClass();
        let classAccessFlags: i32 = cf.getAccessFlags()&~AccessFlags::ACC_SUPER;
        let sourceFile: CstString = if (==PositionList::NONE) { None } else { cf.getSourceFile() };
                let out: ClassDefItem = ClassDefItem::new(&thisClass, classAccessFlags, cf.getSuperclass(), cf.getInterfaces(), &sourceFile);
                let classAnnotations: Annotations = AttributeTranslator::getClassAnnotations(&cf, &cfOptions);
                if classAnnotations.size()!=0                {
                    out.setClassAnnotations(&classAnnotations, &dexFile);
                }                
                let fieldIdsSection: FieldIdsSection = dexFile.getFieldIds();
                let methodIdsSection: MethodIdsSection = dexFile.getMethodIds();
                let methodHandlesSection: MethodHandlesSection = dexFile.getMethodHandles();
                let callSiteIds: CallSiteIdsSection = dexFile.getCallSiteIds();
                CfTranslator::processFields(&cf, &out, &dexFile);
                CfTranslator::processMethods(&context, &cf, &cfOptions, &dexOptions, &out, &dexFile);
                let constantPool: ConstantPool = cf.getConstantPool();
                let constantPoolSize: i32 = constantPool.size();
                for(                let i: i32 = 0;;i<constantPoolSizei += 1)                {
                    let constant: Constant = constantPool.getOrNull(i);
                    if //constant instanceof CstMethodRef                    {
                        methodIdsSection.intern((CstBaseMethodRef*)constant);
                    }                    else                     if //constant instanceof CstInterfaceMethodRef                    {
                        methodIdsSection.intern(((CstInterfaceMethodRef*)constant).toMethodRef());
                    }                    else                     if //constant instanceof CstFieldRef                    {
                        fieldIdsSection.intern((CstFieldRef*)constant);
                    }                    else                     if //constant instanceof CstEnumRef                    {
                        fieldIdsSection.intern(((CstEnumRef*)constant).getFieldRef());
                    }                    else                     if //constant instanceof CstMethodHandle                    {
                        methodHandlesSection.intern((CstMethodHandle*)constant);
                    }                    else                     if //constant instanceof CstInvokeDynamic                    {
                        let cstInvokeDynamic: CstInvokeDynamic = (CstInvokeDynamic*)constant;
                        let index: i32 = cstInvokeDynamic.getBootstrapMethodIndex();
                        let bootstrapMethod: Item = cf.getBootstrapMethods().get(index);
                        let callSite: CstCallSite = CstCallSite::make(bootstrapMethod.getBootstrapMethodHandle(), cstInvokeDynamic.getNat(), bootstrapMethod.getBootstrapMethodArguments());
                        cstInvokeDynamic.setDeclaringClass(cf.getThisClass());
                        cstInvokeDynamic.setCallSite(&callSite);
                        for ref in cstInvokeDynamic.getReferences()                        {
                            callSiteIds.intern(&ref_renamed);
                        }
                    }                    
                }
                return out;
            }
            pub fn processFields(cf: &DirectClassFile, out: &ClassDefItem, dexFile: &DexFile)            {
                let thisClass: CstType = cf.getThisClass();
                let fields: FieldList = cf.getFields();
                let sz: i32 = fields.size();
                for(                let i: i32 = 0;;i<szi += 1)                {
                    let one: Field = fields.get(i);
                    try                    {
                        let field: CstFieldRef = CstFieldRef::new(&thisClass, one.getNat());
                        let accessFlags: i32 = one.getAccessFlags();
                        if AccessFlags::isStatic(accessFlags)                        {
                            let constVal: TypedConstant = one.getConstantValue();
                            let fi: EncodedField = EncodedField::new(&field, accessFlags);
                            if constVal!=None                            {
                                constVal=CfTranslator::coerceConstant(&constVal, field.getType());
                            }                            
                            out.addStaticField(&fi, &constVal);
                        }                        else                         {
                            let fi: EncodedField = EncodedField::new(&field, accessFlags);
                            out.addInstanceField(&fi);
                        }
                        let annotations: Annotations = AttributeTranslator::getAnnotations(one.getAttributes());
                        if annotations.size()!=0                        {
                            out.addFieldAnnotations(&field, &annotations, &dexFile);
                        }                        
                        dexFile.getFieldIds().intern(&field);
                    }                    catch(                    let ex: RuntimeException)                    {
                        let msg: String = "...while processing "+one.getName().toHuman()+" "+one.getDescriptor().toHuman();
                        throw ExceptionWithContext::withContext(&ex, &msg);
                    }
                }
            }
            pub fn coerceConstant(constant: &TypedConstant, type: &Type) -> TypedConstant            {
                let constantType: Type = constant.getType();
                if constantType.equals(&type)                {
                    return constant;
                }                
                match type.getBasicType(){Type::BT_BOOLEAN =>                     {
                        return CstBoolean::make_int(((CstInteger*)constant).getValue());
                    }Type::BT_BYTE =>                     {
                        return CstByte::make_int(((CstInteger*)constant).getValue());
                    }Type::BT_CHAR =>                     {
                        return CstChar::make_int(((CstInteger*)constant).getValue());
                    }Type::BT_SHORT =>                     {
                        return CstShort::make_int(((CstInteger*)constant).getValue());
                    }                _ => {}                {
                    throw UnsupportedOperationException::new("can't coerce "+constant+" to "+type);
                }            }
        }
        pub fn processMethods(context: &DxContext, cf: &DirectClassFile, cfOptions: &CfOptions, dexOptions: &DexOptions, out: &ClassDefItem, dexFile: &DexFile)        {
            let thisClass: CstType = cf.getThisClass();
            let methods: MethodList = cf.getMethods();
            let sz: i32 = methods.size();
            for(            let i: i32 = 0;;i<szi += 1)            {
                let one: Method = methods.get(i);
                try                {
                    let meth: CstMethodRef = CstMethodRef::new(&thisClass, one.getNat());
                    let accessFlags: i32 = one.getAccessFlags();
                    let isStatic: boolean = AccessFlags::isStatic(accessFlags);
                    let isPrivate: boolean = AccessFlags::isPrivate(accessFlags);
                    let isNative: boolean = AccessFlags::isNative(accessFlags);
                    let isAbstract: boolean = AccessFlags::isAbstract(accessFlags);
                    let isConstructor: boolean = meth.isInstanceInit()||meth.isClassInit();
                    let code: DalvCode;
                    if isNative||isAbstract                    {
                        code=None;
                    }                    else                     {
                        let concrete: ConcreteMethod = ConcreteMethod::new(&one, &cf, (!=PositionList::NONE), );
                        let advice: TranslationAdvice;
                        advice=DexTranslationAdvice::THE_ONE;
                        let rmeth: RopMethod = Ropper::convert(&concrete, &advice, &methods, &dexOptions);
                        let nonOptRmeth: RopMethod = None;
                        let paramSize: i32;
                        paramSize=meth.getParameterWordCount(isStatic);
                        let canonicalName: String = thisClass.getClassType().getDescriptor()+"."+one.getName().getString();
                        if &&.shouldOptimize(&canonicalName)                        {
                            if CfTranslator::DEBUG                            {
                                System::err.println_String("Optimizing "+canonicalName);
                            }                            
                            nonOptRmeth=rmeth;
                            rmeth=Optimizer::optimize_RopMethod_int_boolean_boolean_TranslationAdvice(&rmeth, paramSize, isStatic, , &advice);
                            if CfTranslator::DEBUG                            {
                                .compareOptimizerStep(&nonOptRmeth, paramSize, isStatic, &cfOptions, &advice, &rmeth);
                            }                            
                            if                             {
                                .updateRopStatistics(&nonOptRmeth, &rmeth);
                            }                            
                        }                        
                        let locals: LocalVariableInfo = None;
                        if                         {
                            locals=LocalVariableExtractor::extract(&rmeth);
                        }                        
                        code=RopTranslator::translate(&rmeth, , &locals, paramSize, &dexOptions);
                        if &&nonOptRmeth!=None                        {
                            CfTranslator::updateDexStatistics(&context, &cfOptions, &dexOptions, &rmeth, &nonOptRmeth, &locals, paramSize, concrete.getCode().size());
                        }                        
                    }
                    if AccessFlags::isSynchronized(accessFlags)                    {
                        accessFlags|=AccessFlags::ACC_DECLARED_SYNCHRONIZED;
                        if !isNative                        {
                            accessFlags&=~AccessFlags::ACC_SYNCHRONIZED;
                        }                        
                    }                    
                    if isConstructor                    {
                        accessFlags|=AccessFlags::ACC_CONSTRUCTOR;
                    }                    
                    let exceptions: TypeList = AttributeTranslator::getExceptions(&one);
                    let mi: EncodedMethod = EncodedMethod::new(&meth, accessFlags, &code, &exceptions);
                    if meth.isInstanceInit()||meth.isClassInit()||isStatic||isPrivate                    {
                        out.addDirectMethod(&mi);
                    }                    else                     {
                        out.addVirtualMethod(&mi);
                    }
                    let annotations: Annotations = AttributeTranslator::getMethodAnnotations(&one);
                    if annotations.size()!=0                    {
                        out.addMethodAnnotations(&meth, &annotations, &dexFile);
                    }                    
                    let list: AnnotationsList = AttributeTranslator::getParameterAnnotations(&one);
                    if list.size()!=0                    {
                        out.addParameterAnnotations(&meth, &list, &dexFile);
                    }                    
                    dexFile.getMethodIds().intern(&meth);
                }                catch(                let ex: RuntimeException)                {
                    let msg: String = "...while processing "+one.getName().toHuman()+" "+one.getDescriptor().toHuman();
                    throw ExceptionWithContext::withContext(&ex, &msg);
                }
            }
        }
        pub fn updateDexStatistics(context: &DxContext, cfOptions: &CfOptions, dexOptions: &DexOptions, optRmeth: &RopMethod, nonOptRmeth: &RopMethod, locals: &LocalVariableInfo, paramSize: i32, originalByteCount: i32)        {
            let optCode: DalvCode = RopTranslator::translate(&optRmeth, , &locals, paramSize, &dexOptions);
            let nonOptCode: DalvCode = RopTranslator::translate(&nonOptRmeth, , &locals, paramSize, &dexOptions);
            let callback: AssignIndicesCallback = /*new DalvCode.AssignIndicesCallback(){
  @Override public int getIndex(  Constant cst){
    return 0;
  }
}
*/;
            optCode.assignIndices(&callback);
            nonOptCode.assignIndices(&callback);
            .updateDexStatistics(&nonOptCode, &optCode);
            .updateOriginalByteCount(originalByteCount);
        }
}
