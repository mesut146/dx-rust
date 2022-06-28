use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::util::Hex;

struct RegOps{
}
impl RegOps{
    pub const NOP: i32 = 1;
    pub const MOVE: i32 = 2;
    pub const MOVE_PARAM: i32 = 3;
    pub const MOVE_EXCEPTION: i32 = 4;
    pub const CONST: i32 = 5;
    pub const GOTO: i32 = 6;
    pub const IF_EQ: i32 = 7;
    pub const IF_NE: i32 = 8;
    pub const IF_LT: i32 = 9;
    pub const IF_GE: i32 = 10;
    pub const IF_LE: i32 = 11;
    pub const IF_GT: i32 = 12;
    pub const SWITCH: i32 = 13;
    pub const ADD: i32 = 14;
    pub const SUB: i32 = 15;
    pub const MUL: i32 = 16;
    pub const DIV: i32 = 17;
    pub const REM: i32 = 18;
    pub const NEG: i32 = 19;
    pub const AND: i32 = 20;
    pub const OR: i32 = 21;
    pub const XOR: i32 = 22;
    pub const SHL: i32 = 23;
    pub const SHR: i32 = 24;
    pub const USHR: i32 = 25;
    pub const NOT: i32 = 26;
    pub const CMPL: i32 = 27;
    pub const CMPG: i32 = 28;
    pub const CONV: i32 = 29;
    pub const TO_BYTE: i32 = 30;
    pub const TO_CHAR: i32 = 31;
    pub const TO_SHORT: i32 = 32;
    pub const RETURN: i32 = 33;
    pub const ARRAY_LENGTH: i32 = 34;
    pub const THROW: i32 = 35;
    pub const MONITOR_ENTER: i32 = 36;
    pub const MONITOR_EXIT: i32 = 37;
    pub const AGET: i32 = 38;
    pub const APUT: i32 = 39;
    pub const NEW_INSTANCE: i32 = 40;
    pub const NEW_ARRAY: i32 = 41;
    pub const FILLED_NEW_ARRAY: i32 = 42;
    pub const CHECK_CAST: i32 = 43;
    pub const INSTANCE_OF: i32 = 44;
    pub const GET_FIELD: i32 = 45;
    pub const GET_STATIC: i32 = 46;
    pub const PUT_FIELD: i32 = 47;
    pub const PUT_STATIC: i32 = 48;
    pub const INVOKE_STATIC: i32 = 49;
    pub const INVOKE_VIRTUAL: i32 = 50;
    pub const INVOKE_SUPER: i32 = 51;
    pub const INVOKE_DIRECT: i32 = 52;
    pub const INVOKE_INTERFACE: i32 = 53;
    pub const MARK_LOCAL: i32 = 54;
    pub const MOVE_RESULT: i32 = 55;
    pub const MOVE_RESULT_PSEUDO: i32 = 56;
    pub const FILL_ARRAY_DATA: i32 = 57;
    pub const INVOKE_POLYMORPHIC: i32 = 58;
    pub const INVOKE_CUSTOM: i32 = 59;
    pub fn new(&self)    {
    }
    pub fn opName(opcode: i32) -> String    {
        match opcode{RegOps::NOP =>             return "nop";RegOps::MOVE =>             return "move";RegOps::MOVE_PARAM =>             return "move-param";RegOps::MOVE_EXCEPTION =>             return "move-exception";RegOps::CONST =>             return "const";RegOps::GOTO =>             return "goto";RegOps::IF_EQ =>             return "if-eq";RegOps::IF_NE =>             return "if-ne";RegOps::IF_LT =>             return "if-lt";RegOps::IF_GE =>             return "if-ge";RegOps::IF_LE =>             return "if-le";RegOps::IF_GT =>             return "if-gt";RegOps::SWITCH =>             return "switch";RegOps::ADD =>             return "add";RegOps::SUB =>             return "sub";RegOps::MUL =>             return "mul";RegOps::DIV =>             return "div";RegOps::REM =>             return "rem";RegOps::NEG =>             return "neg";RegOps::AND =>             return "and";RegOps::OR =>             return "or";RegOps::XOR =>             return "xor";RegOps::SHL =>             return "shl";RegOps::SHR =>             return "shr";RegOps::USHR =>             return "ushr";RegOps::NOT =>             return "not";RegOps::CMPL =>             return "cmpl";RegOps::CMPG =>             return "cmpg";RegOps::CONV =>             return "conv";RegOps::TO_BYTE =>             return "to-byte";RegOps::TO_CHAR =>             return "to-char";RegOps::TO_SHORT =>             return "to-short";RegOps::RETURN =>             return "return";RegOps::ARRAY_LENGTH =>             return "array-length";RegOps::THROW =>             return "throw";RegOps::MONITOR_ENTER =>             return "monitor-enter";RegOps::MONITOR_EXIT =>             return "monitor-exit";RegOps::AGET =>             return "aget";RegOps::APUT =>             return "aput";RegOps::NEW_INSTANCE =>             return "new-instance";RegOps::NEW_ARRAY =>             return "new-array";RegOps::FILLED_NEW_ARRAY =>             return "filled-new-array";RegOps::CHECK_CAST =>             return "check-cast";RegOps::INSTANCE_OF =>             return "instance-of";RegOps::GET_FIELD =>             return "get-field";RegOps::GET_STATIC =>             return "get-static";RegOps::PUT_FIELD =>             return "put-field";RegOps::PUT_STATIC =>             return "put-static";RegOps::INVOKE_STATIC =>             return "invoke-static";RegOps::INVOKE_VIRTUAL =>             return "invoke-virtual";RegOps::INVOKE_SUPER =>             return "invoke-super";RegOps::INVOKE_DIRECT =>             return "invoke-direct";RegOps::INVOKE_INTERFACE =>             return "invoke-interface";RegOps::MOVE_RESULT =>             return "move-result";RegOps::MOVE_RESULT_PSEUDO =>             return "move-result-pseudo";RegOps::FILL_ARRAY_DATA =>             return "fill-array-data";RegOps::INVOKE_POLYMORPHIC =>             return "invoke-polymorphic";RegOps::INVOKE_CUSTOM =>             return "invoke-custom";        }
        return "unknown-"+Hex::u1(opcode);
    }
    pub fn flippedIfOpcode(opcode: i32) -> i32    {
        match opcode{RegOps::IF_EQ => RegOps::IF_NE =>             return opcode;RegOps::IF_LT =>             return RegOps::IF_GT;RegOps::IF_GE =>             return RegOps::IF_LE;RegOps::IF_LE =>             return RegOps::IF_GE;RegOps::IF_GT =>             return RegOps::IF_LT;        _ => {}        throw RuntimeException::new("Unrecognized IF regop: "+opcode);    }
}
}
