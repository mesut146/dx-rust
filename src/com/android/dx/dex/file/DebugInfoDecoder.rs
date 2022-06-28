use crate::helper;
use crate::com::android::dx::dex::code::PositionList::Entry;
use crate::com::android::dex::util::ByteArrayByteInput;
use crate::com::android::dx::dex::file::DebugInfoDecoder::PositionEntry;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::dex::code::DalvCode;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::code::LocalList;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::file::DebugInfoDecoder::LocalEntry;
use crate::com::android::dx::dex::code::LocalList::Entry;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dex::util::ByteInput;
use crate::com::android::dex::Leb128;
use crate::com::android::dx::dex::code::LocalList::Disposition;
use crate::com::android::dx::dex::code::DalvInsnList;
use crate::com::android::dx::dex::file::DebugInfoDecoder;
use crate::com::android::dx::dex::code::PositionList;

struct DebugInfoDecoder{
    pub encoded: Vec<i8>,
    pub positions: ArrayList<PositionEntry>,
    pub locals: ArrayList<LocalEntry>,
    pub codesize: i32,
    pub lastEntryForReg: Vec<LocalEntry>,
    pub desc: Prototype,
    pub isStatic: boolean,
    pub file: DexFile,
    pub regSize: i32,
    pub line: i32,
    pub address: i32,
    pub thisStringIdx: i32,
}
impl DebugInfoDecoder{
    pub fn new(&self, encoded: &Vec<i8>, codesize: i32, regSize: i32, isStatic: boolean, ref: &CstMethodRef, file: &DexFile)    {
        if encoded==None        {
            throw NullPointerException::new("encoded == null");
        }        
        self->encoded=encoded;
        self->isStatic=isStatic;
        self->desc=ref.getPrototype();
        self->file=file;
        self->regSize=regSize;
        self.positions=ArrayList<PositionEntry>::new();
        self.locals=ArrayList<LocalEntry>::new();
        self->codesize=codesize;
        self.lastEntryForReg=new LocalEntry[regSize];
        let idx: i32 = -1;
        try        {
            idx=file.getStringIds().indexOf(CstString::new("this"));
        }        catch(        let ex: IllegalArgumentException)        {
        }
        self.thisStringIdx=idx;
    }
    pub fn getPositionList(&self) -> List<PositionEntry>    {
        return self.positions;
    }
    pub fn getLocals(&self) -> List<LocalEntry>    {
        return self.locals;
    }
    pub fn decode(&self)    {
        try        {
            decode0();
        }        catch(        let ex: Exception)        {
            throw ExceptionWithContext::withContext(&ex, "...while decoding debug info");
        }
    }
    pub fn readStringIndex(&self, bs: &ByteInput) -> i32    {
        let offsetIndex: i32 = Leb128::readUnsignedLeb128(&bs);
        return offsetIndex-1;
    }
    pub fn getParamBase(&self) -> i32    {
        return self.regSize-self.desc.getParameterTypes().getWordCount()-(if self.isStatic { 0 } else { 1 });
            }
            pub fn decode0(&self)            {
                let bs: ByteInput = ByteArrayByteInput::new(&self.encoded);
                self.line=Leb128::readUnsignedLeb128(&bs);
                let szParams: i32 = Leb128::readUnsignedLeb128(&bs);
                let params: StdTypeList = self.desc.getParameterTypes();
                let curReg: i32 = getParamBase();
                if szParams!=params.size()                {
                    throw RuntimeException::new("Mismatch between parameters_size and prototype");
                }                
                if !self.isStatic                {
                    let thisEntry: LocalEntry = LocalEntry::new(0, true, curReg, self.thisStringIdx, 0, 0);
                    self.locals.add_LocalEntry(&thisEntry);
                    self.lastEntryForReg[curReg]=thisEntry;
                    curReg += 1;
                }                
                for(                let i: i32 = 0;;i<szParamsi += 1)                {
                    let paramType: Type = params.getType(i);
                    let le: LocalEntry;
                    let nameIdx: i32 = readStringIndex(&bs);
                    if nameIdx==-1                    {
                        le=LocalEntry::new(0, true, curReg, -1, 0, 0);
                    }                    else                     {
                        le=LocalEntry::new(0, true, curReg, nameIdx, 0, 0);
                    }
                    self.locals.add_LocalEntry(&le);
                    self.lastEntryForReg[curReg]=le;
                    curReg+=paramType.getCategory();
                }
                for(;)                {
                    let opcode: i32 = bs.readByte()&0xff;
                    match opcode{DebugInfoConstants::DBG_START_LOCAL =>                         {
                            let reg: i32 = Leb128::readUnsignedLeb128(&bs);
                            let nameIdx: i32 = readStringIndex(&bs);
                            let typeIdx: i32 = readStringIndex(&bs);
                            let le: LocalEntry = LocalEntry::new(self.address, true, reg, nameIdx, typeIdx, 0);
                            self.locals.add_LocalEntry(&le);
                            self.lastEntryForReg[reg]=le;
                        }                        break;DebugInfoConstants::DBG_START_LOCAL_EXTENDED =>                         {
                            let reg: i32 = Leb128::readUnsignedLeb128(&bs);
                            let nameIdx: i32 = readStringIndex(&bs);
                            let typeIdx: i32 = readStringIndex(&bs);
                            let sigIdx: i32 = readStringIndex(&bs);
                            let le: LocalEntry = LocalEntry::new(self.address, true, reg, nameIdx, typeIdx, sigIdx);
                            self.locals.add_LocalEntry(&le);
                            self.lastEntryForReg[reg]=le;
                        }                        break;DebugInfoConstants::DBG_RESTART_LOCAL =>                         {
                            let reg: i32 = Leb128::readUnsignedLeb128(&bs);
                            let prevle: LocalEntry;
                            let le: LocalEntry;
                            try                            {
                                prevle=self.lastEntryForReg[reg];
                                if                                 {
                                    throw RuntimeException::new("nonsensical "+"RESTART_LOCAL on live register v"+reg);
                                }                                
                                le=LocalEntry::new(self.address, true, reg, , , 0);
                            }                            catch(                            let ex: NullPointerException)                            {
                                throw RuntimeException::new("Encountered RESTART_LOCAL on new v"+reg);
                            }
                            self.locals.add_LocalEntry(&le);
                            self.lastEntryForReg[reg]=le;
                        }                        break;DebugInfoConstants::DBG_END_LOCAL =>                         {
                            let reg: i32 = Leb128::readUnsignedLeb128(&bs);
                            let prevle: LocalEntry;
                            let le: LocalEntry;
                            try                            {
                                prevle=self.lastEntryForReg[reg];
                                if !                                {
                                    throw RuntimeException::new("nonsensical "+"END_LOCAL on dead register v"+reg);
                                }                                
                                le=LocalEntry::new(self.address, false, reg, , , );
                            }                            catch(                            let ex: NullPointerException)                            {
                                throw RuntimeException::new("Encountered END_LOCAL on new v"+reg);
                            }
                            self.locals.add_LocalEntry(&le);
                            self.lastEntryForReg[reg]=le;
                        }                        break;DebugInfoConstants::DBG_END_SEQUENCE =>                         return;DebugInfoConstants::DBG_ADVANCE_PC =>                         self.address+=Leb128::readUnsignedLeb128(&bs);                        break;DebugInfoConstants::DBG_ADVANCE_LINE =>                         self.line+=Leb128::readSignedLeb128(&bs);                        break;DebugInfoConstants::DBG_SET_PROLOGUE_END =>                         break;DebugInfoConstants::DBG_SET_EPILOGUE_BEGIN =>                         break;DebugInfoConstants::DBG_SET_FILE =>                         break;                    _ => {}                    if opcode<DebugInfoConstants::DBG_FIRST_SPECIAL                    {
                        throw RuntimeException::new("Invalid extended opcode encountered "+opcode);
                    }                                        let adjopcode: i32 = opcode-DebugInfoConstants::DBG_FIRST_SPECIAL;                    self.address+=adjopcode/DebugInfoConstants::DBG_LINE_RANGE;                    self.line+=DebugInfoConstants::DBG_LINE_BASE+(adjopcode%DebugInfoConstants::DBG_LINE_RANGE);                    self.positions.add_PositionEntry(PositionEntry::new(self.address, self.line));                    break;                }
            }
        }
        pub fn validateEncode(info: &Vec<i8>, file: &DexFile, ref: &CstMethodRef, code: &DalvCode, isStatic: boolean)        {
            let pl: PositionList = code.getPositions();
            let ll: LocalList = code.getLocals();
            let insns: DalvInsnList = code.getInsns();
            let codeSize: i32 = insns.codeSize();
            let countRegisters: i32 = insns.getRegistersSize();
            try            {
                DebugInfoDecoder::validateEncode0(&info, codeSize, countRegisters, isStatic, &ref, &file, &pl, &ll);
            }            catch(            let ex: RuntimeException)            {
                System::err.println_String("instructions:");
                insns.debugPrint_OutputStream_String_boolean(&System::err, "  ", true);
                System::err.println_String("local list:");
                ll.debugPrint(&System::err, "  ");
                throw ExceptionWithContext::withContext(&ex, "while processing "+ref.toHuman());
            }
        }
        pub fn validateEncode0(info: &Vec<i8>, codeSize: i32, countRegisters: i32, isStatic: boolean, ref: &CstMethodRef, file: &DexFile, pl: &PositionList, ll: &LocalList)        {
            let decoder: DebugInfoDecoder = DebugInfoDecoder::new(&info, codeSize, countRegisters, isStatic, &ref, &file);
            decoder.decode();
            let decodedEntries: List<PositionEntry> = decoder.getPositionList();
            if decodedEntries.size()!=pl.size()            {
                throw RuntimeException::new("Decoded positions table not same size was "+decodedEntries.size()+" expected "+pl.size());
            }            
            for entry in decodedEntries            {
                let found: boolean = false;
                for(                let i: i32 = pl.size()-1;;i>=0i -= 1)                {
                    let ple: Entry = pl.get(i);
                    if ==ple.getPosition().getLine()&&==ple.getAddress()                    {
                        found=true;
                        break;
                    }                    
                }
                if !found                {
                    throw RuntimeException::new("Could not match position entry: "++", "+);
                }                
            }
            let decodedLocals: List<LocalEntry> = decoder.getLocals();
            let thisStringIdx: i32 = ;
            let decodedSz: i32 = decodedLocals.size();
            let paramBase: i32 = decoder.getParamBase();
            for(            let i: i32 = 0;;i<decodedSzi += 1)            {
                let entry: LocalEntry = decodedLocals.get(i);
                let idx: i32 = ;
                if (idx<0)||(idx==thisStringIdx)                {
                    for(                    let j: i32 = i+1;;j<decodedSzj += 1)                    {
                        let e2: LocalEntry = decodedLocals.get(j);
                        if !=0                        {
                            break;
                        }                        
                        if (==)&&                        {
                            decodedLocals.set(i, &e2);
                            decodedLocals.remove_int(j);
                            decodedSz -= 1;
                            break;
                        }                        
                    }
                }                
            }
            let origSz: i32 = ll.size();
            let decodeAt: i32 = 0;
            let problem: boolean = false;
            for(            let i: i32 = 0;;i<origSzi += 1)            {
                let origEntry: Entry = ll.get(i);
                if origEntry.getDisposition()==Disposition::END_REPLACED                {
                    continue;
                }                
                let decodedEntry: LocalEntry;
                loop                {
                    decodedEntry=decodedLocals.get(decodeAt);                    if >=0                    {
                        break;
                    }                                        decodeAt += 1;                    if !decodeAt<decodedSz{ break; }                }
                let decodedAddress: i32 = ;
                if !=origEntry.getRegister()                {
                    System::err.println_String("local register mismatch at orig "+i+" / decoded "+decodeAt);
                    problem=true;
                    break;
                }                
                if !=origEntry.isStart()                {
                    System::err.println_String("local start/end mismatch at orig "+i+" / decoded "+decodeAt);
                    problem=true;
                    break;
                }                
                if (decodedAddress!=origEntry.getAddress())&&!((decodedAddress==0)&&(>=paramBase))                {
                    System::err.println_String("local address mismatch at orig "+i+" / decoded "+decodeAt);
                    problem=true;
                    break;
                }                
                decodeAt += 1;
            }
            if problem            {
                System::err.println_String("decoded locals:");
                for e in decodedLocals                {
                    System::err.println_String("  "+e);
                }
                throw RuntimeException::new("local table problem");
            }            
        }
}
