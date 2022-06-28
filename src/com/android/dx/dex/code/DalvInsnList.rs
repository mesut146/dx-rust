use crate::helper;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::CstInsn;
use crate::com::android::dx::dex::code::DalvInsn;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::dex::code::MultiCstInsn;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::util::IndentingWriter;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::code::DalvInsnList;

struct DalvInsnList{
    pub regCount: i32,
}
impl DalvInsnList{
    pub fn makeImmutable(list: &ArrayList<DalvInsn>, regCount: i32) -> DalvInsnList    {
        let size: i32 = list.size();
        let result: DalvInsnList = DalvInsnList::new(size, regCount);
        for(        let i: i32 = 0;;i<sizei += 1)        {
            result.set(i, list.get(i));
        }
        result.setImmutable();
        return result;
    }
    pub fn new(&self, size: i32, regCount: i32)    {
        super(size);

        self->regCount=regCount;
    }
    pub fn get(&self, n: i32) -> DalvInsn    {
        return (DalvInsn*)get0(n);
    }
    pub fn set(&self, n: i32, insn: &DalvInsn)    {
        set0(n, &insn);
    }
    pub fn codeSize(&self) -> i32    {
        let sz: i32 = size();
        if sz==0        {
            return 0;
        }        
        let last: DalvInsn = get(sz-1);
        return last.getNextAddress();
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
        let startCursor: i32 = out.getCursor();
        let sz: i32 = size();
        if out.annotates()        {
            let verbose: boolean = out.isVerbose();
            for(            let i: i32 = 0;;i<szi += 1)            {
                let insn: DalvInsn = (DalvInsn*)get0(i);
                let codeBytes: i32 = insn.codeSize()*2;
                let s: String;
                if (codeBytes!=0)||verbose                {
                    s=insn.listingString("  ", out.getAnnotationWidth(), true);
                }                else                 {
                    s=None;
                }
                if s!=None                {
                    out.annotate_int_String(codeBytes, &s);
                }                else                 if codeBytes!=0                {
                    out.annotate_int_String(codeBytes, "");
                }                
            }
        }        
        for(        let i: i32 = 0;;i<szi += 1)        {
            let insn: DalvInsn = (DalvInsn*)get0(i);
            try            {
                insn.writeTo(&out);
            }            catch(            let ex: RuntimeException)            {
                throw ExceptionWithContext::withContext(&ex, "...while writing "+insn);
            }
        }
        let written: i32 = (out.getCursor()-startCursor)/2;
        if written!=codeSize()        {
            throw RuntimeException::new("write length mismatch; expected "+codeSize()+" but actually wrote "+written);
        }        
    }
    pub fn getRegistersSize(&self) -> i32    {
        return self.regCount;
    }
    pub fn getOutsSize(&self) -> i32    {
        let sz: i32 = size();
        let result: i32 = 0;
        for(        let i: i32 = 0;;i<szi += 1)        {
            let insn: DalvInsn = (DalvInsn*)get0(i);
            let count: i32 = 0;
            if //insn instanceof CstInsn            {
                let cst: Constant = ((CstInsn*)insn).getConstant();
                if //cst instanceof CstBaseMethodRef                {
                    let methodRef: CstBaseMethodRef = (CstBaseMethodRef*)cst;
                    let isStatic: boolean = (insn.getOpcode().getFamily()==Opcodes::INVOKE_STATIC);
                    count=methodRef.getParameterWordCount(isStatic);
                }                else                 if //cst instanceof CstCallSiteRef                {
                    let invokeDynamicRef: CstCallSiteRef = (CstCallSiteRef*)cst;
                    count=invokeDynamicRef.getPrototype().getParameterTypes().getWordCount();
                }                
            }            else             if //insn instanceof MultiCstInsn            {
                if insn.getOpcode().getFamily()!=Opcodes::INVOKE_POLYMORPHIC                {
                    throw RuntimeException::new("Expecting invoke-polymorphic");
                }                
                let mci: MultiCstInsn = (MultiCstInsn*)insn;
                let proto: CstProtoRef = (CstProtoRef*)mci.getConstant(1);
                count=proto.getPrototype().getParameterTypes().getWordCount();
                count=count+1;
            }            else             {
                continue;
            }
            if count>result            {
                result=count;
            }            
        }
        return result;
    }
    pub fn debugPrint(&self, out: &Writer, prefix: &String, verbose: boolean)    {
        let iw: IndentingWriter = IndentingWriter::new(&out, 0, &prefix);
        let sz: i32 = size();
        try        {
            for(            let i: i32 = 0;;i<szi += 1)            {
                let insn: DalvInsn = (DalvInsn*)get0(i);
                let s: String;
                if (insn.codeSize()!=0)||verbose                {
                    s=insn.listingString("", 0, verbose);
                }                else                 {
                    s=None;
                }
                if s!=None                {
                    iw.write_String(&s);
                }                
            }
            iw.flush();
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new(&ex);
        }
    }
    pub fn debugPrint(&self, out: &OutputStream, prefix: &String, verbose: boolean)    {
        let w: Writer = OutputStreamWriter::new(&out);
        debugPrint_Writer_String_boolean(&w, &prefix, verbose);
        try        {
            w.flush();
        }        catch(        let ex: IOException)        {
            throw RuntimeException::new(&ex);
        }
    }
}
