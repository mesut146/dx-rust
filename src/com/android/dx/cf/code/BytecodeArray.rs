use crate::helper;
use crate::com::android::dx::rop::cst::CstFloat;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::cf::code::BytecodeArray::BaseVisitor;
use crate::com::android::dx::cf::code::BytecodeArray::ConstantParserVisitor;
use crate::com::android::dx::cf::code::BytecodeArray::Visitor;
use crate::com::android::dx::rop::cst::CstInteger;
use crate::com::android::dx::cf::code::ByteOps;
use crate::com::android::dx::util::ByteArray;
use crate::com::android::dx::rop::cst::CstLong;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstKnownNull;
use crate::com::android::dx::rop::cst::CstInvokeDynamic;
use crate::com::android::dx::cf::code::SimException;
use crate::com::android::dx::cf::code::SwitchList;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstLiteralBits;
use crate::com::android::dx::rop::cst::ConstantPool;
use crate::com::android::dx::util::Bits;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::cst::CstDouble;

let static EMPTY_VISITOR: Visitor = BaseVisitor::new();
struct BytecodeArray{
    pub bytes: ByteArray,
    pub pool: ConstantPool,
}
impl BytecodeArray{
    pub fn new(&self, bytes: &ByteArray, pool: &ConstantPool)    {
        if bytes==None        {
            throw NullPointerException::new("bytes == null");
        }        
        if pool==None        {
            throw NullPointerException::new("pool == null");
        }        
        self->bytes=bytes;
        self->pool=pool;
    }
    pub fn getBytes(&self) -> ByteArray    {
        return self.bytes;
    }
    pub fn size(&self) -> i32    {
        return self.bytes.size();
    }
    pub fn byteLength(&self) -> i32    {
        return 4+self.bytes.size();
    }
    pub fn forEach(&self, visitor: &Visitor)    {
        let sz: i32 = self.bytes.size();
        let at: i32 = 0;
        while at<sz        {
            at+=parseInstruction(at, &visitor);
        }
    }
    pub fn getInstructionOffsets(&self) -> Vec<i32>    {
        let sz: i32 = self.bytes.size();
        let result: Vec<i32> = Bits::makeBitSet(sz);
        let at: i32 = 0;
        while at<sz        {
            Bits::set_int[]_int_boolean(&result, at, true);
            let length: i32 = parseInstruction(at, None);
            at+=length;
        }
        return result;
    }
    pub fn processWorkSet(&self, workSet: &Vec<i32>, visitor: &Visitor)    {
        if visitor==None        {
            throw NullPointerException::new("visitor == null");
        }        
        for(;)        {
            let offset: i32 = Bits::findFirst_int[]_int(&workSet, 0);
            if offset<0            {
                break;
            }            
            Bits::clear(&workSet, offset);
            parseInstruction(offset, &visitor);
            visitor.setPreviousOffset(offset);
        }
    }
    pub fn parseInstruction(&self, offset: i32, visitor: &Visitor) -> i32    {
        if visitor==None        {
            visitor=BytecodeArray::EMPTY_VISITOR;
        }        
        try        {
            let opcode: i32 = self.bytes.getUnsignedByte(offset);
            let info: i32 = ByteOps::opInfo(opcode);
            let fmt: i32 = info&ByteOps::FMT_MASK;
            match opcode{ByteOps::NOP =>                 {
                    visitor.visitNoArgs(opcode, offset, 1, &Type::VOID);
                    return 1;
                }ByteOps::ACONST_NULL =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstKnownNull::THE_ONE, 0);
                    return 1;
                }ByteOps::ICONST_M1 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_M1, -1);
                    return 1;
                }ByteOps::ICONST_0 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_0, 0);
                    return 1;
                }ByteOps::ICONST_1 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_1, 1);
                    return 1;
                }ByteOps::ICONST_2 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_2, 2);
                    return 1;
                }ByteOps::ICONST_3 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_3, 3);
                    return 1;
                }ByteOps::ICONST_4 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_4, 4);
                    return 1;
                }ByteOps::ICONST_5 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstInteger::VALUE_5, 5);
                    return 1;
                }ByteOps::LCONST_0 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstLong::VALUE_0, 0);
                    return 1;
                }ByteOps::LCONST_1 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstLong::VALUE_1, 0);
                    return 1;
                }ByteOps::FCONST_0 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstFloat::VALUE_0, 0);
                    return 1;
                }ByteOps::FCONST_1 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstFloat::VALUE_1, 0);
                    return 1;
                }ByteOps::FCONST_2 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstFloat::VALUE_2, 0);
                    return 1;
                }ByteOps::DCONST_0 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstDouble::VALUE_0, 0);
                    return 1;
                }ByteOps::DCONST_1 =>                 {
                    visitor.visitConstant(ByteOps::LDC, offset, 1, &CstDouble::VALUE_1, 0);
                    return 1;
                }ByteOps::BIPUSH =>                 {
                    let value: i32 = self.bytes.getByte(offset+1);
                    visitor.visitConstant(ByteOps::LDC, offset, 2, CstInteger::make(value), value);
                    return 2;
                }ByteOps::SIPUSH =>                 {
                    let value: i32 = self.bytes.getShort(offset+1);
                    visitor.visitConstant(ByteOps::LDC, offset, 3, CstInteger::make(value), value);
                    return 3;
                }ByteOps::LDC =>                 {
                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                    let cst: Constant = self.pool.get(idx);
                    let value: i32 = if (//cst instanceof CstInteger) { ((CstInteger*)cst).getValue() } else { 0 };
                            visitor.visitConstant(ByteOps::LDC, offset, 2, &cst, value);
                            return 2;
                        }ByteOps::LDC_W =>                         {
                            let idx: i32 = self.bytes.getUnsignedShort(offset+1);
                            let cst: Constant = self.pool.get(idx);
                            let value: i32 = if (//cst instanceof CstInteger) { ((CstInteger*)cst).getValue() } else { 0 };
                                    visitor.visitConstant(ByteOps::LDC, offset, 3, &cst, value);
                                    return 3;
                                }ByteOps::LDC2_W =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedShort(offset+1);
                                    let cst: Constant = self.pool.get(idx);
                                    visitor.visitConstant(ByteOps::LDC2_W, offset, 3, &cst, 0);
                                    return 3;
                                }ByteOps::ILOAD =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 2, idx, &Type::INT, 0);
                                    return 2;
                                }ByteOps::LLOAD =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 2, idx, &Type::LONG, 0);
                                    return 2;
                                }ByteOps::FLOAD =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 2, idx, &Type::FLOAT, 0);
                                    return 2;
                                }ByteOps::DLOAD =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 2, idx, &Type::DOUBLE, 0);
                                    return 2;
                                }ByteOps::ALOAD =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 2, idx, &Type::OBJECT, 0);
                                    return 2;
                                }ByteOps::ILOAD_0 => ByteOps::ILOAD_1 => ByteOps::ILOAD_2 => ByteOps::ILOAD_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::ILOAD_0;
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 1, idx, &Type::INT, 0);
                                    return 1;
                                }ByteOps::LLOAD_0 => ByteOps::LLOAD_1 => ByteOps::LLOAD_2 => ByteOps::LLOAD_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::LLOAD_0;
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 1, idx, &Type::LONG, 0);
                                    return 1;
                                }ByteOps::FLOAD_0 => ByteOps::FLOAD_1 => ByteOps::FLOAD_2 => ByteOps::FLOAD_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::FLOAD_0;
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 1, idx, &Type::FLOAT, 0);
                                    return 1;
                                }ByteOps::DLOAD_0 => ByteOps::DLOAD_1 => ByteOps::DLOAD_2 => ByteOps::DLOAD_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::DLOAD_0;
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 1, idx, &Type::DOUBLE, 0);
                                    return 1;
                                }ByteOps::ALOAD_0 => ByteOps::ALOAD_1 => ByteOps::ALOAD_2 => ByteOps::ALOAD_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::ALOAD_0;
                                    visitor.visitLocal(ByteOps::ILOAD, offset, 1, idx, &Type::OBJECT, 0);
                                    return 1;
                                }ByteOps::IALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::INT);
                                    return 1;
                                }ByteOps::LALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::LONG);
                                    return 1;
                                }ByteOps::FALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::FLOAT);
                                    return 1;
                                }ByteOps::DALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::DOUBLE);
                                    return 1;
                                }ByteOps::AALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::OBJECT);
                                    return 1;
                                }ByteOps::BALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::BYTE);
                                    return 1;
                                }ByteOps::CALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::CHAR);
                                    return 1;
                                }ByteOps::SALOAD =>                                 {
                                    visitor.visitNoArgs(ByteOps::IALOAD, offset, 1, &Type::SHORT);
                                    return 1;
                                }ByteOps::ISTORE =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 2, idx, &Type::INT, 0);
                                    return 2;
                                }ByteOps::LSTORE =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 2, idx, &Type::LONG, 0);
                                    return 2;
                                }ByteOps::FSTORE =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 2, idx, &Type::FLOAT, 0);
                                    return 2;
                                }ByteOps::DSTORE =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 2, idx, &Type::DOUBLE, 0);
                                    return 2;
                                }ByteOps::ASTORE =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 2, idx, &Type::OBJECT, 0);
                                    return 2;
                                }ByteOps::ISTORE_0 => ByteOps::ISTORE_1 => ByteOps::ISTORE_2 => ByteOps::ISTORE_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::ISTORE_0;
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 1, idx, &Type::INT, 0);
                                    return 1;
                                }ByteOps::LSTORE_0 => ByteOps::LSTORE_1 => ByteOps::LSTORE_2 => ByteOps::LSTORE_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::LSTORE_0;
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 1, idx, &Type::LONG, 0);
                                    return 1;
                                }ByteOps::FSTORE_0 => ByteOps::FSTORE_1 => ByteOps::FSTORE_2 => ByteOps::FSTORE_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::FSTORE_0;
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 1, idx, &Type::FLOAT, 0);
                                    return 1;
                                }ByteOps::DSTORE_0 => ByteOps::DSTORE_1 => ByteOps::DSTORE_2 => ByteOps::DSTORE_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::DSTORE_0;
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 1, idx, &Type::DOUBLE, 0);
                                    return 1;
                                }ByteOps::ASTORE_0 => ByteOps::ASTORE_1 => ByteOps::ASTORE_2 => ByteOps::ASTORE_3 =>                                 {
                                    let idx: i32 = opcode-ByteOps::ASTORE_0;
                                    visitor.visitLocal(ByteOps::ISTORE, offset, 1, idx, &Type::OBJECT, 0);
                                    return 1;
                                }ByteOps::IASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::INT);
                                    return 1;
                                }ByteOps::LASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::LONG);
                                    return 1;
                                }ByteOps::FASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::FLOAT);
                                    return 1;
                                }ByteOps::DASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::DOUBLE);
                                    return 1;
                                }ByteOps::AASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::OBJECT);
                                    return 1;
                                }ByteOps::BASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::BYTE);
                                    return 1;
                                }ByteOps::CASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::CHAR);
                                    return 1;
                                }ByteOps::SASTORE =>                                 {
                                    visitor.visitNoArgs(ByteOps::IASTORE, offset, 1, &Type::SHORT);
                                    return 1;
                                }ByteOps::POP => ByteOps::POP2 => ByteOps::DUP => ByteOps::DUP_X1 => ByteOps::DUP_X2 => ByteOps::DUP2 => ByteOps::DUP2_X1 => ByteOps::DUP2_X2 => ByteOps::SWAP =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::VOID);
                                    return 1;
                                }ByteOps::IADD => ByteOps::ISUB => ByteOps::IMUL => ByteOps::IDIV => ByteOps::IREM => ByteOps::INEG => ByteOps::ISHL => ByteOps::ISHR => ByteOps::IUSHR => ByteOps::IAND => ByteOps::IOR => ByteOps::IXOR =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::INT);
                                    return 1;
                                }ByteOps::LADD => ByteOps::LSUB => ByteOps::LMUL => ByteOps::LDIV => ByteOps::LREM => ByteOps::LNEG => ByteOps::LSHL => ByteOps::LSHR => ByteOps::LUSHR => ByteOps::LAND => ByteOps::LOR => ByteOps::LXOR =>                                 {
                                    visitor.visitNoArgs(opcode-1, offset, 1, &Type::LONG);
                                    return 1;
                                }ByteOps::FADD => ByteOps::FSUB => ByteOps::FMUL => ByteOps::FDIV => ByteOps::FREM => ByteOps::FNEG =>                                 {
                                    visitor.visitNoArgs(opcode-2, offset, 1, &Type::FLOAT);
                                    return 1;
                                }ByteOps::DADD => ByteOps::DSUB => ByteOps::DMUL => ByteOps::DDIV => ByteOps::DREM => ByteOps::DNEG =>                                 {
                                    visitor.visitNoArgs(opcode-3, offset, 1, &Type::DOUBLE);
                                    return 1;
                                }ByteOps::IINC =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    let value: i32 = self.bytes.getByte(offset+2);
                                    visitor.visitLocal(opcode, offset, 3, idx, &Type::INT, value);
                                    return 3;
                                }ByteOps::I2L => ByteOps::F2L => ByteOps::D2L =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::LONG);
                                    return 1;
                                }ByteOps::I2F => ByteOps::L2F => ByteOps::D2F =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::FLOAT);
                                    return 1;
                                }ByteOps::I2D => ByteOps::L2D => ByteOps::F2D =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::DOUBLE);
                                    return 1;
                                }ByteOps::L2I => ByteOps::F2I => ByteOps::D2I => ByteOps::I2B => ByteOps::I2C => ByteOps::I2S => ByteOps::LCMP => ByteOps::FCMPL => ByteOps::FCMPG => ByteOps::DCMPL => ByteOps::DCMPG => ByteOps::ARRAYLENGTH =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::INT);
                                    return 1;
                                }ByteOps::IFEQ => ByteOps::IFNE => ByteOps::IFLT => ByteOps::IFGE => ByteOps::IFGT => ByteOps::IFLE => ByteOps::IF_ICMPEQ => ByteOps::IF_ICMPNE => ByteOps::IF_ICMPLT => ByteOps::IF_ICMPGE => ByteOps::IF_ICMPGT => ByteOps::IF_ICMPLE => ByteOps::IF_ACMPEQ => ByteOps::IF_ACMPNE => ByteOps::GOTO => ByteOps::JSR => ByteOps::IFNULL => ByteOps::IFNONNULL =>                                 {
                                    let target: i32 = offset+self.bytes.getShort(offset+1);
                                    visitor.visitBranch(opcode, offset, 3, target);
                                    return 3;
                                }ByteOps::RET =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedByte(offset+1);
                                    visitor.visitLocal(opcode, offset, 2, idx, &Type::RETURN_ADDRESS, 0);
                                    return 2;
                                }ByteOps::TABLESWITCH =>                                 {
                                    return parseTableswitch(offset, &visitor);
                                }ByteOps::LOOKUPSWITCH =>                                 {
                                    return parseLookupswitch(offset, &visitor);
                                }ByteOps::IRETURN =>                                 {
                                    visitor.visitNoArgs(ByteOps::IRETURN, offset, 1, &Type::INT);
                                    return 1;
                                }ByteOps::LRETURN =>                                 {
                                    visitor.visitNoArgs(ByteOps::IRETURN, offset, 1, &Type::LONG);
                                    return 1;
                                }ByteOps::FRETURN =>                                 {
                                    visitor.visitNoArgs(ByteOps::IRETURN, offset, 1, &Type::FLOAT);
                                    return 1;
                                }ByteOps::DRETURN =>                                 {
                                    visitor.visitNoArgs(ByteOps::IRETURN, offset, 1, &Type::DOUBLE);
                                    return 1;
                                }ByteOps::ARETURN =>                                 {
                                    visitor.visitNoArgs(ByteOps::IRETURN, offset, 1, &Type::OBJECT);
                                    return 1;
                                }ByteOps::RETURN => ByteOps::ATHROW => ByteOps::MONITORENTER => ByteOps::MONITOREXIT =>                                 {
                                    visitor.visitNoArgs(opcode, offset, 1, &Type::VOID);
                                    return 1;
                                }ByteOps::GETSTATIC => ByteOps::PUTSTATIC => ByteOps::GETFIELD => ByteOps::PUTFIELD => ByteOps::INVOKEVIRTUAL => ByteOps::INVOKESPECIAL => ByteOps::INVOKESTATIC => ByteOps::NEW => ByteOps::ANEWARRAY => ByteOps::CHECKCAST => ByteOps::INSTANCEOF =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedShort(offset+1);
                                    let cst: Constant = self.pool.get(idx);
                                    visitor.visitConstant(opcode, offset, 3, &cst, 0);
                                    return 3;
                                }ByteOps::INVOKEINTERFACE =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedShort(offset+1);
                                    let count: i32 = self.bytes.getUnsignedByte(offset+3);
                                    let expectZero: i32 = self.bytes.getUnsignedByte(offset+4);
                                    let cst: Constant = self.pool.get(idx);
                                    visitor.visitConstant(opcode, offset, 5, &cst, count|(expectZero<<8));
                                    return 5;
                                }ByteOps::INVOKEDYNAMIC =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedShort(offset+1);
                                    let cstInvokeDynamic: CstInvokeDynamic = (CstInvokeDynamic*)self.pool.get(idx);
                                    visitor.visitConstant(opcode, offset, 5, &cstInvokeDynamic, 0);
                                    return 5;
                                }ByteOps::NEWARRAY =>                                 {
                                    return parseNewarray(offset, &visitor);
                                }ByteOps::WIDE =>                                 {
                                    return parseWide(offset, &visitor);
                                }ByteOps::MULTIANEWARRAY =>                                 {
                                    let idx: i32 = self.bytes.getUnsignedShort(offset+1);
                                    let dimensions: i32 = self.bytes.getUnsignedByte(offset+3);
                                    let cst: Constant = self.pool.get(idx);
                                    visitor.visitConstant(opcode, offset, 4, &cst, dimensions);
                                    return 4;
                                }ByteOps::GOTO_W => ByteOps::JSR_W =>                                 {
                                    let target: i32 = offset+self.bytes.getInt(offset+1);
                                    let newop: i32 = if (opcode==ByteOps::GOTO_W) { ByteOps::GOTO } else { ByteOps::JSR };
                                            visitor.visitBranch(newop, offset, 5, target);
                                            return 5;
                                        }                                    _ => {}                                    {
                                        visitor.visitInvalid(opcode, offset, 1);
                                        return 1;
                                    }                                }
                            }                            catch(                            let ex: SimException)                            {
                                ex.addContext("...at bytecode offset "+Hex::u4(offset));
                                throw ex;
                            }                            catch(                            let ex: RuntimeException)                            {
                                let se: SimException = SimException::new(&ex);
                                se.addContext("...at bytecode offset "+Hex::u4(offset));
                                throw se;
                            }
                        }
                        pub fn parseTableswitch(&self, offset: i32, visitor: &Visitor) -> i32                        {
                            let at: i32 = (offset+4)&~3;
                            let padding: i32 = 0;
                            for(                            let i: i32 = offset+1;;i<ati += 1)                            {
                                padding=(padding<<8)|self.bytes.getUnsignedByte(i);
                            }
                            let defaultTarget: i32 = offset+self.bytes.getInt(at);
                            let low: i32 = self.bytes.getInt(at+4);
                            let high: i32 = self.bytes.getInt(at+8);
                            let count: i32 = high-low+1;
                            at+=12;
                            if low>high                            {
                                throw SimException::new("low / high inversion");
                            }                            
                            let cases: SwitchList = SwitchList::new(count);
                            for(                            let i: i32 = 0;;i<counti += 1)                            {
                                let target: i32 = offset+self.bytes.getInt(at);
                                at+=4;
                                cases.add(low+i, target);
                            }
                            cases.setDefaultTarget(defaultTarget);
                            cases.removeSuperfluousDefaults();
                            cases.setImmutable();
                            let length: i32 = at-offset;
                            visitor.visitSwitch(ByteOps::LOOKUPSWITCH, offset, length, &cases, padding);
                            return length;
                        }
                        pub fn parseLookupswitch(&self, offset: i32, visitor: &Visitor) -> i32                        {
                            let at: i32 = (offset+4)&~3;
                            let padding: i32 = 0;
                            for(                            let i: i32 = offset+1;;i<ati += 1)                            {
                                padding=(padding<<8)|self.bytes.getUnsignedByte(i);
                            }
                            let defaultTarget: i32 = offset+self.bytes.getInt(at);
                            let npairs: i32 = self.bytes.getInt(at+4);
                            at+=8;
                            let cases: SwitchList = SwitchList::new(npairs);
                            for(                            let i: i32 = 0;;i<npairsi += 1)                            {
                                let match: i32 = self.bytes.getInt(at);
                                let target: i32 = offset+self.bytes.getInt(at+4);
                                at+=8;
                                cases.add(match_renamed, target);
                            }
                            cases.setDefaultTarget(defaultTarget);
                            cases.removeSuperfluousDefaults();
                            cases.setImmutable();
                            let length: i32 = at-offset;
                            visitor.visitSwitch(ByteOps::LOOKUPSWITCH, offset, length, &cases, padding);
                            return length;
                        }
                        pub fn parseNewarray(&self, offset: i32, visitor: &Visitor) -> i32                        {
                            let value: i32 = self.bytes.getUnsignedByte(offset+1);
                            let type: CstType;
                            match value{ByteOps::NEWARRAY_BOOLEAN =>                                 {
                                    type_renamed=CstType::BOOLEAN_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_CHAR =>                                 {
                                    type_renamed=CstType::CHAR_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_DOUBLE =>                                 {
                                    type_renamed=CstType::DOUBLE_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_FLOAT =>                                 {
                                    type_renamed=CstType::FLOAT_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_BYTE =>                                 {
                                    type_renamed=CstType::BYTE_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_SHORT =>                                 {
                                    type_renamed=CstType::SHORT_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_INT =>                                 {
                                    type_renamed=CstType::INT_ARRAY;
                                    break;
                                }ByteOps::NEWARRAY_LONG =>                                 {
                                    type_renamed=CstType::LONG_ARRAY;
                                    break;
                                }                            _ => {}                            {
                                throw SimException::new("bad newarray code "+Hex::u1(value));
                            }                        }
                        let previousOffset: i32 = visitor.getPreviousOffset();
                        let constantVisitor: ConstantParserVisitor = ConstantParserVisitor::new(, self);
                        let arrayLength: i32 = 0;
                        if previousOffset>=0                        {
                            parseInstruction(previousOffset, &constantVisitor);
                            if //constantVisitor.cst instanceof CstInteger&&+previousOffset==offset                            {
                                arrayLength=;
                            }                            
                        }                        
                        let nInit: i32 = 0;
                        let curOffset: i32 = offset+2;
                        let lastOffset: i32 = curOffset;
                        let initVals: ArrayList<Constant> = ArrayList<Constant>::new();
                        if arrayLength!=0                        {
                            while true                            {
                                let punt: boolean = false;
                                let nextByte: i32 = self.bytes.getUnsignedByte(curOffset += 1);
                                if nextByte!=ByteOps::DUP{                                    break;                                }                                
                                parseInstruction(curOffset, &constantVisitor);
                                if ==0||!(//constantVisitor.cst instanceof CstInteger)||!=nInit{                                    break;                                }                                
                                curOffset+=;
                                parseInstruction(curOffset, &constantVisitor);
                                if ==0||!(//constantVisitor.cst instanceof CstLiteralBits){                                    break;                                }                                
                                curOffset+=;
                                initVals.add_Constant(&);
                                nextByte=self.bytes.getUnsignedByte(curOffset += 1);
                                match value{ByteOps::NEWARRAY_BYTE => ByteOps::NEWARRAY_BOOLEAN =>                                     {
                                        if nextByte!=ByteOps::BASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }ByteOps::NEWARRAY_CHAR =>                                     {
                                        if nextByte!=ByteOps::CASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }ByteOps::NEWARRAY_DOUBLE =>                                     {
                                        if nextByte!=ByteOps::DASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }ByteOps::NEWARRAY_FLOAT =>                                     {
                                        if nextByte!=ByteOps::FASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }ByteOps::NEWARRAY_SHORT =>                                     {
                                        if nextByte!=ByteOps::SASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }ByteOps::NEWARRAY_INT =>                                     {
                                        if nextByte!=ByteOps::IASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }ByteOps::NEWARRAY_LONG =>                                     {
                                        if nextByte!=ByteOps::LASTORE                                        {
                                            punt=true;
                                        }                                        
                                        break;
                                    }                                _ => {}                                punt=true;                                break;                            }
                            if punt                            {
                                break;
                            }                            
                            lastOffset=curOffset;
                            nInit += 1;
                        }
                    }                    
                    if nInit<2||nInit!=arrayLength                    {
                        visitor.visitNewarray(offset, 2, &type_renamed, None);
                        return 2;
                    }                    else                     {
                        visitor.visitNewarray(offset, lastOffset-offset, &type_renamed, &initVals);
                        return lastOffset-offset;
                    }
                }
                pub fn parseWide(&self, offset: i32, visitor: &Visitor) -> i32                {
                    let opcode: i32 = self.bytes.getUnsignedByte(offset+1);
                    let idx: i32 = self.bytes.getUnsignedShort(offset+2);
                    match opcode{ByteOps::ILOAD =>                         {
                            visitor.visitLocal(ByteOps::ILOAD, offset, 4, idx, &Type::INT, 0);
                            return 4;
                        }ByteOps::LLOAD =>                         {
                            visitor.visitLocal(ByteOps::ILOAD, offset, 4, idx, &Type::LONG, 0);
                            return 4;
                        }ByteOps::FLOAD =>                         {
                            visitor.visitLocal(ByteOps::ILOAD, offset, 4, idx, &Type::FLOAT, 0);
                            return 4;
                        }ByteOps::DLOAD =>                         {
                            visitor.visitLocal(ByteOps::ILOAD, offset, 4, idx, &Type::DOUBLE, 0);
                            return 4;
                        }ByteOps::ALOAD =>                         {
                            visitor.visitLocal(ByteOps::ILOAD, offset, 4, idx, &Type::OBJECT, 0);
                            return 4;
                        }ByteOps::ISTORE =>                         {
                            visitor.visitLocal(ByteOps::ISTORE, offset, 4, idx, &Type::INT, 0);
                            return 4;
                        }ByteOps::LSTORE =>                         {
                            visitor.visitLocal(ByteOps::ISTORE, offset, 4, idx, &Type::LONG, 0);
                            return 4;
                        }ByteOps::FSTORE =>                         {
                            visitor.visitLocal(ByteOps::ISTORE, offset, 4, idx, &Type::FLOAT, 0);
                            return 4;
                        }ByteOps::DSTORE =>                         {
                            visitor.visitLocal(ByteOps::ISTORE, offset, 4, idx, &Type::DOUBLE, 0);
                            return 4;
                        }ByteOps::ASTORE =>                         {
                            visitor.visitLocal(ByteOps::ISTORE, offset, 4, idx, &Type::OBJECT, 0);
                            return 4;
                        }ByteOps::RET =>                         {
                            visitor.visitLocal(opcode, offset, 4, idx, &Type::RETURN_ADDRESS, 0);
                            return 4;
                        }ByteOps::IINC =>                         {
                            let value: i32 = self.bytes.getShort(offset+4);
                            visitor.visitLocal(opcode, offset, 6, idx, &Type::INT, value);
                            return 6;
                        }                    _ => {}                    {
                        visitor.visitInvalid(ByteOps::WIDE, offset, 1);
                        return 1;
                    }                }
            }
}
