use crate::helper;
use crate::com::android::dx::dex::code::form::Form11n;
use crate::com::android::dx::io::Opcodes;
use crate::com::android::dx::dex::code::form::Form22c;
use crate::com::android::dx::dex::code::form::Form31i;
use crate::com::android::dx::dex::code::form::Form22b;
use crate::com::android::dx::dex::code::form::Form51l;
use crate::com::android::dx::dex::code::form::Form21c;
use crate::com::android::dx::dex::code::form::Form11x;
use crate::com::android::dx::dex::code::form::Form31t;
use crate::com::android::dx::dex::code::form::Form10t;
use crate::com::android::dx::dex::code::form::Form23x;
use crate::com::android::dx::dex::code::form::Form22s;
use crate::com::android::dx::dex::code::form::Form20t;
use crate::com::android::dx::dex::code::form::Form31c;
use crate::com::android::dx::dex::code::form::Form10x;
use crate::com::android::dx::dex::code::Dop;
use crate::com::android::dx::dex::code::form::Form35c;
use crate::com::android::dx::dex::code::form::Form21s;
use crate::com::android::dx::dex::code::form::Form21h;
use crate::com::android::dx::dex::code::form::SpecialFormat;
use crate::com::android::dx::dex::code::form::Form12x;
use crate::com::android::dx::dex::code::form::Form22x;
use crate::com::android::dx::dex::code::form::Form3rc;
use crate::com::android::dx::dex::code::form::Form30t;
use crate::com::android::dx::dex::code::form::Form22t;
use crate::com::android::dx::dex::code::form::Form21t;
use crate::com::android::dx::dex::code::form::Form4rcc;
use crate::com::android::dx::dex::code::form::Form45cc;
use crate::com::android::dx::dex::code::form::Form32x;

let static DOPS: Option<Vec<Dop>> = Node;
let static SPECIAL_FORMAT: Dop = Dop::new(Opcodes::SPECIAL_FORMAT, Opcodes::SPECIAL_FORMAT, Opcodes::NO_NEXT, &SpecialFormat::THE_ONE, false);
let static NOP: Dop = Dop::new(Opcodes::NOP, Opcodes::NOP, Opcodes::NO_NEXT, &Form10x::THE_ONE, false);
let static MOVE: Dop = Dop::new(Opcodes::MOVE, Opcodes::MOVE, Opcodes::MOVE_FROM16, &Form12x::THE_ONE, true);
let static MOVE_FROM16: Dop = Dop::new(Opcodes::MOVE_FROM16, Opcodes::MOVE, Opcodes::MOVE_16, &Form22x::THE_ONE, true);
let static MOVE_16: Dop = Dop::new(Opcodes::MOVE_16, Opcodes::MOVE, Opcodes::NO_NEXT, &Form32x::THE_ONE, true);
let static MOVE_WIDE: Dop = Dop::new(Opcodes::MOVE_WIDE, Opcodes::MOVE_WIDE, Opcodes::MOVE_WIDE_FROM16, &Form12x::THE_ONE, true);
let static MOVE_WIDE_FROM16: Dop = Dop::new(Opcodes::MOVE_WIDE_FROM16, Opcodes::MOVE_WIDE, Opcodes::MOVE_WIDE_16, &Form22x::THE_ONE, true);
let static MOVE_WIDE_16: Dop = Dop::new(Opcodes::MOVE_WIDE_16, Opcodes::MOVE_WIDE, Opcodes::NO_NEXT, &Form32x::THE_ONE, true);
let static MOVE_OBJECT: Dop = Dop::new(Opcodes::MOVE_OBJECT, Opcodes::MOVE_OBJECT, Opcodes::MOVE_OBJECT_FROM16, &Form12x::THE_ONE, true);
let static MOVE_OBJECT_FROM16: Dop = Dop::new(Opcodes::MOVE_OBJECT_FROM16, Opcodes::MOVE_OBJECT, Opcodes::MOVE_OBJECT_16, &Form22x::THE_ONE, true);
let static MOVE_OBJECT_16: Dop = Dop::new(Opcodes::MOVE_OBJECT_16, Opcodes::MOVE_OBJECT, Opcodes::NO_NEXT, &Form32x::THE_ONE, true);
let static MOVE_RESULT: Dop = Dop::new(Opcodes::MOVE_RESULT, Opcodes::MOVE_RESULT, Opcodes::NO_NEXT, &Form11x::THE_ONE, true);
let static MOVE_RESULT_WIDE: Dop = Dop::new(Opcodes::MOVE_RESULT_WIDE, Opcodes::MOVE_RESULT_WIDE, Opcodes::NO_NEXT, &Form11x::THE_ONE, true);
let static MOVE_RESULT_OBJECT: Dop = Dop::new(Opcodes::MOVE_RESULT_OBJECT, Opcodes::MOVE_RESULT_OBJECT, Opcodes::NO_NEXT, &Form11x::THE_ONE, true);
let static MOVE_EXCEPTION: Dop = Dop::new(Opcodes::MOVE_EXCEPTION, Opcodes::MOVE_EXCEPTION, Opcodes::NO_NEXT, &Form11x::THE_ONE, true);
let static RETURN_VOID: Dop = Dop::new(Opcodes::RETURN_VOID, Opcodes::RETURN_VOID, Opcodes::NO_NEXT, &Form10x::THE_ONE, false);
let static RETURN: Dop = Dop::new(Opcodes::RETURN, Opcodes::RETURN, Opcodes::NO_NEXT, &Form11x::THE_ONE, false);
let static RETURN_WIDE: Dop = Dop::new(Opcodes::RETURN_WIDE, Opcodes::RETURN_WIDE, Opcodes::NO_NEXT, &Form11x::THE_ONE, false);
let static RETURN_OBJECT: Dop = Dop::new(Opcodes::RETURN_OBJECT, Opcodes::RETURN_OBJECT, Opcodes::NO_NEXT, &Form11x::THE_ONE, false);
let static CONST_4: Dop = Dop::new(Opcodes::CONST_4, Opcodes::CONST, Opcodes::CONST_16, &Form11n::THE_ONE, true);
let static CONST_16: Dop = Dop::new(Opcodes::CONST_16, Opcodes::CONST, Opcodes::CONST_HIGH16, &Form21s::THE_ONE, true);
let static CONST: Dop = Dop::new(Opcodes::CONST, Opcodes::CONST, Opcodes::NO_NEXT, &Form31i::THE_ONE, true);
let static CONST_HIGH16: Dop = Dop::new(Opcodes::CONST_HIGH16, Opcodes::CONST, Opcodes::CONST, &Form21h::THE_ONE, true);
let static CONST_WIDE_16: Dop = Dop::new(Opcodes::CONST_WIDE_16, Opcodes::CONST_WIDE, Opcodes::CONST_WIDE_HIGH16, &Form21s::THE_ONE, true);
let static CONST_WIDE_32: Dop = Dop::new(Opcodes::CONST_WIDE_32, Opcodes::CONST_WIDE, Opcodes::CONST_WIDE, &Form31i::THE_ONE, true);
let static CONST_WIDE: Dop = Dop::new(Opcodes::CONST_WIDE, Opcodes::CONST_WIDE, Opcodes::NO_NEXT, &Form51l::THE_ONE, true);
let static CONST_WIDE_HIGH16: Dop = Dop::new(Opcodes::CONST_WIDE_HIGH16, Opcodes::CONST_WIDE, Opcodes::CONST_WIDE_32, &Form21h::THE_ONE, true);
let static CONST_STRING: Dop = Dop::new(Opcodes::CONST_STRING, Opcodes::CONST_STRING, Opcodes::CONST_STRING_JUMBO, &Form21c::THE_ONE, true);
let static CONST_STRING_JUMBO: Dop = Dop::new(Opcodes::CONST_STRING_JUMBO, Opcodes::CONST_STRING, Opcodes::NO_NEXT, &Form31c::THE_ONE, true);
let static CONST_CLASS: Dop = Dop::new(Opcodes::CONST_CLASS, Opcodes::CONST_CLASS, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static MONITOR_ENTER: Dop = Dop::new(Opcodes::MONITOR_ENTER, Opcodes::MONITOR_ENTER, Opcodes::NO_NEXT, &Form11x::THE_ONE, false);
let static MONITOR_EXIT: Dop = Dop::new(Opcodes::MONITOR_EXIT, Opcodes::MONITOR_EXIT, Opcodes::NO_NEXT, &Form11x::THE_ONE, false);
let static CHECK_CAST: Dop = Dop::new(Opcodes::CHECK_CAST, Opcodes::CHECK_CAST, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static INSTANCE_OF: Dop = Dop::new(Opcodes::INSTANCE_OF, Opcodes::INSTANCE_OF, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static ARRAY_LENGTH: Dop = Dop::new(Opcodes::ARRAY_LENGTH, Opcodes::ARRAY_LENGTH, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static NEW_INSTANCE: Dop = Dop::new(Opcodes::NEW_INSTANCE, Opcodes::NEW_INSTANCE, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static NEW_ARRAY: Dop = Dop::new(Opcodes::NEW_ARRAY, Opcodes::NEW_ARRAY, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static FILLED_NEW_ARRAY: Dop = Dop::new(Opcodes::FILLED_NEW_ARRAY, Opcodes::FILLED_NEW_ARRAY, Opcodes::FILLED_NEW_ARRAY_RANGE, &Form35c::THE_ONE, false);
let static FILLED_NEW_ARRAY_RANGE: Dop = Dop::new(Opcodes::FILLED_NEW_ARRAY_RANGE, Opcodes::FILLED_NEW_ARRAY, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static FILL_ARRAY_DATA: Dop = Dop::new(Opcodes::FILL_ARRAY_DATA, Opcodes::FILL_ARRAY_DATA, Opcodes::NO_NEXT, &Form31t::THE_ONE, false);
let static THROW: Dop = Dop::new(Opcodes::THROW, Opcodes::THROW, Opcodes::NO_NEXT, &Form11x::THE_ONE, false);
let static GOTO: Dop = Dop::new(Opcodes::GOTO, Opcodes::GOTO, Opcodes::GOTO_16, &Form10t::THE_ONE, false);
let static GOTO_16: Dop = Dop::new(Opcodes::GOTO_16, Opcodes::GOTO, Opcodes::GOTO_32, &Form20t::THE_ONE, false);
let static GOTO_32: Dop = Dop::new(Opcodes::GOTO_32, Opcodes::GOTO, Opcodes::NO_NEXT, &Form30t::THE_ONE, false);
let static PACKED_SWITCH: Dop = Dop::new(Opcodes::PACKED_SWITCH, Opcodes::PACKED_SWITCH, Opcodes::NO_NEXT, &Form31t::THE_ONE, false);
let static SPARSE_SWITCH: Dop = Dop::new(Opcodes::SPARSE_SWITCH, Opcodes::SPARSE_SWITCH, Opcodes::NO_NEXT, &Form31t::THE_ONE, false);
let static CMPL_FLOAT: Dop = Dop::new(Opcodes::CMPL_FLOAT, Opcodes::CMPL_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static CMPG_FLOAT: Dop = Dop::new(Opcodes::CMPG_FLOAT, Opcodes::CMPG_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static CMPL_DOUBLE: Dop = Dop::new(Opcodes::CMPL_DOUBLE, Opcodes::CMPL_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static CMPG_DOUBLE: Dop = Dop::new(Opcodes::CMPG_DOUBLE, Opcodes::CMPG_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static CMP_LONG: Dop = Dop::new(Opcodes::CMP_LONG, Opcodes::CMP_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static IF_EQ: Dop = Dop::new(Opcodes::IF_EQ, Opcodes::IF_EQ, Opcodes::NO_NEXT, &Form22t::THE_ONE, false);
let static IF_NE: Dop = Dop::new(Opcodes::IF_NE, Opcodes::IF_NE, Opcodes::NO_NEXT, &Form22t::THE_ONE, false);
let static IF_LT: Dop = Dop::new(Opcodes::IF_LT, Opcodes::IF_LT, Opcodes::NO_NEXT, &Form22t::THE_ONE, false);
let static IF_GE: Dop = Dop::new(Opcodes::IF_GE, Opcodes::IF_GE, Opcodes::NO_NEXT, &Form22t::THE_ONE, false);
let static IF_GT: Dop = Dop::new(Opcodes::IF_GT, Opcodes::IF_GT, Opcodes::NO_NEXT, &Form22t::THE_ONE, false);
let static IF_LE: Dop = Dop::new(Opcodes::IF_LE, Opcodes::IF_LE, Opcodes::NO_NEXT, &Form22t::THE_ONE, false);
let static IF_EQZ: Dop = Dop::new(Opcodes::IF_EQZ, Opcodes::IF_EQZ, Opcodes::NO_NEXT, &Form21t::THE_ONE, false);
let static IF_NEZ: Dop = Dop::new(Opcodes::IF_NEZ, Opcodes::IF_NEZ, Opcodes::NO_NEXT, &Form21t::THE_ONE, false);
let static IF_LTZ: Dop = Dop::new(Opcodes::IF_LTZ, Opcodes::IF_LTZ, Opcodes::NO_NEXT, &Form21t::THE_ONE, false);
let static IF_GEZ: Dop = Dop::new(Opcodes::IF_GEZ, Opcodes::IF_GEZ, Opcodes::NO_NEXT, &Form21t::THE_ONE, false);
let static IF_GTZ: Dop = Dop::new(Opcodes::IF_GTZ, Opcodes::IF_GTZ, Opcodes::NO_NEXT, &Form21t::THE_ONE, false);
let static IF_LEZ: Dop = Dop::new(Opcodes::IF_LEZ, Opcodes::IF_LEZ, Opcodes::NO_NEXT, &Form21t::THE_ONE, false);
let static AGET: Dop = Dop::new(Opcodes::AGET, Opcodes::AGET, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AGET_WIDE: Dop = Dop::new(Opcodes::AGET_WIDE, Opcodes::AGET_WIDE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AGET_OBJECT: Dop = Dop::new(Opcodes::AGET_OBJECT, Opcodes::AGET_OBJECT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AGET_BOOLEAN: Dop = Dop::new(Opcodes::AGET_BOOLEAN, Opcodes::AGET_BOOLEAN, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AGET_BYTE: Dop = Dop::new(Opcodes::AGET_BYTE, Opcodes::AGET_BYTE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AGET_CHAR: Dop = Dop::new(Opcodes::AGET_CHAR, Opcodes::AGET_CHAR, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AGET_SHORT: Dop = Dop::new(Opcodes::AGET_SHORT, Opcodes::AGET_SHORT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static APUT: Dop = Dop::new(Opcodes::APUT, Opcodes::APUT, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static APUT_WIDE: Dop = Dop::new(Opcodes::APUT_WIDE, Opcodes::APUT_WIDE, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static APUT_OBJECT: Dop = Dop::new(Opcodes::APUT_OBJECT, Opcodes::APUT_OBJECT, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static APUT_BOOLEAN: Dop = Dop::new(Opcodes::APUT_BOOLEAN, Opcodes::APUT_BOOLEAN, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static APUT_BYTE: Dop = Dop::new(Opcodes::APUT_BYTE, Opcodes::APUT_BYTE, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static APUT_CHAR: Dop = Dop::new(Opcodes::APUT_CHAR, Opcodes::APUT_CHAR, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static APUT_SHORT: Dop = Dop::new(Opcodes::APUT_SHORT, Opcodes::APUT_SHORT, Opcodes::NO_NEXT, &Form23x::THE_ONE, false);
let static IGET: Dop = Dop::new(Opcodes::IGET, Opcodes::IGET, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IGET_WIDE: Dop = Dop::new(Opcodes::IGET_WIDE, Opcodes::IGET_WIDE, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IGET_OBJECT: Dop = Dop::new(Opcodes::IGET_OBJECT, Opcodes::IGET_OBJECT, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IGET_BOOLEAN: Dop = Dop::new(Opcodes::IGET_BOOLEAN, Opcodes::IGET_BOOLEAN, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IGET_BYTE: Dop = Dop::new(Opcodes::IGET_BYTE, Opcodes::IGET_BYTE, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IGET_CHAR: Dop = Dop::new(Opcodes::IGET_CHAR, Opcodes::IGET_CHAR, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IGET_SHORT: Dop = Dop::new(Opcodes::IGET_SHORT, Opcodes::IGET_SHORT, Opcodes::NO_NEXT, &Form22c::THE_ONE, true);
let static IPUT: Dop = Dop::new(Opcodes::IPUT, Opcodes::IPUT, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static IPUT_WIDE: Dop = Dop::new(Opcodes::IPUT_WIDE, Opcodes::IPUT_WIDE, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static IPUT_OBJECT: Dop = Dop::new(Opcodes::IPUT_OBJECT, Opcodes::IPUT_OBJECT, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static IPUT_BOOLEAN: Dop = Dop::new(Opcodes::IPUT_BOOLEAN, Opcodes::IPUT_BOOLEAN, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static IPUT_BYTE: Dop = Dop::new(Opcodes::IPUT_BYTE, Opcodes::IPUT_BYTE, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static IPUT_CHAR: Dop = Dop::new(Opcodes::IPUT_CHAR, Opcodes::IPUT_CHAR, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static IPUT_SHORT: Dop = Dop::new(Opcodes::IPUT_SHORT, Opcodes::IPUT_SHORT, Opcodes::NO_NEXT, &Form22c::THE_ONE, false);
let static SGET: Dop = Dop::new(Opcodes::SGET, Opcodes::SGET, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SGET_WIDE: Dop = Dop::new(Opcodes::SGET_WIDE, Opcodes::SGET_WIDE, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SGET_OBJECT: Dop = Dop::new(Opcodes::SGET_OBJECT, Opcodes::SGET_OBJECT, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SGET_BOOLEAN: Dop = Dop::new(Opcodes::SGET_BOOLEAN, Opcodes::SGET_BOOLEAN, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SGET_BYTE: Dop = Dop::new(Opcodes::SGET_BYTE, Opcodes::SGET_BYTE, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SGET_CHAR: Dop = Dop::new(Opcodes::SGET_CHAR, Opcodes::SGET_CHAR, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SGET_SHORT: Dop = Dop::new(Opcodes::SGET_SHORT, Opcodes::SGET_SHORT, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static SPUT: Dop = Dop::new(Opcodes::SPUT, Opcodes::SPUT, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static SPUT_WIDE: Dop = Dop::new(Opcodes::SPUT_WIDE, Opcodes::SPUT_WIDE, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static SPUT_OBJECT: Dop = Dop::new(Opcodes::SPUT_OBJECT, Opcodes::SPUT_OBJECT, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static SPUT_BOOLEAN: Dop = Dop::new(Opcodes::SPUT_BOOLEAN, Opcodes::SPUT_BOOLEAN, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static SPUT_BYTE: Dop = Dop::new(Opcodes::SPUT_BYTE, Opcodes::SPUT_BYTE, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static SPUT_CHAR: Dop = Dop::new(Opcodes::SPUT_CHAR, Opcodes::SPUT_CHAR, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static SPUT_SHORT: Dop = Dop::new(Opcodes::SPUT_SHORT, Opcodes::SPUT_SHORT, Opcodes::NO_NEXT, &Form21c::THE_ONE, false);
let static INVOKE_VIRTUAL: Dop = Dop::new(Opcodes::INVOKE_VIRTUAL, Opcodes::INVOKE_VIRTUAL, Opcodes::INVOKE_VIRTUAL_RANGE, &Form35c::THE_ONE, false);
let static INVOKE_SUPER: Dop = Dop::new(Opcodes::INVOKE_SUPER, Opcodes::INVOKE_SUPER, Opcodes::INVOKE_SUPER_RANGE, &Form35c::THE_ONE, false);
let static INVOKE_DIRECT: Dop = Dop::new(Opcodes::INVOKE_DIRECT, Opcodes::INVOKE_DIRECT, Opcodes::INVOKE_DIRECT_RANGE, &Form35c::THE_ONE, false);
let static INVOKE_STATIC: Dop = Dop::new(Opcodes::INVOKE_STATIC, Opcodes::INVOKE_STATIC, Opcodes::INVOKE_STATIC_RANGE, &Form35c::THE_ONE, false);
let static INVOKE_INTERFACE: Dop = Dop::new(Opcodes::INVOKE_INTERFACE, Opcodes::INVOKE_INTERFACE, Opcodes::INVOKE_INTERFACE_RANGE, &Form35c::THE_ONE, false);
let static INVOKE_VIRTUAL_RANGE: Dop = Dop::new(Opcodes::INVOKE_VIRTUAL_RANGE, Opcodes::INVOKE_VIRTUAL, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static INVOKE_SUPER_RANGE: Dop = Dop::new(Opcodes::INVOKE_SUPER_RANGE, Opcodes::INVOKE_SUPER, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static INVOKE_DIRECT_RANGE: Dop = Dop::new(Opcodes::INVOKE_DIRECT_RANGE, Opcodes::INVOKE_DIRECT, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static INVOKE_STATIC_RANGE: Dop = Dop::new(Opcodes::INVOKE_STATIC_RANGE, Opcodes::INVOKE_STATIC, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static INVOKE_INTERFACE_RANGE: Dop = Dop::new(Opcodes::INVOKE_INTERFACE_RANGE, Opcodes::INVOKE_INTERFACE, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static NEG_INT: Dop = Dop::new(Opcodes::NEG_INT, Opcodes::NEG_INT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static NOT_INT: Dop = Dop::new(Opcodes::NOT_INT, Opcodes::NOT_INT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static NEG_LONG: Dop = Dop::new(Opcodes::NEG_LONG, Opcodes::NEG_LONG, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static NOT_LONG: Dop = Dop::new(Opcodes::NOT_LONG, Opcodes::NOT_LONG, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static NEG_FLOAT: Dop = Dop::new(Opcodes::NEG_FLOAT, Opcodes::NEG_FLOAT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static NEG_DOUBLE: Dop = Dop::new(Opcodes::NEG_DOUBLE, Opcodes::NEG_DOUBLE, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static INT_TO_LONG: Dop = Dop::new(Opcodes::INT_TO_LONG, Opcodes::INT_TO_LONG, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static INT_TO_FLOAT: Dop = Dop::new(Opcodes::INT_TO_FLOAT, Opcodes::INT_TO_FLOAT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static INT_TO_DOUBLE: Dop = Dop::new(Opcodes::INT_TO_DOUBLE, Opcodes::INT_TO_DOUBLE, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static LONG_TO_INT: Dop = Dop::new(Opcodes::LONG_TO_INT, Opcodes::LONG_TO_INT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static LONG_TO_FLOAT: Dop = Dop::new(Opcodes::LONG_TO_FLOAT, Opcodes::LONG_TO_FLOAT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static LONG_TO_DOUBLE: Dop = Dop::new(Opcodes::LONG_TO_DOUBLE, Opcodes::LONG_TO_DOUBLE, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static FLOAT_TO_INT: Dop = Dop::new(Opcodes::FLOAT_TO_INT, Opcodes::FLOAT_TO_INT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static FLOAT_TO_LONG: Dop = Dop::new(Opcodes::FLOAT_TO_LONG, Opcodes::FLOAT_TO_LONG, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static FLOAT_TO_DOUBLE: Dop = Dop::new(Opcodes::FLOAT_TO_DOUBLE, Opcodes::FLOAT_TO_DOUBLE, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static DOUBLE_TO_INT: Dop = Dop::new(Opcodes::DOUBLE_TO_INT, Opcodes::DOUBLE_TO_INT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static DOUBLE_TO_LONG: Dop = Dop::new(Opcodes::DOUBLE_TO_LONG, Opcodes::DOUBLE_TO_LONG, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static DOUBLE_TO_FLOAT: Dop = Dop::new(Opcodes::DOUBLE_TO_FLOAT, Opcodes::DOUBLE_TO_FLOAT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static INT_TO_BYTE: Dop = Dop::new(Opcodes::INT_TO_BYTE, Opcodes::INT_TO_BYTE, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static INT_TO_CHAR: Dop = Dop::new(Opcodes::INT_TO_CHAR, Opcodes::INT_TO_CHAR, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static INT_TO_SHORT: Dop = Dop::new(Opcodes::INT_TO_SHORT, Opcodes::INT_TO_SHORT, Opcodes::NO_NEXT, &Form12x::THE_ONE, true);
let static ADD_INT: Dop = Dop::new(Opcodes::ADD_INT, Opcodes::ADD_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SUB_INT: Dop = Dop::new(Opcodes::SUB_INT, Opcodes::SUB_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static MUL_INT: Dop = Dop::new(Opcodes::MUL_INT, Opcodes::MUL_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static DIV_INT: Dop = Dop::new(Opcodes::DIV_INT, Opcodes::DIV_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static REM_INT: Dop = Dop::new(Opcodes::REM_INT, Opcodes::REM_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AND_INT: Dop = Dop::new(Opcodes::AND_INT, Opcodes::AND_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static OR_INT: Dop = Dop::new(Opcodes::OR_INT, Opcodes::OR_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static XOR_INT: Dop = Dop::new(Opcodes::XOR_INT, Opcodes::XOR_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SHL_INT: Dop = Dop::new(Opcodes::SHL_INT, Opcodes::SHL_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SHR_INT: Dop = Dop::new(Opcodes::SHR_INT, Opcodes::SHR_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static USHR_INT: Dop = Dop::new(Opcodes::USHR_INT, Opcodes::USHR_INT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static ADD_LONG: Dop = Dop::new(Opcodes::ADD_LONG, Opcodes::ADD_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SUB_LONG: Dop = Dop::new(Opcodes::SUB_LONG, Opcodes::SUB_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static MUL_LONG: Dop = Dop::new(Opcodes::MUL_LONG, Opcodes::MUL_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static DIV_LONG: Dop = Dop::new(Opcodes::DIV_LONG, Opcodes::DIV_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static REM_LONG: Dop = Dop::new(Opcodes::REM_LONG, Opcodes::REM_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static AND_LONG: Dop = Dop::new(Opcodes::AND_LONG, Opcodes::AND_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static OR_LONG: Dop = Dop::new(Opcodes::OR_LONG, Opcodes::OR_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static XOR_LONG: Dop = Dop::new(Opcodes::XOR_LONG, Opcodes::XOR_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SHL_LONG: Dop = Dop::new(Opcodes::SHL_LONG, Opcodes::SHL_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SHR_LONG: Dop = Dop::new(Opcodes::SHR_LONG, Opcodes::SHR_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static USHR_LONG: Dop = Dop::new(Opcodes::USHR_LONG, Opcodes::USHR_LONG, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static ADD_FLOAT: Dop = Dop::new(Opcodes::ADD_FLOAT, Opcodes::ADD_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SUB_FLOAT: Dop = Dop::new(Opcodes::SUB_FLOAT, Opcodes::SUB_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static MUL_FLOAT: Dop = Dop::new(Opcodes::MUL_FLOAT, Opcodes::MUL_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static DIV_FLOAT: Dop = Dop::new(Opcodes::DIV_FLOAT, Opcodes::DIV_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static REM_FLOAT: Dop = Dop::new(Opcodes::REM_FLOAT, Opcodes::REM_FLOAT, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static ADD_DOUBLE: Dop = Dop::new(Opcodes::ADD_DOUBLE, Opcodes::ADD_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static SUB_DOUBLE: Dop = Dop::new(Opcodes::SUB_DOUBLE, Opcodes::SUB_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static MUL_DOUBLE: Dop = Dop::new(Opcodes::MUL_DOUBLE, Opcodes::MUL_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static DIV_DOUBLE: Dop = Dop::new(Opcodes::DIV_DOUBLE, Opcodes::DIV_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static REM_DOUBLE: Dop = Dop::new(Opcodes::REM_DOUBLE, Opcodes::REM_DOUBLE, Opcodes::NO_NEXT, &Form23x::THE_ONE, true);
let static ADD_INT_2ADDR: Dop = Dop::new(Opcodes::ADD_INT_2ADDR, Opcodes::ADD_INT, Opcodes::ADD_INT, &Form12x::THE_ONE, true);
let static SUB_INT_2ADDR: Dop = Dop::new(Opcodes::SUB_INT_2ADDR, Opcodes::SUB_INT, Opcodes::SUB_INT, &Form12x::THE_ONE, true);
let static MUL_INT_2ADDR: Dop = Dop::new(Opcodes::MUL_INT_2ADDR, Opcodes::MUL_INT, Opcodes::MUL_INT, &Form12x::THE_ONE, true);
let static DIV_INT_2ADDR: Dop = Dop::new(Opcodes::DIV_INT_2ADDR, Opcodes::DIV_INT, Opcodes::DIV_INT, &Form12x::THE_ONE, true);
let static REM_INT_2ADDR: Dop = Dop::new(Opcodes::REM_INT_2ADDR, Opcodes::REM_INT, Opcodes::REM_INT, &Form12x::THE_ONE, true);
let static AND_INT_2ADDR: Dop = Dop::new(Opcodes::AND_INT_2ADDR, Opcodes::AND_INT, Opcodes::AND_INT, &Form12x::THE_ONE, true);
let static OR_INT_2ADDR: Dop = Dop::new(Opcodes::OR_INT_2ADDR, Opcodes::OR_INT, Opcodes::OR_INT, &Form12x::THE_ONE, true);
let static XOR_INT_2ADDR: Dop = Dop::new(Opcodes::XOR_INT_2ADDR, Opcodes::XOR_INT, Opcodes::XOR_INT, &Form12x::THE_ONE, true);
let static SHL_INT_2ADDR: Dop = Dop::new(Opcodes::SHL_INT_2ADDR, Opcodes::SHL_INT, Opcodes::SHL_INT, &Form12x::THE_ONE, true);
let static SHR_INT_2ADDR: Dop = Dop::new(Opcodes::SHR_INT_2ADDR, Opcodes::SHR_INT, Opcodes::SHR_INT, &Form12x::THE_ONE, true);
let static USHR_INT_2ADDR: Dop = Dop::new(Opcodes::USHR_INT_2ADDR, Opcodes::USHR_INT, Opcodes::USHR_INT, &Form12x::THE_ONE, true);
let static ADD_LONG_2ADDR: Dop = Dop::new(Opcodes::ADD_LONG_2ADDR, Opcodes::ADD_LONG, Opcodes::ADD_LONG, &Form12x::THE_ONE, true);
let static SUB_LONG_2ADDR: Dop = Dop::new(Opcodes::SUB_LONG_2ADDR, Opcodes::SUB_LONG, Opcodes::SUB_LONG, &Form12x::THE_ONE, true);
let static MUL_LONG_2ADDR: Dop = Dop::new(Opcodes::MUL_LONG_2ADDR, Opcodes::MUL_LONG, Opcodes::MUL_LONG, &Form12x::THE_ONE, true);
let static DIV_LONG_2ADDR: Dop = Dop::new(Opcodes::DIV_LONG_2ADDR, Opcodes::DIV_LONG, Opcodes::DIV_LONG, &Form12x::THE_ONE, true);
let static REM_LONG_2ADDR: Dop = Dop::new(Opcodes::REM_LONG_2ADDR, Opcodes::REM_LONG, Opcodes::REM_LONG, &Form12x::THE_ONE, true);
let static AND_LONG_2ADDR: Dop = Dop::new(Opcodes::AND_LONG_2ADDR, Opcodes::AND_LONG, Opcodes::AND_LONG, &Form12x::THE_ONE, true);
let static OR_LONG_2ADDR: Dop = Dop::new(Opcodes::OR_LONG_2ADDR, Opcodes::OR_LONG, Opcodes::OR_LONG, &Form12x::THE_ONE, true);
let static XOR_LONG_2ADDR: Dop = Dop::new(Opcodes::XOR_LONG_2ADDR, Opcodes::XOR_LONG, Opcodes::XOR_LONG, &Form12x::THE_ONE, true);
let static SHL_LONG_2ADDR: Dop = Dop::new(Opcodes::SHL_LONG_2ADDR, Opcodes::SHL_LONG, Opcodes::SHL_LONG, &Form12x::THE_ONE, true);
let static SHR_LONG_2ADDR: Dop = Dop::new(Opcodes::SHR_LONG_2ADDR, Opcodes::SHR_LONG, Opcodes::SHR_LONG, &Form12x::THE_ONE, true);
let static USHR_LONG_2ADDR: Dop = Dop::new(Opcodes::USHR_LONG_2ADDR, Opcodes::USHR_LONG, Opcodes::USHR_LONG, &Form12x::THE_ONE, true);
let static ADD_FLOAT_2ADDR: Dop = Dop::new(Opcodes::ADD_FLOAT_2ADDR, Opcodes::ADD_FLOAT, Opcodes::ADD_FLOAT, &Form12x::THE_ONE, true);
let static SUB_FLOAT_2ADDR: Dop = Dop::new(Opcodes::SUB_FLOAT_2ADDR, Opcodes::SUB_FLOAT, Opcodes::SUB_FLOAT, &Form12x::THE_ONE, true);
let static MUL_FLOAT_2ADDR: Dop = Dop::new(Opcodes::MUL_FLOAT_2ADDR, Opcodes::MUL_FLOAT, Opcodes::MUL_FLOAT, &Form12x::THE_ONE, true);
let static DIV_FLOAT_2ADDR: Dop = Dop::new(Opcodes::DIV_FLOAT_2ADDR, Opcodes::DIV_FLOAT, Opcodes::DIV_FLOAT, &Form12x::THE_ONE, true);
let static REM_FLOAT_2ADDR: Dop = Dop::new(Opcodes::REM_FLOAT_2ADDR, Opcodes::REM_FLOAT, Opcodes::REM_FLOAT, &Form12x::THE_ONE, true);
let static ADD_DOUBLE_2ADDR: Dop = Dop::new(Opcodes::ADD_DOUBLE_2ADDR, Opcodes::ADD_DOUBLE, Opcodes::ADD_DOUBLE, &Form12x::THE_ONE, true);
let static SUB_DOUBLE_2ADDR: Dop = Dop::new(Opcodes::SUB_DOUBLE_2ADDR, Opcodes::SUB_DOUBLE, Opcodes::SUB_DOUBLE, &Form12x::THE_ONE, true);
let static MUL_DOUBLE_2ADDR: Dop = Dop::new(Opcodes::MUL_DOUBLE_2ADDR, Opcodes::MUL_DOUBLE, Opcodes::MUL_DOUBLE, &Form12x::THE_ONE, true);
let static DIV_DOUBLE_2ADDR: Dop = Dop::new(Opcodes::DIV_DOUBLE_2ADDR, Opcodes::DIV_DOUBLE, Opcodes::DIV_DOUBLE, &Form12x::THE_ONE, true);
let static REM_DOUBLE_2ADDR: Dop = Dop::new(Opcodes::REM_DOUBLE_2ADDR, Opcodes::REM_DOUBLE, Opcodes::REM_DOUBLE, &Form12x::THE_ONE, true);
let static ADD_INT_LIT16: Dop = Dop::new(Opcodes::ADD_INT_LIT16, Opcodes::ADD_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static RSUB_INT: Dop = Dop::new(Opcodes::RSUB_INT, Opcodes::RSUB_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static MUL_INT_LIT16: Dop = Dop::new(Opcodes::MUL_INT_LIT16, Opcodes::MUL_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static DIV_INT_LIT16: Dop = Dop::new(Opcodes::DIV_INT_LIT16, Opcodes::DIV_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static REM_INT_LIT16: Dop = Dop::new(Opcodes::REM_INT_LIT16, Opcodes::REM_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static AND_INT_LIT16: Dop = Dop::new(Opcodes::AND_INT_LIT16, Opcodes::AND_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static OR_INT_LIT16: Dop = Dop::new(Opcodes::OR_INT_LIT16, Opcodes::OR_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static XOR_INT_LIT16: Dop = Dop::new(Opcodes::XOR_INT_LIT16, Opcodes::XOR_INT, Opcodes::NO_NEXT, &Form22s::THE_ONE, true);
let static ADD_INT_LIT8: Dop = Dop::new(Opcodes::ADD_INT_LIT8, Opcodes::ADD_INT, Opcodes::ADD_INT_LIT16, &Form22b::THE_ONE, true);
let static RSUB_INT_LIT8: Dop = Dop::new(Opcodes::RSUB_INT_LIT8, Opcodes::RSUB_INT, Opcodes::RSUB_INT, &Form22b::THE_ONE, true);
let static MUL_INT_LIT8: Dop = Dop::new(Opcodes::MUL_INT_LIT8, Opcodes::MUL_INT, Opcodes::MUL_INT_LIT16, &Form22b::THE_ONE, true);
let static DIV_INT_LIT8: Dop = Dop::new(Opcodes::DIV_INT_LIT8, Opcodes::DIV_INT, Opcodes::DIV_INT_LIT16, &Form22b::THE_ONE, true);
let static REM_INT_LIT8: Dop = Dop::new(Opcodes::REM_INT_LIT8, Opcodes::REM_INT, Opcodes::REM_INT_LIT16, &Form22b::THE_ONE, true);
let static AND_INT_LIT8: Dop = Dop::new(Opcodes::AND_INT_LIT8, Opcodes::AND_INT, Opcodes::AND_INT_LIT16, &Form22b::THE_ONE, true);
let static OR_INT_LIT8: Dop = Dop::new(Opcodes::OR_INT_LIT8, Opcodes::OR_INT, Opcodes::OR_INT_LIT16, &Form22b::THE_ONE, true);
let static XOR_INT_LIT8: Dop = Dop::new(Opcodes::XOR_INT_LIT8, Opcodes::XOR_INT, Opcodes::XOR_INT_LIT16, &Form22b::THE_ONE, true);
let static SHL_INT_LIT8: Dop = Dop::new(Opcodes::SHL_INT_LIT8, Opcodes::SHL_INT, Opcodes::NO_NEXT, &Form22b::THE_ONE, true);
let static SHR_INT_LIT8: Dop = Dop::new(Opcodes::SHR_INT_LIT8, Opcodes::SHR_INT, Opcodes::NO_NEXT, &Form22b::THE_ONE, true);
let static USHR_INT_LIT8: Dop = Dop::new(Opcodes::USHR_INT_LIT8, Opcodes::USHR_INT, Opcodes::NO_NEXT, &Form22b::THE_ONE, true);
let static INVOKE_POLYMORPHIC: Dop = Dop::new(Opcodes::INVOKE_POLYMORPHIC, Opcodes::INVOKE_POLYMORPHIC, Opcodes::INVOKE_POLYMORPHIC_RANGE, &Form45cc::THE_ONE, false);
let static INVOKE_POLYMORPHIC_RANGE: Dop = Dop::new(Opcodes::INVOKE_POLYMORPHIC_RANGE, Opcodes::INVOKE_POLYMORPHIC, Opcodes::NO_NEXT, &Form4rcc::THE_ONE, false);
let static INVOKE_CUSTOM: Dop = Dop::new(Opcodes::INVOKE_CUSTOM, Opcodes::INVOKE_CUSTOM, Opcodes::INVOKE_CUSTOM_RANGE, &Form35c::THE_ONE, false);
let static INVOKE_CUSTOM_RANGE: Dop = Dop::new(Opcodes::INVOKE_CUSTOM_RANGE, Opcodes::INVOKE_CUSTOM, Opcodes::NO_NEXT, &Form3rc::THE_ONE, false);
let static CONST_METHOD_HANDLE: Dop = Dop::new(Opcodes::CONST_METHOD_HANDLE, Opcodes::CONST_METHOD_HANDLE, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
let static CONST_METHOD_TYPE: Dop = Dop::new(Opcodes::CONST_METHOD_TYPE, Opcodes::CONST_METHOD_TYPE, Opcodes::NO_NEXT, &Form21c::THE_ONE, true);
struct Dops{
}
impl Dops{
    pub fn new(&self)    {
    }
    pub fn get(opcode: i32) -> Dop    {
        let idx: i32 = opcode-Opcodes::MIN_VALUE;
        try        {
            let result: Dop = Dops::DOPS[idx];
            if result!=None            {
                return result;
            }            
        }        catch(        let ex: ArrayIndexOutOfBoundsException)        {
        }
        throw IllegalArgumentException::new("bogus opcode");
    }
    pub fn getNextOrNull(opcode: &Dop, options: &DexOptions) -> Dop    {
        let nextOpcode: i32 = opcode.getNextOpcode();
        if nextOpcode==Opcodes::NO_NEXT        {
            return None;
        }        
        opcode=Dops::get(nextOpcode);
        return opcode;
    }
    pub fn set(opcode: &Dop)    {
        let idx: i32 = opcode.getOpcode()-Opcodes::MIN_VALUE;
        Dops::DOPS[idx]=opcode;
    }
}