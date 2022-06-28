use crate::helper;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dx::dex::file::IndexedItem;
use crate::com::android::dx::dex::file::CatchStructs;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::dex::file::DebugInfoItem;
use crate::com::android::dx::dex::code::DalvCode::AssignIndicesCallback;

struct CodeItem{
    pub ref: CstMethodRef,
    pub code: DalvCode,
    pub catches: CatchStructs,
    pub isStatic: boolean,
    pub throwsList: TypeList,
    pub debugInfo: DebugInfoItem,
}
impl CodeItem{
    pub const ALIGNMENT: i32 = 4;
    pub const HEADER_SIZE: i32 = 16;
    pub fn new(&self, ref: &CstMethodRef, code: &DalvCode, isStatic: boolean, throwsList: &TypeList)    {
        super(ALIGNMENT,-1);

        if ref==None        {
            throw NullPointerException::new("ref == null");
        }        
        if code==None        {
            throw NullPointerException::new("code == null");
        }        
        if throwsList==None        {
            throw NullPointerException::new("throwsList == null");
        }        
        self->ref=ref;
        self->code=code;
        self->isStatic=isStatic;
        self->throwsList=throwsList;
        self->catches=None;
        self->debugInfo=None;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_CODE_ITEM;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let byteData: MixedItemSection = file.getByteData();
        let typeIds: TypeIdsSection = file.getTypeIds();
        if self.code.hasPositions()||self.code.hasLocals()        {
            self.debugInfo=DebugInfoItem::new(&self.code, self.isStatic, &self.ref_renamed);
            byteData.add(&self.debugInfo);
        }        
        if self.code.hasAnyCatches()        {
            for type in self.code.getCatchTypes()            {
                typeIds.intern_Type(&type_renamed);
            }
            self.catches=CatchStructs::new(&self.code);
        }        
        for c in self.code.getInsnConstants()        {
            file.internIfAppropriate(&c);
        }
    }
    pub fn toString(&self) -> String    {
        return "CodeItem{"+toHuman()+"}";
    }
    pub fn toHuman(&self) -> String    {
        return self.ref_renamed.toHuman();
    }
    pub fn getRef(&self) -> CstMethodRef    {
        return self.ref_renamed;
    }
    pub fn debugPrint(&self, out: &PrintWriter, prefix: &String, verbose: boolean)    {
        out.println_String(self.ref_renamed.toHuman()+":");
        let insns: DalvInsnList = self.code.getInsns();
        out.println_String("regs: "+Hex::u2(getRegistersSize())+"; ins: "+Hex::u2(getInsSize())+"; outs: "+Hex::u2(getOutsSize()));
        insns.debugPrint_Writer_String_boolean(&out, &prefix, verbose);
        let prefix2: String = prefix+"  ";
        if self.catches!=None        {
            out.print_String(&prefix);
            out.println_String("catches");
            self.catches.debugPrint(&out, &prefix2);
        }        
        if self.debugInfo!=None        {
            out.print_String(&prefix);
            out.println_String("debug info");
            self.debugInfo.debugPrint(&out, &prefix2);
        }        
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
        let file: DexFile = addedTo.getFile();
        let catchesSize: i32;
        self.code.assignIndices(/*new DalvCode.AssignIndicesCallback(){
  @Override public int getIndex(  Constant cst){
    IndexedItem item=file.findItemOrNull(cst);
    if (item == null) {
      return -1;
    }
    return item.getIndex();
  }
}
*/);
        if self.catches!=None        {
            self.catches.encode(&file);
            catchesSize=self.catches.writeSize();
        }        else         {
            catchesSize=0;
        }
        let insnsSize: i32 = self.code.getInsns().codeSize();
        if (insnsSize&1)!=0        {
            insnsSize += 1;
        }        
        setWriteSize(CodeItem::HEADER_SIZE+(insnsSize*2)+catchesSize);
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let annotates: boolean = out.annotates();
        let regSz: i32 = getRegistersSize();
        let outsSz: i32 = getOutsSize();
        let insSz: i32 = getInsSize();
        let insnsSz: i32 = self.code.getInsns().codeSize();
        let needPadding: boolean = (insnsSz&1)!=0;
        let triesSz: i32 = if (self.catches==None) { 0 } else { self.catches.triesSize() };
                let debugOff: i32 = if (self.debugInfo==None) { 0 } else { self.debugInfo.getAbsoluteOffset() };
                        if annotates                        {
                            out.annotate_int_String(0, offsetString()+' '+self.ref_renamed.toHuman());
                            out.annotate_int_String(2, "  registers_size: "+Hex::u2(regSz));
                            out.annotate_int_String(2, "  ins_size:       "+Hex::u2(insSz));
                            out.annotate_int_String(2, "  outs_size:      "+Hex::u2(outsSz));
                            out.annotate_int_String(2, "  tries_size:     "+Hex::u2(triesSz));
                            out.annotate_int_String(4, "  debug_off:      "+Hex::u4(debugOff));
                            out.annotate_int_String(4, "  insns_size:     "+Hex::u4(insnsSz));
                            let size: i32 = self.throwsList.size();
                            if size!=0                            {
                                out.annotate_int_String(0, "  throws "+StdTypeList::toHuman(&self.throwsList));
                            }                            
                        }                        
                        out.writeShort(regSz);
                        out.writeShort(insSz);
                        out.writeShort(outsSz);
                        out.writeShort(triesSz);
                        out.writeInt(debugOff);
                        out.writeInt(insnsSz);
                        writeCodes(&file, &out);
                        if self.catches!=None                        {
                            if needPadding                            {
                                if annotates                                {
                                    out.annotate_int_String(2, "  padding: 0");
                                }                                
                                out.writeShort(0);
                            }                            
                            self.catches.writeTo(&file, &out);
                        }                        
                        if annotates                        {
                            if self.debugInfo!=None                            {
                                out.annotate_int_String(0, "  debug info");
                                self.debugInfo.annotateTo(&file, &out, "    ");
                            }                            
                        }                        
                    }
                    pub fn writeCodes(&self, file: &DexFile, out: &AnnotatedOutput)                    {
                        let insns: DalvInsnList = self.code.getInsns();
                        try                        {
                            insns.writeTo(&out);
                        }                        catch(                        let ex: RuntimeException)                        {
                            throw ExceptionWithContext::withContext(&ex, "...while writing "+"instructions for "+self.ref_renamed.toHuman());
                        }
                    }
                    pub fn getInsSize(&self) -> i32                    {
                        return self.ref_renamed.getParameterWordCount(self.isStatic);
                    }
                    pub fn getOutsSize(&self) -> i32                    {
                        return self.code.getInsns().getOutsSize();
                    }
                    pub fn getRegistersSize(&self) -> i32                    {
                        return self.code.getInsns().getRegistersSize();
                    }
}
