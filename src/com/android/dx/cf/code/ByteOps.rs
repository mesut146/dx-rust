use crate::helper;
use crate::com::android::dx::util::Hex;

let static OPCODE_INFO: Vec<i32> = new int[256];
let static OPCODE_NAMES: Vec<String> = new String[256];
let static OPCODE_DETAILS: String = "00 - nop;"+"01 - aconst_null;"+"02 - iconst_m1;"+"03 - iconst_0;"+"04 - iconst_1;"+"05 - iconst_2;"+"06 - iconst_3;"+"07 - iconst_4;"+"08 - iconst_5;"+"09 - lconst_0;"+"0a - lconst_1;"+"0b - fconst_0;"+"0c - fconst_1;"+"0d - fconst_2;"+"0e - dconst_0;"+"0f - dconst_1;"+"10 y bipush;"+"11 S sipush;"+"12 L:IFcs ldc;"+"13 p:IFcs ldc_w;"+"14 p:DJ ldc2_w;"+"15 l iload;"+"16 m lload;"+"17 l fload;"+"18 m dload;"+"19 l aload;"+"1a 0 iload_0;"+"1b 1 iload_1;"+"1c 2 iload_2;"+"1d 3 iload_3;"+"1e 1 lload_0;"+"1f 2 lload_1;"+"20 3 lload_2;"+"21 4 lload_3;"+"22 0 fload_0;"+"23 1 fload_1;"+"24 2 fload_2;"+"25 3 fload_3;"+"26 1 dload_0;"+"27 2 dload_1;"+"28 3 dload_2;"+"29 4 dload_3;"+"2a 0 aload_0;"+"2b 1 aload_1;"+"2c 2 aload_2;"+"2d 3 aload_3;"+"2e - iaload;"+"2f - laload;"+"30 - faload;"+"31 - daload;"+"32 - aaload;"+"33 - baload;"+"34 - caload;"+"35 - saload;"+"36 - istore;"+"37 - lstore;"+"38 - fstore;"+"39 - dstore;"+"3a - astore;"+"3b 0 istore_0;"+"3c 1 istore_1;"+"3d 2 istore_2;"+"3e 3 istore_3;"+"3f 1 lstore_0;"+"40 2 lstore_1;"+"41 3 lstore_2;"+"42 4 lstore_3;"+"43 0 fstore_0;"+"44 1 fstore_1;"+"45 2 fstore_2;"+"46 3 fstore_3;"+"47 1 dstore_0;"+"48 2 dstore_1;"+"49 3 dstore_2;"+"4a 4 dstore_3;"+"4b 0 astore_0;"+"4c 1 astore_1;"+"4d 2 astore_2;"+"4e 3 astore_3;"+"4f - iastore;"+"50 - lastore;"+"51 - fastore;"+"52 - dastore;"+"53 - aastore;"+"54 - bastore;"+"55 - castore;"+"56 - sastore;"+"57 - pop;"+"58 - pop2;"+"59 - dup;"+"5a - dup_x1;"+"5b - dup_x2;"+"5c - dup2;"+"5d - dup2_x1;"+"5e - dup2_x2;"+"5f - swap;"+"60 - iadd;"+"61 - ladd;"+"62 - fadd;"+"63 - dadd;"+"64 - isub;"+"65 - lsub;"+"66 - fsub;"+"67 - dsub;"+"68 - imul;"+"69 - lmul;"+"6a - fmul;"+"6b - dmul;"+"6c - idiv;"+"6d - ldiv;"+"6e - fdiv;"+"6f - ddiv;"+"70 - irem;"+"71 - lrem;"+"72 - frem;"+"73 - drem;"+"74 - ineg;"+"75 - lneg;"+"76 - fneg;"+"77 - dneg;"+"78 - ishl;"+"79 - lshl;"+"7a - ishr;"+"7b - lshr;"+"7c - iushr;"+"7d - lushr;"+"7e - iand;"+"7f - land;"+"80 - ior;"+"81 - lor;"+"82 - ixor;"+"83 - lxor;"+"84 l iinc;"+"85 - i2l;"+"86 - i2f;"+"87 - i2d;"+"88 - l2i;"+"89 - l2f;"+"8a - l2d;"+"8b - f2i;"+"8c - f2l;"+"8d - f2d;"+"8e - d2i;"+"8f - d2l;"+"90 - d2f;"+"91 - i2b;"+"92 - i2c;"+"93 - i2s;"+"94 - lcmp;"+"95 - fcmpl;"+"96 - fcmpg;"+"97 - dcmpl;"+"98 - dcmpg;"+"99 b ifeq;"+"9a b ifne;"+"9b b iflt;"+"9c b ifge;"+"9d b ifgt;"+"9e b ifle;"+"9f b if_icmpeq;"+"a0 b if_icmpne;"+"a1 b if_icmplt;"+"a2 b if_icmpge;"+"a3 b if_icmpgt;"+"a4 b if_icmple;"+"a5 b if_acmpeq;"+"a6 b if_acmpne;"+"a7 b goto;"+"a8 b jsr;"+"a9 l ret;"+"aa T tableswitch;"+"ab U lookupswitch;"+"ac - ireturn;"+"ad - lreturn;"+"ae - freturn;"+"af - dreturn;"+"b0 - areturn;"+"b1 - return;"+"b2 p:f getstatic;"+"b3 p:f putstatic;"+"b4 p:f getfield;"+"b5 p:f putfield;"+"b6 p:m invokevirtual;"+"b7 p:m invokespecial;"+"b8 p:m invokestatic;"+"b9 I:i invokeinterface;"+"bb p:c new;"+"bc y newarray;"+"bd p:c anewarray;"+"be - arraylength;"+"bf - athrow;"+"c0 p:c checkcast;"+"c1 p:c instanceof;"+"c2 - monitorenter;"+"c3 - monitorexit;"+"c4 W wide;"+"c5 M:c multianewarray;"+"c6 b ifnull;"+"c7 b ifnonnull;"+"c8 c goto_w;"+"c9 c jsr_w;";
struct ByteOps{
}
impl ByteOps{
    pub const NOP: i32 = 0x00;
    pub const ACONST_NULL: i32 = 0x01;
    pub const ICONST_M1: i32 = 0x02;
    pub const ICONST_0: i32 = 0x03;
    pub const ICONST_1: i32 = 0x04;
    pub const ICONST_2: i32 = 0x05;
    pub const ICONST_3: i32 = 0x06;
    pub const ICONST_4: i32 = 0x07;
    pub const ICONST_5: i32 = 0x08;
    pub const LCONST_0: i32 = 0x09;
    pub const LCONST_1: i32 = 0x0a;
    pub const FCONST_0: i32 = 0x0b;
    pub const FCONST_1: i32 = 0x0c;
    pub const FCONST_2: i32 = 0x0d;
    pub const DCONST_0: i32 = 0x0e;
    pub const DCONST_1: i32 = 0x0f;
    pub const BIPUSH: i32 = 0x10;
    pub const SIPUSH: i32 = 0x11;
    pub const LDC: i32 = 0x12;
    pub const LDC_W: i32 = 0x13;
    pub const LDC2_W: i32 = 0x14;
    pub const ILOAD: i32 = 0x15;
    pub const LLOAD: i32 = 0x16;
    pub const FLOAD: i32 = 0x17;
    pub const DLOAD: i32 = 0x18;
    pub const ALOAD: i32 = 0x19;
    pub const ILOAD_0: i32 = 0x1a;
    pub const ILOAD_1: i32 = 0x1b;
    pub const ILOAD_2: i32 = 0x1c;
    pub const ILOAD_3: i32 = 0x1d;
    pub const LLOAD_0: i32 = 0x1e;
    pub const LLOAD_1: i32 = 0x1f;
    pub const LLOAD_2: i32 = 0x20;
    pub const LLOAD_3: i32 = 0x21;
    pub const FLOAD_0: i32 = 0x22;
    pub const FLOAD_1: i32 = 0x23;
    pub const FLOAD_2: i32 = 0x24;
    pub const FLOAD_3: i32 = 0x25;
    pub const DLOAD_0: i32 = 0x26;
    pub const DLOAD_1: i32 = 0x27;
    pub const DLOAD_2: i32 = 0x28;
    pub const DLOAD_3: i32 = 0x29;
    pub const ALOAD_0: i32 = 0x2a;
    pub const ALOAD_1: i32 = 0x2b;
    pub const ALOAD_2: i32 = 0x2c;
    pub const ALOAD_3: i32 = 0x2d;
    pub const IALOAD: i32 = 0x2e;
    pub const LALOAD: i32 = 0x2f;
    pub const FALOAD: i32 = 0x30;
    pub const DALOAD: i32 = 0x31;
    pub const AALOAD: i32 = 0x32;
    pub const BALOAD: i32 = 0x33;
    pub const CALOAD: i32 = 0x34;
    pub const SALOAD: i32 = 0x35;
    pub const ISTORE: i32 = 0x36;
    pub const LSTORE: i32 = 0x37;
    pub const FSTORE: i32 = 0x38;
    pub const DSTORE: i32 = 0x39;
    pub const ASTORE: i32 = 0x3a;
    pub const ISTORE_0: i32 = 0x3b;
    pub const ISTORE_1: i32 = 0x3c;
    pub const ISTORE_2: i32 = 0x3d;
    pub const ISTORE_3: i32 = 0x3e;
    pub const LSTORE_0: i32 = 0x3f;
    pub const LSTORE_1: i32 = 0x40;
    pub const LSTORE_2: i32 = 0x41;
    pub const LSTORE_3: i32 = 0x42;
    pub const FSTORE_0: i32 = 0x43;
    pub const FSTORE_1: i32 = 0x44;
    pub const FSTORE_2: i32 = 0x45;
    pub const FSTORE_3: i32 = 0x46;
    pub const DSTORE_0: i32 = 0x47;
    pub const DSTORE_1: i32 = 0x48;
    pub const DSTORE_2: i32 = 0x49;
    pub const DSTORE_3: i32 = 0x4a;
    pub const ASTORE_0: i32 = 0x4b;
    pub const ASTORE_1: i32 = 0x4c;
    pub const ASTORE_2: i32 = 0x4d;
    pub const ASTORE_3: i32 = 0x4e;
    pub const IASTORE: i32 = 0x4f;
    pub const LASTORE: i32 = 0x50;
    pub const FASTORE: i32 = 0x51;
    pub const DASTORE: i32 = 0x52;
    pub const AASTORE: i32 = 0x53;
    pub const BASTORE: i32 = 0x54;
    pub const CASTORE: i32 = 0x55;
    pub const SASTORE: i32 = 0x56;
    pub const POP: i32 = 0x57;
    pub const POP2: i32 = 0x58;
    pub const DUP: i32 = 0x59;
    pub const DUP_X1: i32 = 0x5a;
    pub const DUP_X2: i32 = 0x5b;
    pub const DUP2: i32 = 0x5c;
    pub const DUP2_X1: i32 = 0x5d;
    pub const DUP2_X2: i32 = 0x5e;
    pub const SWAP: i32 = 0x5f;
    pub const IADD: i32 = 0x60;
    pub const LADD: i32 = 0x61;
    pub const FADD: i32 = 0x62;
    pub const DADD: i32 = 0x63;
    pub const ISUB: i32 = 0x64;
    pub const LSUB: i32 = 0x65;
    pub const FSUB: i32 = 0x66;
    pub const DSUB: i32 = 0x67;
    pub const IMUL: i32 = 0x68;
    pub const LMUL: i32 = 0x69;
    pub const FMUL: i32 = 0x6a;
    pub const DMUL: i32 = 0x6b;
    pub const IDIV: i32 = 0x6c;
    pub const LDIV: i32 = 0x6d;
    pub const FDIV: i32 = 0x6e;
    pub const DDIV: i32 = 0x6f;
    pub const IREM: i32 = 0x70;
    pub const LREM: i32 = 0x71;
    pub const FREM: i32 = 0x72;
    pub const DREM: i32 = 0x73;
    pub const INEG: i32 = 0x74;
    pub const LNEG: i32 = 0x75;
    pub const FNEG: i32 = 0x76;
    pub const DNEG: i32 = 0x77;
    pub const ISHL: i32 = 0x78;
    pub const LSHL: i32 = 0x79;
    pub const ISHR: i32 = 0x7a;
    pub const LSHR: i32 = 0x7b;
    pub const IUSHR: i32 = 0x7c;
    pub const LUSHR: i32 = 0x7d;
    pub const IAND: i32 = 0x7e;
    pub const LAND: i32 = 0x7f;
    pub const IOR: i32 = 0x80;
    pub const LOR: i32 = 0x81;
    pub const IXOR: i32 = 0x82;
    pub const LXOR: i32 = 0x83;
    pub const IINC: i32 = 0x84;
    pub const I2L: i32 = 0x85;
    pub const I2F: i32 = 0x86;
    pub const I2D: i32 = 0x87;
    pub const L2I: i32 = 0x88;
    pub const L2F: i32 = 0x89;
    pub const L2D: i32 = 0x8a;
    pub const F2I: i32 = 0x8b;
    pub const F2L: i32 = 0x8c;
    pub const F2D: i32 = 0x8d;
    pub const D2I: i32 = 0x8e;
    pub const D2L: i32 = 0x8f;
    pub const D2F: i32 = 0x90;
    pub const I2B: i32 = 0x91;
    pub const I2C: i32 = 0x92;
    pub const I2S: i32 = 0x93;
    pub const LCMP: i32 = 0x94;
    pub const FCMPL: i32 = 0x95;
    pub const FCMPG: i32 = 0x96;
    pub const DCMPL: i32 = 0x97;
    pub const DCMPG: i32 = 0x98;
    pub const IFEQ: i32 = 0x99;
    pub const IFNE: i32 = 0x9a;
    pub const IFLT: i32 = 0x9b;
    pub const IFGE: i32 = 0x9c;
    pub const IFGT: i32 = 0x9d;
    pub const IFLE: i32 = 0x9e;
    pub const IF_ICMPEQ: i32 = 0x9f;
    pub const IF_ICMPNE: i32 = 0xa0;
    pub const IF_ICMPLT: i32 = 0xa1;
    pub const IF_ICMPGE: i32 = 0xa2;
    pub const IF_ICMPGT: i32 = 0xa3;
    pub const IF_ICMPLE: i32 = 0xa4;
    pub const IF_ACMPEQ: i32 = 0xa5;
    pub const IF_ACMPNE: i32 = 0xa6;
    pub const GOTO: i32 = 0xa7;
    pub const JSR: i32 = 0xa8;
    pub const RET: i32 = 0xa9;
    pub const TABLESWITCH: i32 = 0xaa;
    pub const LOOKUPSWITCH: i32 = 0xab;
    pub const IRETURN: i32 = 0xac;
    pub const LRETURN: i32 = 0xad;
    pub const FRETURN: i32 = 0xae;
    pub const DRETURN: i32 = 0xaf;
    pub const ARETURN: i32 = 0xb0;
    pub const RETURN: i32 = 0xb1;
    pub const GETSTATIC: i32 = 0xb2;
    pub const PUTSTATIC: i32 = 0xb3;
    pub const GETFIELD: i32 = 0xb4;
    pub const PUTFIELD: i32 = 0xb5;
    pub const INVOKEVIRTUAL: i32 = 0xb6;
    pub const INVOKESPECIAL: i32 = 0xb7;
    pub const INVOKESTATIC: i32 = 0xb8;
    pub const INVOKEINTERFACE: i32 = 0xb9;
    pub const INVOKEDYNAMIC: i32 = 0xba;
    pub const NEW: i32 = 0xbb;
    pub const NEWARRAY: i32 = 0xbc;
    pub const ANEWARRAY: i32 = 0xbd;
    pub const ARRAYLENGTH: i32 = 0xbe;
    pub const ATHROW: i32 = 0xbf;
    pub const CHECKCAST: i32 = 0xc0;
    pub const INSTANCEOF: i32 = 0xc1;
    pub const MONITORENTER: i32 = 0xc2;
    pub const MONITOREXIT: i32 = 0xc3;
    pub const WIDE: i32 = 0xc4;
    pub const MULTIANEWARRAY: i32 = 0xc5;
    pub const IFNULL: i32 = 0xc6;
    pub const IFNONNULL: i32 = 0xc7;
    pub const GOTO_W: i32 = 0xc8;
    pub const JSR_W: i32 = 0xc9;
    pub const NEWARRAY_BOOLEAN: i32 = 4;
    pub const NEWARRAY_CHAR: i32 = 5;
    pub const NEWARRAY_FLOAT: i32 = 6;
    pub const NEWARRAY_DOUBLE: i32 = 7;
    pub const NEWARRAY_BYTE: i32 = 8;
    pub const NEWARRAY_SHORT: i32 = 9;
    pub const NEWARRAY_INT: i32 = 10;
    pub const NEWARRAY_LONG: i32 = 11;
    pub const FMT_INVALID: i32 = 0;
    pub const FMT_NO_ARGS: i32 = 1;
    pub const FMT_NO_ARGS_LOCALS_1: i32 = 2;
    pub const FMT_NO_ARGS_LOCALS_2: i32 = 3;
    pub const FMT_NO_ARGS_LOCALS_3: i32 = 4;
    pub const FMT_NO_ARGS_LOCALS_4: i32 = 5;
    pub const FMT_NO_ARGS_LOCALS_5: i32 = 6;
    pub const FMT_BRANCH: i32 = 7;
    pub const FMT_WIDE_BRANCH: i32 = 8;
    pub const FMT_CPI: i32 = 9;
    pub const FMT_LOCAL_1: i32 = 10;
    pub const FMT_LOCAL_2: i32 = 11;
    pub const FMT_LITERAL_BYTE: i32 = 12;
    pub const FMT_INVOKEINTERFACE: i32 = 13;
    pub const FMT_LDC: i32 = 14;
    pub const FMT_SIPUSH: i32 = 15;
    pub const FMT_TABLESWITCH: i32 = 16;
    pub const FMT_LOOKUPSWITCH: i32 = 17;
    pub const FMT_MULTIANEWARRAY: i32 = 18;
    pub const FMT_WIDE: i32 = 19;
    pub const FMT_MASK: i32 = 0x1f;
    pub const CPOK_Integer: i32 = 0x20;
    pub const CPOK_Float: i32 = 0x40;
    pub const CPOK_Long: i32 = 0x80;
    pub const CPOK_Double: i32 = 0x100;
    pub const CPOK_Class: i32 = 0x200;
    pub const CPOK_String: i32 = 0x400;
    pub const CPOK_Fieldref: i32 = 0x800;
    pub const CPOK_Methodref: i32 = 0x1000;
    pub const CPOK_InterfaceMethodref: i32 = 0x2000;
    pub fn new(&self)    {
    }
    pub fn opName(opcode: i32) -> String    {
        let result: String = ByteOps::OPCODE_NAMES[opcode];
        if result==None        {
            result="unused_"+Hex::u1(opcode);
            ByteOps::OPCODE_NAMES[opcode]=result;
        }        
        return result;
    }
    pub fn opInfo(opcode: i32) -> i32    {
        return ByteOps::OPCODE_INFO[opcode];
    }
}
