use crate::helper;
use crate::com::android::dx::dex::code::PositionList::Entry;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::code::LocalList;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::code::LocalList::Entry;
use crate::com::android::dx::util::ByteArrayAnnotatedOutput;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::code::LocalList::Disposition;
use crate::com::android::dx::dex::code::PositionList;

struct DebugInfoEncoder{
    pub positions: PositionList,
    pub locals: LocalList,
    pub output: ByteArrayAnnotatedOutput,
    pub file: DexFile,
    pub codeSize: i32,
    pub regSize: i32,
    pub desc: Prototype,
    pub isStatic: boolean,
    pub address: i32,
    pub line: i32,
    pub annotateTo: AnnotatedOutput,
    pub debugPrint: PrintWriter,
    pub prefix: String,
    pub shouldConsume: boolean,
    pub lastEntryForReg: Vec<LocalList.Entry>,
}
impl DebugInfoEncoder{
    pub const DEBUG: boolean = false;
    pub fn new(&self, positions: &PositionList, locals: &LocalList, file: &DexFile, codeSize: i32, regSize: i32, isStatic: boolean, ref: &CstMethodRef)    {
        self->positions=positions;
        self->locals=locals;
        self->file=file;
        self->desc=ref.getPrototype();
        self->isStatic=isStatic;
        self->codeSize=codeSize;
        self->regSize=regSize;
        self.output=ByteArrayAnnotatedOutput::new();
        self.lastEntryForReg=new LocalList.Entry[regSize];
    }
    pub fn annotate(&self, length: i32, message: &String)    {
        if self.prefix!=None        {
            message=self.prefix+message;
        }        
        if self.annotateTo!=None        {
            self.annotateTo.annotate_int_String(if self.shouldConsume { length } else { 0 }, &message);
                }                
                if self.debugPrint!=None                {
                    self.debugPrint.println_String(&message);
                }                
            }
            pub fn convert(&self) -> Vec<i8>            {
                try                {
                    let ret: Vec<i8>;
                    ret=convert0();
                    if DebugInfoEncoder::DEBUG                    {
                        for(                        let i: i32 = 0;;i<ret.len()i += 1)                        {
                            System::err.printf_String_Object[]("byte %02x\n", (0xff&ret[i]));
                        }
                    }                    
                    return ret;
                }                catch(                let ex: IOException)                {
                    throw ExceptionWithContext::withContext(&ex, "...while encoding debug info");
                }
            }
            pub fn convertAndAnnotate(&self, prefix: &String, debugPrint: &PrintWriter, out: &AnnotatedOutput, consume: boolean) -> Vec<i8>            {
                self->prefix=prefix;
                self->debugPrint=debugPrint;
                self.annotateTo=out;
                self.shouldConsume=consume;
                let result: Vec<i8> = convert();
                return result;
            }
            pub fn convert0(&self) -> Vec<i8>            {
                let sortedPositions: ArrayList<Entry> = buildSortedPositions();
                let methodArgs: ArrayList<Entry> = extractMethodArguments();
                emitHeader(&sortedPositions, &methodArgs);
                self.output.writeByte(DebugInfoConstants::DBG_SET_PROLOGUE_END);
                if self.annotateTo!=None||self.debugPrint!=None                {
                    annotate(1, String::format_String_Object[]("%04x: prologue end", self.address));
                }                
                let positionsSz: i32 = sortedPositions.size();
                let localsSz: i32 = self.locals.size();
                let curPositionIdx: i32 = 0;
                let curLocalIdx: i32 = 0;
                for(;)                {
                    curLocalIdx=emitLocalsAtAddress(curLocalIdx);
                    curPositionIdx=emitPositionsAtAddress(curPositionIdx, &sortedPositions);
                    let nextAddrL: i32 = Integer::MAX_VALUE;
                    let nextAddrP: i32 = Integer::MAX_VALUE;
                    if curLocalIdx<localsSz                    {
                        nextAddrL=self.locals.get(curLocalIdx).getAddress();
                    }                    
                    if curPositionIdx<positionsSz                    {
                        nextAddrP=sortedPositions.get(curPositionIdx).getAddress();
                    }                    
                    let next: i32 = Math::min_int_int(nextAddrP, nextAddrL);
                    if next==Integer::MAX_VALUE                    {
                        break;
                    }                    
                    if next==self.codeSize&&nextAddrL==Integer::MAX_VALUE&&nextAddrP==Integer::MAX_VALUE                    {
                        break;
                    }                    
                    if next==nextAddrP                    {
                        emitPosition(sortedPositions.get(curPositionIdx += 1));
                    }                    else                     {
                        emitAdvancePc(next-self.address);
                    }
                }
                emitEndSequence();
                return self.output.toByteArray();
            }
            pub fn emitLocalsAtAddress(&self, curLocalIdx: i32) -> i32            {
                let sz: i32 = self.locals.size();
                while (curLocalIdx<sz)&&(self.locals.get(curLocalIdx).getAddress()==self.address)                {
                    let entry: Entry = self.locals.get(curLocalIdx += 1);
                    let reg: i32 = entry.getRegister();
                    let prevEntry: Entry = self.lastEntryForReg[reg];
                    if entry==prevEntry                    {
                        continue;
                    }                    
                    self.lastEntryForReg[reg]=entry;
                    if entry.isStart()                    {
                        if (prevEntry!=None)&&entry.matches_Entry(&prevEntry)                        {
                            if prevEntry.isStart()                            {
                                throw RuntimeException::new("shouldn't happen");
                            }                            
                            emitLocalRestart(&entry);
                        }                        else                         {
                            emitLocalStart(&entry);
                        }
                    }                    else                     {
                        if entry.getDisposition()!=Disposition::END_REPLACED                        {
                            emitLocalEnd(&entry);
                        }                        
                    }
                }
                return curLocalIdx;
            }
            pub fn emitPositionsAtAddress(&self, curPositionIdx: i32, sortedPositions: &ArrayList<PositionList.Entry>) -> i32            {
                let positionsSz: i32 = sortedPositions.size();
                while (curPositionIdx<positionsSz)&&(sortedPositions.get(curPositionIdx).getAddress()==self.address)                {
                    emitPosition(sortedPositions.get(curPositionIdx += 1));
                }
                return curPositionIdx;
            }
            pub fn emitHeader(&self, sortedPositions: &ArrayList<PositionList.Entry>, methodArgs: &ArrayList<LocalList.Entry>)            {
                let annotate: boolean = (self.annotateTo!=None)||(self.debugPrint!=None);
                let mark: i32 = self.output.getCursor();
                if sortedPositions.size()>0                {
                    let entry: Entry = sortedPositions.get(0);
                    self.line=entry.getPosition().getLine();
                }                
                self.output.writeUleb128(self.line);
                if annotate                {
                    annotate(self.output.getCursor()-mark, "line_start: "+self.line);
                }                
                let curParam: i32 = getParamBase();
                let paramTypes: StdTypeList = self.desc.getParameterTypes();
                let szParamTypes: i32 = paramTypes.size();
                if !self.isStatic                {
                    for arg in methodArgs                    {
                        if curParam==arg.getRegister()                        {
                            self.lastEntryForReg[curParam]=arg;
                            break;
                        }                        
                    }
                    curParam += 1;
                }                
                mark=self.output.getCursor();
                self.output.writeUleb128(szParamTypes);
                if annotate                {
                    annotate(self.output.getCursor()-mark, String::format_String_Object[]("parameters_size: %04x", szParamTypes));
                }                
                for(                let i: i32 = 0;;i<szParamTypesi += 1)                {
                    let pt: Type = paramTypes.get(i);
                    let found: Entry = None;
                    mark=self.output.getCursor();
                    for arg in methodArgs                    {
                        if curParam==arg.getRegister()                        {
                            found=arg;
                            if arg.getSignature()!=None                            {
                                emitStringIndex(None);
                            }                            else                             {
                                emitStringIndex(arg.getName());
                            }
                            self.lastEntryForReg[curParam]=arg;
                            break;
                        }                        
                    }
                    if found==None                    {
                        emitStringIndex(None);
                    }                    
                    if annotate                    {
                        let parameterName: String = if (found==None||found.getSignature()!=None) { "<unnamed>" } else { found.getName().toHuman() };
                                annotate(self.output.getCursor()-mark, "parameter "+parameterName+" "+RegisterSpec::PREFIX+curParam);
                            }                            
                            curParam+=pt.getCategory();
                        }
                        for arg in self.lastEntryForReg                        {
                            if arg==None                            {
                                continue;
                            }                            
                            let signature: CstString = arg.getSignature();
                            if signature!=None                            {
                                emitLocalStartExtended(&arg);
                            }                            
                        }
                    }
                    pub fn buildSortedPositions(&self) -> ArrayList<PositionList.Entry>                    {
                        let sz: i32 = if (self.positions==None) { 0 } else { self.positions.size() };
                                let result: ArrayList<Entry> = ArrayList::new(sz);
                                for(                                let i: i32 = 0;;i<szi += 1)                                {
                                    result.add_Entry(self.positions.get(i));
                                }
                                Collections::sort_List<Entry>_Comparator<? super Entry>(&result, /*new Comparator<PositionList.Entry>(){
  @Override public int compare(  PositionList.Entry a,  PositionList.Entry b){
    return a.getAddress() - b.getAddress();
  }
  @Override public boolean equals(  Object obj){
    return obj == this;
  }
}
*/);
                                return result;
                            }
                            pub fn getParamBase(&self) -> i32                            {
                                return self.regSize-self.desc.getParameterTypes().getWordCount()-(if self.isStatic { 0 } else { 1 });
                                    }
                                    pub fn extractMethodArguments(&self) -> ArrayList<LocalList.Entry>                                    {
                                        let result: ArrayList<Entry> = ArrayList::new(self.desc.getParameterTypes().size());
                                        let argBase: i32 = getParamBase();
                                        let seen: BitSet = BitSet::new(self.regSize-argBase);
                                        let sz: i32 = self.locals.size();
                                        for(                                        let i: i32 = 0;;i<szi += 1)                                        {
                                            let e: Entry = self.locals.get(i);
                                            let reg: i32 = e.getRegister();
                                            if reg<argBase                                            {
                                                continue;
                                            }                                            
                                            if seen.get_int(reg-argBase)                                            {
                                                continue;
                                            }                                            
                                            seen.set_int(reg-argBase);
                                            result.add_Entry(&e);
                                        }
                                        Collections::sort_List<Entry>_Comparator<? super Entry>(&result, /*new Comparator<LocalList.Entry>(){
  @Override public int compare(  LocalList.Entry a,  LocalList.Entry b){
    return a.getRegister() - b.getRegister();
  }
  @Override public boolean equals(  Object obj){
    return obj == this;
  }
}
*/);
                                        return result;
                                    }
                                    pub fn entryAnnotationString(&self, e: &LocalList.Entry) -> String                                    {
                                        let sb: StringBuilder = StringBuilder::new();
                                        sb.append_String(&RegisterSpec::PREFIX);
                                        sb.append_int(e.getRegister());
                                        sb.append_char(' ');
                                        let name: CstString = e.getName();
                                        if name==None                                        {
                                            sb.append_String("null");
                                        }                                        else                                         {
                                            sb.append_String(name.toHuman());
                                        }
                                        sb.append_char(' ');
                                        let type: CstType = e.getType();
                                        if type_renamed==None                                        {
                                            sb.append_String("null");
                                        }                                        else                                         {
                                            sb.append_String(type_renamed.toHuman());
                                        }
                                        let signature: CstString = e.getSignature();
                                        if signature!=None                                        {
                                            sb.append_char(' ');
                                            sb.append_String(signature.toHuman());
                                        }                                        
                                        return sb.toString();
                                    }
                                    pub fn emitLocalRestart(&self, entry: &LocalList.Entry)                                    {
                                        let mark: i32 = self.output.getCursor();
                                        self.output.writeByte(DebugInfoConstants::DBG_RESTART_LOCAL);
                                        emitUnsignedLeb128(entry.getRegister());
                                        if self.annotateTo!=None||self.debugPrint!=None                                        {
                                            annotate(self.output.getCursor()-mark, String::format_String_Object[]("%04x: +local restart %s", self.address, entryAnnotationString(&entry)));
                                        }                                        
                                        if DebugInfoEncoder::DEBUG                                        {
                                            System::err.println_String("emit local restart");
                                        }                                        
                                    }
                                    pub fn emitStringIndex(&self, string: &CstString)                                    {
                                        if (string==None)||(self.file==None)                                        {
                                            self.output.writeUleb128(0);
                                        }                                        else                                         {
                                            self.output.writeUleb128(1+self.file.getStringIds().indexOf(&string));
                                        }
                                        if DebugInfoEncoder::DEBUG                                        {
                                            System::err.printf_String_Object[]("Emit string %s\n", if string==None { "<null>" } else { string.toQuoted() });
                                                }                                                
                                            }
                                            pub fn emitTypeIndex(&self, type: &CstType)                                            {
                                                if (type==None)||(self.file==None)                                                {
                                                    self.output.writeUleb128(0);
                                                }                                                else                                                 {
                                                    self.output.writeUleb128(1+self.file.getTypeIds().indexOf_CstType(&type));
                                                }
                                                if DebugInfoEncoder::DEBUG                                                {
                                                    System::err.printf_String_Object[]("Emit type %s\n", if type==None { "<null>" } else { type.toHuman() });
                                                        }                                                        
                                                    }
                                                    pub fn emitLocalStart(&self, entry: &LocalList.Entry)                                                    {
                                                        if entry.getSignature()!=None                                                        {
                                                            emitLocalStartExtended(&entry);
                                                            return;
                                                        }                                                        
                                                        let mark: i32 = self.output.getCursor();
                                                        self.output.writeByte(DebugInfoConstants::DBG_START_LOCAL);
                                                        emitUnsignedLeb128(entry.getRegister());
                                                        emitStringIndex(entry.getName());
                                                        emitTypeIndex(entry.getType());
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(self.output.getCursor()-mark, String::format_String_Object[]("%04x: +local %s", self.address, entryAnnotationString(&entry)));
                                                        }                                                        
                                                        if DebugInfoEncoder::DEBUG                                                        {
                                                            System::err.println_String("emit local start");
                                                        }                                                        
                                                    }
                                                    pub fn emitLocalStartExtended(&self, entry: &LocalList.Entry)                                                    {
                                                        let mark: i32 = self.output.getCursor();
                                                        self.output.writeByte(DebugInfoConstants::DBG_START_LOCAL_EXTENDED);
                                                        emitUnsignedLeb128(entry.getRegister());
                                                        emitStringIndex(entry.getName());
                                                        emitTypeIndex(entry.getType());
                                                        emitStringIndex(entry.getSignature());
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(self.output.getCursor()-mark, String::format_String_Object[]("%04x: +localx %s", self.address, entryAnnotationString(&entry)));
                                                        }                                                        
                                                        if DebugInfoEncoder::DEBUG                                                        {
                                                            System::err.println_String("emit local start");
                                                        }                                                        
                                                    }
                                                    pub fn emitLocalEnd(&self, entry: &LocalList.Entry)                                                    {
                                                        let mark: i32 = self.output.getCursor();
                                                        self.output.writeByte(DebugInfoConstants::DBG_END_LOCAL);
                                                        self.output.writeUleb128(entry.getRegister());
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(self.output.getCursor()-mark, String::format_String_Object[]("%04x: -local %s", self.address, entryAnnotationString(&entry)));
                                                        }                                                        
                                                        if DebugInfoEncoder::DEBUG                                                        {
                                                            System::err.println_String("emit local end");
                                                        }                                                        
                                                    }
                                                    pub fn emitPosition(&self, entry: &PositionList.Entry)                                                    {
                                                        let pos: SourcePosition = entry.getPosition();
                                                        let newLine: i32 = pos.getLine();
                                                        let newAddress: i32 = entry.getAddress();
                                                        let opcode: i32;
                                                        let deltaLines: i32 = newLine-self.line;
                                                        let deltaAddress: i32 = newAddress-self.address;
                                                        if deltaAddress<0                                                        {
                                                            throw RuntimeException::new("Position entries must be in ascending address order");
                                                        }                                                        
                                                        if (deltaLines<DebugInfoConstants::DBG_LINE_BASE)||(deltaLines>(DebugInfoConstants::DBG_LINE_BASE+DebugInfoConstants::DBG_LINE_RANGE-1))                                                        {
                                                            emitAdvanceLine(deltaLines);
                                                            deltaLines=0;
                                                        }                                                        
                                                        opcode=DebugInfoEncoder::computeOpcode(deltaLines, deltaAddress);
                                                        if (opcode&~0xff)>0                                                        {
                                                            emitAdvancePc(deltaAddress);
                                                            deltaAddress=0;
                                                            opcode=DebugInfoEncoder::computeOpcode(deltaLines, deltaAddress);
                                                            if (opcode&~0xff)>0                                                            {
                                                                emitAdvanceLine(deltaLines);
                                                                deltaLines=0;
                                                                opcode=DebugInfoEncoder::computeOpcode(deltaLines, deltaAddress);
                                                            }                                                            
                                                        }                                                        
                                                        self.output.writeByte(opcode);
                                                        self.line+=deltaLines;
                                                        self.address+=deltaAddress;
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(1, String::format_String_Object[]("%04x: line %d", self.address, self.line));
                                                        }                                                        
                                                    }
                                                    pub fn computeOpcode(deltaLines: i32, deltaAddress: i32) -> i32                                                    {
                                                        if deltaLines<DebugInfoConstants::DBG_LINE_BASE||deltaLines>(DebugInfoConstants::DBG_LINE_BASE+DebugInfoConstants::DBG_LINE_RANGE-1)                                                        {
                                                            throw RuntimeException::new("Parameter out of range");
                                                        }                                                        
                                                        return (deltaLines-DebugInfoConstants::DBG_LINE_BASE)+(DebugInfoConstants::DBG_LINE_RANGE*deltaAddress)+DebugInfoConstants::DBG_FIRST_SPECIAL;
                                                    }
                                                    pub fn emitAdvanceLine(&self, deltaLines: i32)                                                    {
                                                        let mark: i32 = self.output.getCursor();
                                                        self.output.writeByte(DebugInfoConstants::DBG_ADVANCE_LINE);
                                                        self.output.writeSleb128(deltaLines);
                                                        self.line+=deltaLines;
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(self.output.getCursor()-mark, String::format_String_Object[]("line = %d", self.line));
                                                        }                                                        
                                                        if DebugInfoEncoder::DEBUG                                                        {
                                                            System::err.printf_String_Object[]("Emitting advance_line for %d\n", deltaLines);
                                                        }                                                        
                                                    }
                                                    pub fn emitAdvancePc(&self, deltaAddress: i32)                                                    {
                                                        let mark: i32 = self.output.getCursor();
                                                        self.output.writeByte(DebugInfoConstants::DBG_ADVANCE_PC);
                                                        self.output.writeUleb128(deltaAddress);
                                                        self.address+=deltaAddress;
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(self.output.getCursor()-mark, String::format_String_Object[]("%04x: advance pc", self.address));
                                                        }                                                        
                                                        if DebugInfoEncoder::DEBUG                                                        {
                                                            System::err.printf_String_Object[]("Emitting advance_pc for %d\n", deltaAddress);
                                                        }                                                        
                                                    }
                                                    pub fn emitUnsignedLeb128(&self, n: i32)                                                    {
                                                        if n<0                                                        {
                                                            throw RuntimeException::new("Signed value where unsigned required: "+n);
                                                        }                                                        
                                                        self.output.writeUleb128(n);
                                                    }
                                                    pub fn emitEndSequence(&self)                                                    {
                                                        self.output.writeByte(DebugInfoConstants::DBG_END_SEQUENCE);
                                                        if self.annotateTo!=None||self.debugPrint!=None                                                        {
                                                            annotate(1, "end sequence");
                                                        }                                                        
                                                    }
}
