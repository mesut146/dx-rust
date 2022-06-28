use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::cst::CstMethodRef;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::Exceptions;

let static NOP: Rop = Rop::new(RegOps::NOP, &Type::VOID, &StdTypeList::EMPTY, "nop");
let static MOVE_INT: Rop = Rop::new(RegOps::MOVE, &Type::INT, &StdTypeList::INT, "move-int");
let static MOVE_LONG: Rop = Rop::new(RegOps::MOVE, &Type::LONG, &StdTypeList::LONG, "move-long");
let static MOVE_FLOAT: Rop = Rop::new(RegOps::MOVE, &Type::FLOAT, &StdTypeList::FLOAT, "move-float");
let static MOVE_DOUBLE: Rop = Rop::new(RegOps::MOVE, &Type::DOUBLE, &StdTypeList::DOUBLE, "move-double");
let static MOVE_OBJECT: Rop = Rop::new(RegOps::MOVE, &Type::OBJECT, &StdTypeList::OBJECT, "move-object");
let static MOVE_RETURN_ADDRESS: Rop = Rop::new(RegOps::MOVE, &Type::RETURN_ADDRESS, &StdTypeList::RETURN_ADDRESS, "move-return-address");
let static MOVE_PARAM_INT: Rop = Rop::new(RegOps::MOVE_PARAM, &Type::INT, &StdTypeList::EMPTY, "move-param-int");
let static MOVE_PARAM_LONG: Rop = Rop::new(RegOps::MOVE_PARAM, &Type::LONG, &StdTypeList::EMPTY, "move-param-long");
let static MOVE_PARAM_FLOAT: Rop = Rop::new(RegOps::MOVE_PARAM, &Type::FLOAT, &StdTypeList::EMPTY, "move-param-float");
let static MOVE_PARAM_DOUBLE: Rop = Rop::new(RegOps::MOVE_PARAM, &Type::DOUBLE, &StdTypeList::EMPTY, "move-param-double");
let static MOVE_PARAM_OBJECT: Rop = Rop::new(RegOps::MOVE_PARAM, &Type::OBJECT, &StdTypeList::EMPTY, "move-param-object");
let static CONST_INT: Rop = Rop::new(RegOps::CONST, &Type::INT, &StdTypeList::EMPTY, "const-int");
let static CONST_LONG: Rop = Rop::new(RegOps::CONST, &Type::LONG, &StdTypeList::EMPTY, "const-long");
let static CONST_FLOAT: Rop = Rop::new(RegOps::CONST, &Type::FLOAT, &StdTypeList::EMPTY, "const-float");
let static CONST_DOUBLE: Rop = Rop::new(RegOps::CONST, &Type::DOUBLE, &StdTypeList::EMPTY, "const-double");
let static CONST_OBJECT: Rop = Rop::new(RegOps::CONST, &Type::OBJECT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "const-object");
let static CONST_OBJECT_NOTHROW: Rop = Rop::new(RegOps::CONST, &Type::OBJECT, &StdTypeList::EMPTY, "const-object-nothrow");
let static GOTO: Rop = Rop::new(RegOps::GOTO, &Type::VOID, &StdTypeList::EMPTY, Rop::BRANCH_GOTO, "goto");
let static IF_EQZ_INT: Rop = Rop::new(RegOps::IF_EQ, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_IF, "if-eqz-int");
let static IF_NEZ_INT: Rop = Rop::new(RegOps::IF_NE, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_IF, "if-nez-int");
let static IF_LTZ_INT: Rop = Rop::new(RegOps::IF_LT, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_IF, "if-ltz-int");
let static IF_GEZ_INT: Rop = Rop::new(RegOps::IF_GE, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_IF, "if-gez-int");
let static IF_LEZ_INT: Rop = Rop::new(RegOps::IF_LE, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_IF, "if-lez-int");
let static IF_GTZ_INT: Rop = Rop::new(RegOps::IF_GT, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_IF, "if-gtz-int");
let static IF_EQZ_OBJECT: Rop = Rop::new(RegOps::IF_EQ, &Type::VOID, &StdTypeList::OBJECT, Rop::BRANCH_IF, "if-eqz-object");
let static IF_NEZ_OBJECT: Rop = Rop::new(RegOps::IF_NE, &Type::VOID, &StdTypeList::OBJECT, Rop::BRANCH_IF, "if-nez-object");
let static IF_EQ_INT: Rop = Rop::new(RegOps::IF_EQ, &Type::VOID, &StdTypeList::INT_INT, Rop::BRANCH_IF, "if-eq-int");
let static IF_NE_INT: Rop = Rop::new(RegOps::IF_NE, &Type::VOID, &StdTypeList::INT_INT, Rop::BRANCH_IF, "if-ne-int");
let static IF_LT_INT: Rop = Rop::new(RegOps::IF_LT, &Type::VOID, &StdTypeList::INT_INT, Rop::BRANCH_IF, "if-lt-int");
let static IF_GE_INT: Rop = Rop::new(RegOps::IF_GE, &Type::VOID, &StdTypeList::INT_INT, Rop::BRANCH_IF, "if-ge-int");
let static IF_LE_INT: Rop = Rop::new(RegOps::IF_LE, &Type::VOID, &StdTypeList::INT_INT, Rop::BRANCH_IF, "if-le-int");
let static IF_GT_INT: Rop = Rop::new(RegOps::IF_GT, &Type::VOID, &StdTypeList::INT_INT, Rop::BRANCH_IF, "if-gt-int");
let static IF_EQ_OBJECT: Rop = Rop::new(RegOps::IF_EQ, &Type::VOID, &StdTypeList::OBJECT_OBJECT, Rop::BRANCH_IF, "if-eq-object");
let static IF_NE_OBJECT: Rop = Rop::new(RegOps::IF_NE, &Type::VOID, &StdTypeList::OBJECT_OBJECT, Rop::BRANCH_IF, "if-ne-object");
let static SWITCH: Rop = Rop::new(RegOps::SWITCH, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_SWITCH, "switch");
let static ADD_INT: Rop = Rop::new(RegOps::ADD, &Type::INT, &StdTypeList::INT_INT, "add-int");
let static ADD_LONG: Rop = Rop::new(RegOps::ADD, &Type::LONG, &StdTypeList::LONG_LONG, "add-long");
let static ADD_FLOAT: Rop = Rop::new(RegOps::ADD, &Type::FLOAT, &StdTypeList::FLOAT_FLOAT, "add-float");
let static ADD_DOUBLE: Rop = Rop::new(RegOps::ADD, &Type::DOUBLE, &StdTypeList::DOUBLE_DOUBLE, Rop::BRANCH_NONE, "add-double");
let static SUB_INT: Rop = Rop::new(RegOps::SUB, &Type::INT, &StdTypeList::INT_INT, "sub-int");
let static SUB_LONG: Rop = Rop::new(RegOps::SUB, &Type::LONG, &StdTypeList::LONG_LONG, "sub-long");
let static SUB_FLOAT: Rop = Rop::new(RegOps::SUB, &Type::FLOAT, &StdTypeList::FLOAT_FLOAT, "sub-float");
let static SUB_DOUBLE: Rop = Rop::new(RegOps::SUB, &Type::DOUBLE, &StdTypeList::DOUBLE_DOUBLE, Rop::BRANCH_NONE, "sub-double");
let static MUL_INT: Rop = Rop::new(RegOps::MUL, &Type::INT, &StdTypeList::INT_INT, "mul-int");
let static MUL_LONG: Rop = Rop::new(RegOps::MUL, &Type::LONG, &StdTypeList::LONG_LONG, "mul-long");
let static MUL_FLOAT: Rop = Rop::new(RegOps::MUL, &Type::FLOAT, &StdTypeList::FLOAT_FLOAT, "mul-float");
let static MUL_DOUBLE: Rop = Rop::new(RegOps::MUL, &Type::DOUBLE, &StdTypeList::DOUBLE_DOUBLE, Rop::BRANCH_NONE, "mul-double");
let static DIV_INT: Rop = Rop::new(RegOps::DIV, &Type::INT, &StdTypeList::INT_INT, &Exceptions::LIST_Error_ArithmeticException, "div-int");
let static DIV_LONG: Rop = Rop::new(RegOps::DIV, &Type::LONG, &StdTypeList::LONG_LONG, &Exceptions::LIST_Error_ArithmeticException, "div-long");
let static DIV_FLOAT: Rop = Rop::new(RegOps::DIV, &Type::FLOAT, &StdTypeList::FLOAT_FLOAT, "div-float");
let static DIV_DOUBLE: Rop = Rop::new(RegOps::DIV, &Type::DOUBLE, &StdTypeList::DOUBLE_DOUBLE, "div-double");
let static REM_INT: Rop = Rop::new(RegOps::REM, &Type::INT, &StdTypeList::INT_INT, &Exceptions::LIST_Error_ArithmeticException, "rem-int");
let static REM_LONG: Rop = Rop::new(RegOps::REM, &Type::LONG, &StdTypeList::LONG_LONG, &Exceptions::LIST_Error_ArithmeticException, "rem-long");
let static REM_FLOAT: Rop = Rop::new(RegOps::REM, &Type::FLOAT, &StdTypeList::FLOAT_FLOAT, "rem-float");
let static REM_DOUBLE: Rop = Rop::new(RegOps::REM, &Type::DOUBLE, &StdTypeList::DOUBLE_DOUBLE, "rem-double");
let static NEG_INT: Rop = Rop::new(RegOps::NEG, &Type::INT, &StdTypeList::INT, "neg-int");
let static NEG_LONG: Rop = Rop::new(RegOps::NEG, &Type::LONG, &StdTypeList::LONG, "neg-long");
let static NEG_FLOAT: Rop = Rop::new(RegOps::NEG, &Type::FLOAT, &StdTypeList::FLOAT, "neg-float");
let static NEG_DOUBLE: Rop = Rop::new(RegOps::NEG, &Type::DOUBLE, &StdTypeList::DOUBLE, "neg-double");
let static AND_INT: Rop = Rop::new(RegOps::AND, &Type::INT, &StdTypeList::INT_INT, "and-int");
let static AND_LONG: Rop = Rop::new(RegOps::AND, &Type::LONG, &StdTypeList::LONG_LONG, "and-long");
let static OR_INT: Rop = Rop::new(RegOps::OR, &Type::INT, &StdTypeList::INT_INT, "or-int");
let static OR_LONG: Rop = Rop::new(RegOps::OR, &Type::LONG, &StdTypeList::LONG_LONG, "or-long");
let static XOR_INT: Rop = Rop::new(RegOps::XOR, &Type::INT, &StdTypeList::INT_INT, "xor-int");
let static XOR_LONG: Rop = Rop::new(RegOps::XOR, &Type::LONG, &StdTypeList::LONG_LONG, "xor-long");
let static SHL_INT: Rop = Rop::new(RegOps::SHL, &Type::INT, &StdTypeList::INT_INT, "shl-int");
let static SHL_LONG: Rop = Rop::new(RegOps::SHL, &Type::LONG, &StdTypeList::LONG_INT, "shl-long");
let static SHR_INT: Rop = Rop::new(RegOps::SHR, &Type::INT, &StdTypeList::INT_INT, "shr-int");
let static SHR_LONG: Rop = Rop::new(RegOps::SHR, &Type::LONG, &StdTypeList::LONG_INT, "shr-long");
let static USHR_INT: Rop = Rop::new(RegOps::USHR, &Type::INT, &StdTypeList::INT_INT, "ushr-int");
let static USHR_LONG: Rop = Rop::new(RegOps::USHR, &Type::LONG, &StdTypeList::LONG_INT, "ushr-long");
let static NOT_INT: Rop = Rop::new(RegOps::NOT, &Type::INT, &StdTypeList::INT, "not-int");
let static NOT_LONG: Rop = Rop::new(RegOps::NOT, &Type::LONG, &StdTypeList::LONG, "not-long");
let static ADD_CONST_INT: Rop = Rop::new(RegOps::ADD, &Type::INT, &StdTypeList::INT, "add-const-int");
let static ADD_CONST_LONG: Rop = Rop::new(RegOps::ADD, &Type::LONG, &StdTypeList::LONG, "add-const-long");
let static ADD_CONST_FLOAT: Rop = Rop::new(RegOps::ADD, &Type::FLOAT, &StdTypeList::FLOAT, "add-const-float");
let static ADD_CONST_DOUBLE: Rop = Rop::new(RegOps::ADD, &Type::DOUBLE, &StdTypeList::DOUBLE, "add-const-double");
let static SUB_CONST_INT: Rop = Rop::new(RegOps::SUB, &Type::INT, &StdTypeList::INT, "sub-const-int");
let static SUB_CONST_LONG: Rop = Rop::new(RegOps::SUB, &Type::LONG, &StdTypeList::LONG, "sub-const-long");
let static SUB_CONST_FLOAT: Rop = Rop::new(RegOps::SUB, &Type::FLOAT, &StdTypeList::FLOAT, "sub-const-float");
let static SUB_CONST_DOUBLE: Rop = Rop::new(RegOps::SUB, &Type::DOUBLE, &StdTypeList::DOUBLE, "sub-const-double");
let static MUL_CONST_INT: Rop = Rop::new(RegOps::MUL, &Type::INT, &StdTypeList::INT, "mul-const-int");
let static MUL_CONST_LONG: Rop = Rop::new(RegOps::MUL, &Type::LONG, &StdTypeList::LONG, "mul-const-long");
let static MUL_CONST_FLOAT: Rop = Rop::new(RegOps::MUL, &Type::FLOAT, &StdTypeList::FLOAT, "mul-const-float");
let static MUL_CONST_DOUBLE: Rop = Rop::new(RegOps::MUL, &Type::DOUBLE, &StdTypeList::DOUBLE, "mul-const-double");
let static DIV_CONST_INT: Rop = Rop::new(RegOps::DIV, &Type::INT, &StdTypeList::INT, &Exceptions::LIST_Error_ArithmeticException, "div-const-int");
let static DIV_CONST_LONG: Rop = Rop::new(RegOps::DIV, &Type::LONG, &StdTypeList::LONG, &Exceptions::LIST_Error_ArithmeticException, "div-const-long");
let static DIV_CONST_FLOAT: Rop = Rop::new(RegOps::DIV, &Type::FLOAT, &StdTypeList::FLOAT, "div-const-float");
let static DIV_CONST_DOUBLE: Rop = Rop::new(RegOps::DIV, &Type::DOUBLE, &StdTypeList::DOUBLE, "div-const-double");
let static REM_CONST_INT: Rop = Rop::new(RegOps::REM, &Type::INT, &StdTypeList::INT, &Exceptions::LIST_Error_ArithmeticException, "rem-const-int");
let static REM_CONST_LONG: Rop = Rop::new(RegOps::REM, &Type::LONG, &StdTypeList::LONG, &Exceptions::LIST_Error_ArithmeticException, "rem-const-long");
let static REM_CONST_FLOAT: Rop = Rop::new(RegOps::REM, &Type::FLOAT, &StdTypeList::FLOAT, "rem-const-float");
let static REM_CONST_DOUBLE: Rop = Rop::new(RegOps::REM, &Type::DOUBLE, &StdTypeList::DOUBLE, "rem-const-double");
let static AND_CONST_INT: Rop = Rop::new(RegOps::AND, &Type::INT, &StdTypeList::INT, "and-const-int");
let static AND_CONST_LONG: Rop = Rop::new(RegOps::AND, &Type::LONG, &StdTypeList::LONG, "and-const-long");
let static OR_CONST_INT: Rop = Rop::new(RegOps::OR, &Type::INT, &StdTypeList::INT, "or-const-int");
let static OR_CONST_LONG: Rop = Rop::new(RegOps::OR, &Type::LONG, &StdTypeList::LONG, "or-const-long");
let static XOR_CONST_INT: Rop = Rop::new(RegOps::XOR, &Type::INT, &StdTypeList::INT, "xor-const-int");
let static XOR_CONST_LONG: Rop = Rop::new(RegOps::XOR, &Type::LONG, &StdTypeList::LONG, "xor-const-long");
let static SHL_CONST_INT: Rop = Rop::new(RegOps::SHL, &Type::INT, &StdTypeList::INT, "shl-const-int");
let static SHL_CONST_LONG: Rop = Rop::new(RegOps::SHL, &Type::LONG, &StdTypeList::INT, "shl-const-long");
let static SHR_CONST_INT: Rop = Rop::new(RegOps::SHR, &Type::INT, &StdTypeList::INT, "shr-const-int");
let static SHR_CONST_LONG: Rop = Rop::new(RegOps::SHR, &Type::LONG, &StdTypeList::INT, "shr-const-long");
let static USHR_CONST_INT: Rop = Rop::new(RegOps::USHR, &Type::INT, &StdTypeList::INT, "ushr-const-int");
let static USHR_CONST_LONG: Rop = Rop::new(RegOps::USHR, &Type::LONG, &StdTypeList::INT, "ushr-const-long");
let static CMPL_LONG: Rop = Rop::new(RegOps::CMPL, &Type::INT, &StdTypeList::LONG_LONG, "cmpl-long");
let static CMPL_FLOAT: Rop = Rop::new(RegOps::CMPL, &Type::INT, &StdTypeList::FLOAT_FLOAT, "cmpl-float");
let static CMPL_DOUBLE: Rop = Rop::new(RegOps::CMPL, &Type::INT, &StdTypeList::DOUBLE_DOUBLE, "cmpl-double");
let static CMPG_FLOAT: Rop = Rop::new(RegOps::CMPG, &Type::INT, &StdTypeList::FLOAT_FLOAT, "cmpg-float");
let static CMPG_DOUBLE: Rop = Rop::new(RegOps::CMPG, &Type::INT, &StdTypeList::DOUBLE_DOUBLE, "cmpg-double");
let static CONV_L2I: Rop = Rop::new(RegOps::CONV, &Type::INT, &StdTypeList::LONG, "conv-l2i");
let static CONV_F2I: Rop = Rop::new(RegOps::CONV, &Type::INT, &StdTypeList::FLOAT, "conv-f2i");
let static CONV_D2I: Rop = Rop::new(RegOps::CONV, &Type::INT, &StdTypeList::DOUBLE, "conv-d2i");
let static CONV_I2L: Rop = Rop::new(RegOps::CONV, &Type::LONG, &StdTypeList::INT, "conv-i2l");
let static CONV_F2L: Rop = Rop::new(RegOps::CONV, &Type::LONG, &StdTypeList::FLOAT, "conv-f2l");
let static CONV_D2L: Rop = Rop::new(RegOps::CONV, &Type::LONG, &StdTypeList::DOUBLE, "conv-d2l");
let static CONV_I2F: Rop = Rop::new(RegOps::CONV, &Type::FLOAT, &StdTypeList::INT, "conv-i2f");
let static CONV_L2F: Rop = Rop::new(RegOps::CONV, &Type::FLOAT, &StdTypeList::LONG, "conv-l2f");
let static CONV_D2F: Rop = Rop::new(RegOps::CONV, &Type::FLOAT, &StdTypeList::DOUBLE, "conv-d2f");
let static CONV_I2D: Rop = Rop::new(RegOps::CONV, &Type::DOUBLE, &StdTypeList::INT, "conv-i2d");
let static CONV_L2D: Rop = Rop::new(RegOps::CONV, &Type::DOUBLE, &StdTypeList::LONG, "conv-l2d");
let static CONV_F2D: Rop = Rop::new(RegOps::CONV, &Type::DOUBLE, &StdTypeList::FLOAT, "conv-f2d");
let static TO_BYTE: Rop = Rop::new(RegOps::TO_BYTE, &Type::INT, &StdTypeList::INT, "to-byte");
let static TO_CHAR: Rop = Rop::new(RegOps::TO_CHAR, &Type::INT, &StdTypeList::INT, "to-char");
let static TO_SHORT: Rop = Rop::new(RegOps::TO_SHORT, &Type::INT, &StdTypeList::INT, "to-short");
let static RETURN_VOID: Rop = Rop::new(RegOps::RETURN, &Type::VOID, &StdTypeList::EMPTY, Rop::BRANCH_RETURN, "return-void");
let static RETURN_INT: Rop = Rop::new(RegOps::RETURN, &Type::VOID, &StdTypeList::INT, Rop::BRANCH_RETURN, "return-int");
let static RETURN_LONG: Rop = Rop::new(RegOps::RETURN, &Type::VOID, &StdTypeList::LONG, Rop::BRANCH_RETURN, "return-long");
let static RETURN_FLOAT: Rop = Rop::new(RegOps::RETURN, &Type::VOID, &StdTypeList::FLOAT, Rop::BRANCH_RETURN, "return-float");
let static RETURN_DOUBLE: Rop = Rop::new(RegOps::RETURN, &Type::VOID, &StdTypeList::DOUBLE, Rop::BRANCH_RETURN, "return-double");
let static RETURN_OBJECT: Rop = Rop::new(RegOps::RETURN, &Type::VOID, &StdTypeList::OBJECT, Rop::BRANCH_RETURN, "return-object");
let static ARRAY_LENGTH: Rop = Rop::new(RegOps::ARRAY_LENGTH, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "array-length");
let static THROW: Rop = Rop::new(RegOps::THROW, &Type::VOID, &StdTypeList::THROWABLE, &StdTypeList::THROWABLE, "throw");
let static MONITOR_ENTER: Rop = Rop::new(RegOps::MONITOR_ENTER, &Type::VOID, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "monitor-enter");
let static MONITOR_EXIT: Rop = Rop::new(RegOps::MONITOR_EXIT, &Type::VOID, &StdTypeList::OBJECT, &Exceptions::LIST_Error_Null_IllegalMonitorStateException, "monitor-exit");
let static AGET_INT: Rop = Rop::new(RegOps::AGET, &Type::INT, &StdTypeList::INTARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-int");
let static AGET_LONG: Rop = Rop::new(RegOps::AGET, &Type::LONG, &StdTypeList::LONGARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-long");
let static AGET_FLOAT: Rop = Rop::new(RegOps::AGET, &Type::FLOAT, &StdTypeList::FLOATARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-float");
let static AGET_DOUBLE: Rop = Rop::new(RegOps::AGET, &Type::DOUBLE, &StdTypeList::DOUBLEARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-double");
let static AGET_OBJECT: Rop = Rop::new(RegOps::AGET, &Type::OBJECT, &StdTypeList::OBJECTARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-object");
let static AGET_BOOLEAN: Rop = Rop::new(RegOps::AGET, &Type::INT, &StdTypeList::BOOLEANARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-boolean");
let static AGET_BYTE: Rop = Rop::new(RegOps::AGET, &Type::INT, &StdTypeList::BYTEARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-byte");
let static AGET_CHAR: Rop = Rop::new(RegOps::AGET, &Type::INT, &StdTypeList::CHARARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-char");
let static AGET_SHORT: Rop = Rop::new(RegOps::AGET, &Type::INT, &StdTypeList::SHORTARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aget-short");
let static APUT_INT: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::INT_INTARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aput-int");
let static APUT_LONG: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::LONG_LONGARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aput-long");
let static APUT_FLOAT: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::FLOAT_FLOATARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aput-float");
let static APUT_DOUBLE: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::DOUBLE_DOUBLEARR_INT, &Exceptions::LIST_Error_Null_ArrayIndexOutOfBounds, "aput-double");
let static APUT_OBJECT: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::OBJECT_OBJECTARR_INT, &Exceptions::LIST_Error_Null_ArrayIndex_ArrayStore, "aput-object");
let static APUT_BOOLEAN: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::INT_BOOLEANARR_INT, &Exceptions::LIST_Error_Null_ArrayIndex_ArrayStore, "aput-boolean");
let static APUT_BYTE: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::INT_BYTEARR_INT, &Exceptions::LIST_Error_Null_ArrayIndex_ArrayStore, "aput-byte");
let static APUT_CHAR: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::INT_CHARARR_INT, &Exceptions::LIST_Error_Null_ArrayIndex_ArrayStore, "aput-char");
let static APUT_SHORT: Rop = Rop::new(RegOps::APUT, &Type::VOID, &StdTypeList::INT_SHORTARR_INT, &Exceptions::LIST_Error_Null_ArrayIndex_ArrayStore, "aput-short");
let static NEW_INSTANCE: Rop = Rop::new(RegOps::NEW_INSTANCE, &Type::OBJECT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "new-instance");
let static NEW_ARRAY_INT: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::INT_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-int");
let static NEW_ARRAY_LONG: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::LONG_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-long");
let static NEW_ARRAY_FLOAT: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::FLOAT_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-float");
let static NEW_ARRAY_DOUBLE: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::DOUBLE_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-double");
let static NEW_ARRAY_BOOLEAN: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::BOOLEAN_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-boolean");
let static NEW_ARRAY_BYTE: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::BYTE_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-byte");
let static NEW_ARRAY_CHAR: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::CHAR_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-char");
let static NEW_ARRAY_SHORT: Rop = Rop::new(RegOps::NEW_ARRAY, &Type::SHORT_ARRAY, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-short");
let static CHECK_CAST: Rop = Rop::new(RegOps::CHECK_CAST, &Type::VOID, &StdTypeList::OBJECT, &Exceptions::LIST_Error_ClassCastException, "check-cast");
let static INSTANCE_OF: Rop = Rop::new(RegOps::INSTANCE_OF, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error, "instance-of");
let static GET_FIELD_INT: Rop = Rop::new(RegOps::GET_FIELD, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-int");
let static GET_FIELD_LONG: Rop = Rop::new(RegOps::GET_FIELD, &Type::LONG, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-long");
let static GET_FIELD_FLOAT: Rop = Rop::new(RegOps::GET_FIELD, &Type::FLOAT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-float");
let static GET_FIELD_DOUBLE: Rop = Rop::new(RegOps::GET_FIELD, &Type::DOUBLE, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-double");
let static GET_FIELD_OBJECT: Rop = Rop::new(RegOps::GET_FIELD, &Type::OBJECT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-object");
let static GET_FIELD_BOOLEAN: Rop = Rop::new(RegOps::GET_FIELD, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-boolean");
let static GET_FIELD_BYTE: Rop = Rop::new(RegOps::GET_FIELD, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-byte");
let static GET_FIELD_CHAR: Rop = Rop::new(RegOps::GET_FIELD, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-char");
let static GET_FIELD_SHORT: Rop = Rop::new(RegOps::GET_FIELD, &Type::INT, &StdTypeList::OBJECT, &Exceptions::LIST_Error_NullPointerException, "get-field-short");
let static GET_STATIC_INT: Rop = Rop::new(RegOps::GET_STATIC, &Type::INT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-static-int");
let static GET_STATIC_LONG: Rop = Rop::new(RegOps::GET_STATIC, &Type::LONG, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-static-long");
let static GET_STATIC_FLOAT: Rop = Rop::new(RegOps::GET_STATIC, &Type::FLOAT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-static-float");
let static GET_STATIC_DOUBLE: Rop = Rop::new(RegOps::GET_STATIC, &Type::DOUBLE, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-static-double");
let static GET_STATIC_OBJECT: Rop = Rop::new(RegOps::GET_STATIC, &Type::OBJECT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-static-object");
let static GET_STATIC_BOOLEAN: Rop = Rop::new(RegOps::GET_STATIC, &Type::INT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-field-boolean");
let static GET_STATIC_BYTE: Rop = Rop::new(RegOps::GET_STATIC, &Type::INT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-field-byte");
let static GET_STATIC_CHAR: Rop = Rop::new(RegOps::GET_STATIC, &Type::INT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-field-char");
let static GET_STATIC_SHORT: Rop = Rop::new(RegOps::GET_STATIC, &Type::INT, &StdTypeList::EMPTY, &Exceptions::LIST_Error, "get-field-short");
let static PUT_FIELD_INT: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::INT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-int");
let static PUT_FIELD_LONG: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::LONG_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-long");
let static PUT_FIELD_FLOAT: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::FLOAT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-float");
let static PUT_FIELD_DOUBLE: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::DOUBLE_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-double");
let static PUT_FIELD_OBJECT: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::OBJECT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-object");
let static PUT_FIELD_BOOLEAN: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::INT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-boolean");
let static PUT_FIELD_BYTE: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::INT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-byte");
let static PUT_FIELD_CHAR: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::INT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-char");
let static PUT_FIELD_SHORT: Rop = Rop::new(RegOps::PUT_FIELD, &Type::VOID, &StdTypeList::INT_OBJECT, &Exceptions::LIST_Error_NullPointerException, "put-field-short");
let static PUT_STATIC_INT: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::INT, &Exceptions::LIST_Error, "put-static-int");
let static PUT_STATIC_LONG: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::LONG, &Exceptions::LIST_Error, "put-static-long");
let static PUT_STATIC_FLOAT: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::FLOAT, &Exceptions::LIST_Error, "put-static-float");
let static PUT_STATIC_DOUBLE: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::DOUBLE, &Exceptions::LIST_Error, "put-static-double");
let static PUT_STATIC_OBJECT: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::OBJECT, &Exceptions::LIST_Error, "put-static-object");
let static PUT_STATIC_BOOLEAN: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::INT, &Exceptions::LIST_Error, "put-static-boolean");
let static PUT_STATIC_BYTE: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::INT, &Exceptions::LIST_Error, "put-static-byte");
let static PUT_STATIC_CHAR: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::INT, &Exceptions::LIST_Error, "put-static-char");
let static PUT_STATIC_SHORT: Rop = Rop::new(RegOps::PUT_STATIC, &Type::VOID, &StdTypeList::INT, &Exceptions::LIST_Error, "put-static-short");
let static MARK_LOCAL_INT: Rop = Rop::new(RegOps::MARK_LOCAL, &Type::VOID, &StdTypeList::INT, "mark-local-int");
let static MARK_LOCAL_LONG: Rop = Rop::new(RegOps::MARK_LOCAL, &Type::VOID, &StdTypeList::LONG, "mark-local-long");
let static MARK_LOCAL_FLOAT: Rop = Rop::new(RegOps::MARK_LOCAL, &Type::VOID, &StdTypeList::FLOAT, "mark-local-float");
let static MARK_LOCAL_DOUBLE: Rop = Rop::new(RegOps::MARK_LOCAL, &Type::VOID, &StdTypeList::DOUBLE, "mark-local-double");
let static MARK_LOCAL_OBJECT: Rop = Rop::new(RegOps::MARK_LOCAL, &Type::VOID, &StdTypeList::OBJECT, "mark-local-object");
let static FILL_ARRAY_DATA: Rop = Rop::new(RegOps::FILL_ARRAY_DATA, &Type::VOID, &StdTypeList::EMPTY, "fill-array-data");
struct Rops{
}
impl Rops{
    pub fn ropFor(opcode: i32, dest: &TypeBearer, sources: &TypeList, cst: &Constant) -> Rop    {
        match opcode{RegOps::NOP =>             return Rops::NOP;RegOps::MOVE =>             return Rops::opMove(&dest);RegOps::MOVE_PARAM =>             return Rops::opMoveParam(&dest);RegOps::MOVE_EXCEPTION =>             return Rops::opMoveException(&dest);RegOps::CONST =>             return Rops::opConst(&dest);RegOps::GOTO =>             return Rops::GOTO;RegOps::IF_EQ =>             return Rops::opIfEq(&sources);RegOps::IF_NE =>             return Rops::opIfNe(&sources);RegOps::IF_LT =>             return Rops::opIfLt(&sources);RegOps::IF_GE =>             return Rops::opIfGe(&sources);RegOps::IF_LE =>             return Rops::opIfLe(&sources);RegOps::IF_GT =>             return Rops::opIfGt(&sources);RegOps::SWITCH =>             return Rops::SWITCH;RegOps::ADD =>             return Rops::opAdd(&sources);RegOps::SUB =>             return Rops::opSub(&sources);RegOps::MUL =>             return Rops::opMul(&sources);RegOps::DIV =>             return Rops::opDiv(&sources);RegOps::REM =>             return Rops::opRem(&sources);RegOps::NEG =>             return Rops::opNeg(&dest);RegOps::AND =>             return Rops::opAnd(&sources);RegOps::OR =>             return Rops::opOr(&sources);RegOps::XOR =>             return Rops::opXor(&sources);RegOps::SHL =>             return Rops::opShl(&sources);RegOps::SHR =>             return Rops::opShr(&sources);RegOps::USHR =>             return Rops::opUshr(&sources);RegOps::NOT =>             return Rops::opNot(&dest);RegOps::CMPL =>             return Rops::opCmpl(sources.getType(0));RegOps::CMPG =>             return Rops::opCmpg(sources.getType(0));RegOps::CONV =>             return Rops::opConv(&dest, sources.getType(0));RegOps::TO_BYTE =>             return Rops::TO_BYTE;RegOps::TO_CHAR =>             return Rops::TO_CHAR;RegOps::TO_SHORT =>             return Rops::TO_SHORT;RegOps::RETURN =>             {
                if sources.size()==0                {
                    return Rops::RETURN_VOID;
                }                
                return Rops::opReturn(sources.getType(0));
            }RegOps::ARRAY_LENGTH =>             return Rops::ARRAY_LENGTH;RegOps::THROW =>             return Rops::THROW;RegOps::MONITOR_ENTER =>             return Rops::MONITOR_ENTER;RegOps::MONITOR_EXIT =>             return Rops::MONITOR_EXIT;RegOps::AGET =>             {
                let source: Type = sources.getType(0);
                let componentType: Type;
                if source==Type::KNOWN_NULL                {
                    componentType=dest.getType();
                }                else                 {
                    componentType=source.getComponentType();
                }
                return Rops::opAget(&componentType);
            }RegOps::APUT =>             {
                let source: Type = sources.getType(1);
                let componentType: Type;
                if source==Type::KNOWN_NULL                {
                    componentType=sources.getType(0);
                }                else                 {
                    componentType=source.getComponentType();
                }
                return Rops::opAput(&componentType);
            }RegOps::NEW_INSTANCE =>             return Rops::NEW_INSTANCE;RegOps::NEW_ARRAY =>             return Rops::opNewArray(dest.getType());RegOps::CHECK_CAST =>             return Rops::CHECK_CAST;RegOps::INSTANCE_OF =>             return Rops::INSTANCE_OF;RegOps::GET_FIELD =>             return Rops::opGetField(&dest);RegOps::GET_STATIC =>             return Rops::opGetStatic(&dest);RegOps::PUT_FIELD =>             return Rops::opPutField(sources.getType(0));RegOps::PUT_STATIC =>             return Rops::opPutStatic(sources.getType(0));RegOps::INVOKE_STATIC =>             {
                return Rops::opInvokeStatic(((CstMethodRef*)cst).getPrototype());
            }RegOps::INVOKE_VIRTUAL =>             {
                let cstMeth: CstBaseMethodRef = (CstMethodRef*)cst;
                let meth: Prototype = cstMeth.getPrototype();
                let definer: CstType = cstMeth.getDefiningClass();
                meth=meth.withFirstParameter(definer.getClassType());
                return Rops::opInvokeVirtual(&meth);
            }RegOps::INVOKE_SUPER =>             {
                let cstMeth: CstBaseMethodRef = (CstMethodRef*)cst;
                let meth: Prototype = cstMeth.getPrototype();
                let definer: CstType = cstMeth.getDefiningClass();
                meth=meth.withFirstParameter(definer.getClassType());
                return Rops::opInvokeSuper(&meth);
            }RegOps::INVOKE_DIRECT =>             {
                let cstMeth: CstBaseMethodRef = (CstMethodRef*)cst;
                let meth: Prototype = cstMeth.getPrototype();
                let definer: CstType = cstMeth.getDefiningClass();
                meth=meth.withFirstParameter(definer.getClassType());
                return Rops::opInvokeDirect(&meth);
            }RegOps::INVOKE_INTERFACE =>             {
                let cstMeth: CstBaseMethodRef = (CstMethodRef*)cst;
                let meth: Prototype = cstMeth.getPrototype();
                let definer: CstType = cstMeth.getDefiningClass();
                meth=meth.withFirstParameter(definer.getClassType());
                return Rops::opInvokeInterface(&meth);
            }RegOps::INVOKE_POLYMORPHIC =>             {
                let cstMeth: CstBaseMethodRef = (CstMethodRef*)cst;
                let proto: Prototype = cstMeth.getPrototype();
                let definer: CstType = cstMeth.getDefiningClass();
                let meth: Prototype = proto.withFirstParameter(definer.getClassType());
                return Rops::opInvokePolymorphic(&meth);
            }RegOps::INVOKE_CUSTOM =>             {
                let cstInvokeDynamicRef: CstCallSiteRef = (CstCallSiteRef*)cst;
                let proto: Prototype = cstInvokeDynamicRef.getPrototype();
                return Rops::opInvokeCustom(&proto);
            }        }
        throw RuntimeException::new("unknown opcode "+RegOps::opName(opcode));
    }
    pub fn opMove(type: &TypeBearer) -> Rop    {
        match type.getBasicFrameType(){Type::BT_INT =>             return Rops::MOVE_INT;Type::BT_LONG =>             return Rops::MOVE_LONG;Type::BT_FLOAT =>             return Rops::MOVE_FLOAT;Type::BT_DOUBLE =>             return Rops::MOVE_DOUBLE;Type::BT_OBJECT =>             return Rops::MOVE_OBJECT;Type::BT_ADDR =>             return Rops::MOVE_RETURN_ADDRESS;        }
        return Rops::throwBadType(&type);
    }
    pub fn opMoveParam(type: &TypeBearer) -> Rop    {
        match type.getBasicFrameType(){Type::BT_INT =>             return Rops::MOVE_PARAM_INT;Type::BT_LONG =>             return Rops::MOVE_PARAM_LONG;Type::BT_FLOAT =>             return Rops::MOVE_PARAM_FLOAT;Type::BT_DOUBLE =>             return Rops::MOVE_PARAM_DOUBLE;Type::BT_OBJECT =>             return Rops::MOVE_PARAM_OBJECT;        }
        return Rops::throwBadType(&type);
    }
    pub fn opMoveException(type: &TypeBearer) -> Rop    {
        return Rop::new(RegOps::MOVE_EXCEPTION, type.getType(), &StdTypeList::EMPTY, (String*)None);
    }
    pub fn opMoveResult(type: &TypeBearer) -> Rop    {
        return Rop::new(RegOps::MOVE_RESULT, type.getType(), &StdTypeList::EMPTY, (String*)None);
    }
    pub fn opMoveResultPseudo(type: &TypeBearer) -> Rop    {
        return Rop::new(RegOps::MOVE_RESULT_PSEUDO, type.getType(), &StdTypeList::EMPTY, (String*)None);
    }
    pub fn opConst(type: &TypeBearer) -> Rop    {
        if type.getType()==Type::KNOWN_NULL        {
            return Rops::CONST_OBJECT_NOTHROW;
        }        
        match type.getBasicFrameType(){Type::BT_INT =>             return Rops::CONST_INT;Type::BT_LONG =>             return Rops::CONST_LONG;Type::BT_FLOAT =>             return Rops::CONST_FLOAT;Type::BT_DOUBLE =>             return Rops::CONST_DOUBLE;Type::BT_OBJECT =>             return Rops::CONST_OBJECT;        }
        return Rops::throwBadType(&type);
    }
    pub fn opIfEq(types: &TypeList) -> Rop    {
        return Rops::pickIf(&types, &Rops::IF_EQZ_INT, &Rops::IF_EQZ_OBJECT, &Rops::IF_EQ_INT, &Rops::IF_EQ_OBJECT);
    }
    pub fn opIfNe(types: &TypeList) -> Rop    {
        return Rops::pickIf(&types, &Rops::IF_NEZ_INT, &Rops::IF_NEZ_OBJECT, &Rops::IF_NE_INT, &Rops::IF_NE_OBJECT);
    }
    pub fn opIfLt(types: &TypeList) -> Rop    {
        return Rops::pickIf(&types, &Rops::IF_LTZ_INT, None, &Rops::IF_LT_INT, None);
    }
    pub fn opIfGe(types: &TypeList) -> Rop    {
        return Rops::pickIf(&types, &Rops::IF_GEZ_INT, None, &Rops::IF_GE_INT, None);
    }
    pub fn opIfGt(types: &TypeList) -> Rop    {
        return Rops::pickIf(&types, &Rops::IF_GTZ_INT, None, &Rops::IF_GT_INT, None);
    }
    pub fn opIfLe(types: &TypeList) -> Rop    {
        return Rops::pickIf(&types, &Rops::IF_LEZ_INT, None, &Rops::IF_LE_INT, None);
    }
    pub fn pickIf(types: &TypeList, intZ: &Rop, objZ: &Rop, intInt: &Rop, objObj: &Rop) -> Rop    {
        match types.size(){1 =>             {
                match types.getType(0).getBasicFrameType(){Type::BT_INT =>                     {
                        return intZ;
                    }Type::BT_OBJECT =>                     {
                        if objZ!=None                        {
                            return objZ;
                        }                        
                    }                }
                break;
            }2 =>             {
                let bt: i32 = types.getType(0).getBasicFrameType();
                if bt==types.getType(1).getBasicFrameType()                {
                    match bt{Type::BT_INT =>                         {
                            return intInt;
                        }Type::BT_OBJECT =>                         {
                            if objObj!=None                            {
                                return objObj;
                            }                            
                        }                    }
                }                
                break;
            }        }
        return Rops::throwBadTypes(&types);
    }
    pub fn opAdd(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::ADD_CONST_INT, &Rops::ADD_CONST_LONG, &Rops::ADD_CONST_FLOAT, &Rops::ADD_CONST_DOUBLE, &Rops::ADD_INT, &Rops::ADD_LONG, &Rops::ADD_FLOAT, &Rops::ADD_DOUBLE);
    }
    pub fn opSub(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::SUB_CONST_INT, &Rops::SUB_CONST_LONG, &Rops::SUB_CONST_FLOAT, &Rops::SUB_CONST_DOUBLE, &Rops::SUB_INT, &Rops::SUB_LONG, &Rops::SUB_FLOAT, &Rops::SUB_DOUBLE);
    }
    pub fn opMul(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::MUL_CONST_INT, &Rops::MUL_CONST_LONG, &Rops::MUL_CONST_FLOAT, &Rops::MUL_CONST_DOUBLE, &Rops::MUL_INT, &Rops::MUL_LONG, &Rops::MUL_FLOAT, &Rops::MUL_DOUBLE);
    }
    pub fn opDiv(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::DIV_CONST_INT, &Rops::DIV_CONST_LONG, &Rops::DIV_CONST_FLOAT, &Rops::DIV_CONST_DOUBLE, &Rops::DIV_INT, &Rops::DIV_LONG, &Rops::DIV_FLOAT, &Rops::DIV_DOUBLE);
    }
    pub fn opRem(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::REM_CONST_INT, &Rops::REM_CONST_LONG, &Rops::REM_CONST_FLOAT, &Rops::REM_CONST_DOUBLE, &Rops::REM_INT, &Rops::REM_LONG, &Rops::REM_FLOAT, &Rops::REM_DOUBLE);
    }
    pub fn opAnd(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::AND_CONST_INT, &Rops::AND_CONST_LONG, None, None, &Rops::AND_INT, &Rops::AND_LONG, None, None);
    }
    pub fn opOr(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::OR_CONST_INT, &Rops::OR_CONST_LONG, None, None, &Rops::OR_INT, &Rops::OR_LONG, None, None);
    }
    pub fn opXor(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::XOR_CONST_INT, &Rops::XOR_CONST_LONG, None, None, &Rops::XOR_INT, &Rops::XOR_LONG, None, None);
    }
    pub fn opShl(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::SHL_CONST_INT, &Rops::SHL_CONST_LONG, None, None, &Rops::SHL_INT, &Rops::SHL_LONG, None, None);
    }
    pub fn opShr(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::SHR_CONST_INT, &Rops::SHR_CONST_LONG, None, None, &Rops::SHR_INT, &Rops::SHR_LONG, None, None);
    }
    pub fn opUshr(types: &TypeList) -> Rop    {
        return Rops::pickBinaryOp(&types, &Rops::USHR_CONST_INT, &Rops::USHR_CONST_LONG, None, None, &Rops::USHR_INT, &Rops::USHR_LONG, None, None);
    }
    pub fn pickBinaryOp(types: &TypeList, int1: &Rop, long1: &Rop, float1: &Rop, double1: &Rop, int2: &Rop, long2: &Rop, float2: &Rop, double2: &Rop) -> Rop    {
        let bt1: i32 = types.getType(0).getBasicFrameType();
        let result: Rop = None;
        match types.size(){1 =>             {
                match bt1{Type::BT_INT =>                     return int1;Type::BT_LONG =>                     return long1;Type::BT_FLOAT =>                     result=float1;                    break;Type::BT_DOUBLE =>                     result=double1;                    break;                }
                break;
            }2 =>             {
                match bt1{Type::BT_INT =>                     return int2;Type::BT_LONG =>                     return long2;Type::BT_FLOAT =>                     result=float2;                    break;Type::BT_DOUBLE =>                     result=double2;                    break;                }
                break;
            }        }
        if result==None        {
            return Rops::throwBadTypes(&types);
        }        
        return result;
    }
    pub fn opNeg(type: &TypeBearer) -> Rop    {
        match type.getBasicFrameType(){Type::BT_INT =>             return Rops::NEG_INT;Type::BT_LONG =>             return Rops::NEG_LONG;Type::BT_FLOAT =>             return Rops::NEG_FLOAT;Type::BT_DOUBLE =>             return Rops::NEG_DOUBLE;        }
        return Rops::throwBadType(&type);
    }
    pub fn opNot(type: &TypeBearer) -> Rop    {
        match type.getBasicFrameType(){Type::BT_INT =>             return Rops::NOT_INT;Type::BT_LONG =>             return Rops::NOT_LONG;        }
        return Rops::throwBadType(&type);
    }
    pub fn opCmpl(type: &TypeBearer) -> Rop    {
        match type.getBasicType(){Type::BT_LONG =>             return Rops::CMPL_LONG;Type::BT_FLOAT =>             return Rops::CMPL_FLOAT;Type::BT_DOUBLE =>             return Rops::CMPL_DOUBLE;        }
        return Rops::throwBadType(&type);
    }
    pub fn opCmpg(type: &TypeBearer) -> Rop    {
        match type.getBasicType(){Type::BT_FLOAT =>             return Rops::CMPG_FLOAT;Type::BT_DOUBLE =>             return Rops::CMPG_DOUBLE;        }
        return Rops::throwBadType(&type);
    }
    pub fn opConv(dest: &TypeBearer, source: &TypeBearer) -> Rop    {
        let dbt: i32 = dest.getBasicFrameType();
        match source.getBasicFrameType(){Type::BT_INT =>             {
                match dbt{Type::BT_LONG =>                     return Rops::CONV_I2L;Type::BT_FLOAT =>                     return Rops::CONV_I2F;Type::BT_DOUBLE =>                     return Rops::CONV_I2D;                _ => {}                break;            }
        }Type::BT_LONG =>         {
            match dbt{Type::BT_INT =>                 return Rops::CONV_L2I;Type::BT_FLOAT =>                 return Rops::CONV_L2F;Type::BT_DOUBLE =>                 return Rops::CONV_L2D;            _ => {}            break;        }
    }Type::BT_FLOAT =>     {
        match dbt{Type::BT_INT =>             return Rops::CONV_F2I;Type::BT_LONG =>             return Rops::CONV_F2L;Type::BT_DOUBLE =>             return Rops::CONV_F2D;        _ => {}        break;    }
}Type::BT_DOUBLE => {
    match dbt{Type::BT_INT =>         return Rops::CONV_D2I;Type::BT_LONG =>         return Rops::CONV_D2L;Type::BT_FLOAT =>         return Rops::CONV_D2F;    _ => {}    break;}
}}
return Rops::throwBadTypes(StdTypeList::make_Type_Type(dest.getType(), source.getType()));
}
pub fn opReturn(type: &TypeBearer) -> Rop{
match type.getBasicFrameType(){Type::BT_INT => return Rops::RETURN_INT;Type::BT_LONG => return Rops::RETURN_LONG;Type::BT_FLOAT => return Rops::RETURN_FLOAT;Type::BT_DOUBLE => return Rops::RETURN_DOUBLE;Type::BT_OBJECT => return Rops::RETURN_OBJECT;Type::BT_VOID => return Rops::RETURN_VOID;}
return Rops::throwBadType(&type);
}
pub fn opAget(type: &TypeBearer) -> Rop{
match type.getBasicType(){Type::BT_INT => return Rops::AGET_INT;Type::BT_LONG => return Rops::AGET_LONG;Type::BT_FLOAT => return Rops::AGET_FLOAT;Type::BT_DOUBLE => return Rops::AGET_DOUBLE;Type::BT_OBJECT => return Rops::AGET_OBJECT;Type::BT_BOOLEAN => return Rops::AGET_BOOLEAN;Type::BT_BYTE => return Rops::AGET_BYTE;Type::BT_CHAR => return Rops::AGET_CHAR;Type::BT_SHORT => return Rops::AGET_SHORT;}
return Rops::throwBadType(&type);
}
pub fn opAput(type: &TypeBearer) -> Rop{
match type.getBasicType(){Type::BT_INT => return Rops::APUT_INT;Type::BT_LONG => return Rops::APUT_LONG;Type::BT_FLOAT => return Rops::APUT_FLOAT;Type::BT_DOUBLE => return Rops::APUT_DOUBLE;Type::BT_OBJECT => return Rops::APUT_OBJECT;Type::BT_BOOLEAN => return Rops::APUT_BOOLEAN;Type::BT_BYTE => return Rops::APUT_BYTE;Type::BT_CHAR => return Rops::APUT_CHAR;Type::BT_SHORT => return Rops::APUT_SHORT;}
return Rops::throwBadType(&type);
}
pub fn opNewArray(arrayType: &TypeBearer) -> Rop{
let type: Type = arrayType.getType();
let elementType: Type = type_renamed.getComponentType();
match elementType.getBasicType(){Type::BT_INT => return Rops::NEW_ARRAY_INT;Type::BT_LONG => return Rops::NEW_ARRAY_LONG;Type::BT_FLOAT => return Rops::NEW_ARRAY_FLOAT;Type::BT_DOUBLE => return Rops::NEW_ARRAY_DOUBLE;Type::BT_BOOLEAN => return Rops::NEW_ARRAY_BOOLEAN;Type::BT_BYTE => return Rops::NEW_ARRAY_BYTE;Type::BT_CHAR => return Rops::NEW_ARRAY_CHAR;Type::BT_SHORT => return Rops::NEW_ARRAY_SHORT;Type::BT_OBJECT => {
return Rop::new(RegOps::NEW_ARRAY, &type_renamed, &StdTypeList::INT, &Exceptions::LIST_Error_NegativeArraySizeException, "new-array-object");
}}
return Rops::throwBadType(&type_renamed);
}
pub fn opFilledNewArray(arrayType: &TypeBearer, count: i32) -> Rop{
let type: Type = arrayType.getType();
let elementType: Type = type_renamed.getComponentType();
if elementType.isCategory2(){
return Rops::throwBadType(&arrayType);
}
if count<0{
throw IllegalArgumentException::new("count < 0");
}
let sourceTypes: StdTypeList = StdTypeList::new(count);
for(let i: i32 = 0;;i<counti += 1){
sourceTypes.set(i, &elementType);
}
return Rop::new(RegOps::FILLED_NEW_ARRAY, &sourceTypes, &Exceptions::LIST_Error);
}
pub fn opGetField(type: &TypeBearer) -> Rop{
match type.getBasicType(){Type::BT_INT => return Rops::GET_FIELD_INT;Type::BT_LONG => return Rops::GET_FIELD_LONG;Type::BT_FLOAT => return Rops::GET_FIELD_FLOAT;Type::BT_DOUBLE => return Rops::GET_FIELD_DOUBLE;Type::BT_OBJECT => return Rops::GET_FIELD_OBJECT;Type::BT_BOOLEAN => return Rops::GET_FIELD_BOOLEAN;Type::BT_BYTE => return Rops::GET_FIELD_BYTE;Type::BT_CHAR => return Rops::GET_FIELD_CHAR;Type::BT_SHORT => return Rops::GET_FIELD_SHORT;}
return Rops::throwBadType(&type);
}
pub fn opPutField(type: &TypeBearer) -> Rop{
match type.getBasicType(){Type::BT_INT => return Rops::PUT_FIELD_INT;Type::BT_LONG => return Rops::PUT_FIELD_LONG;Type::BT_FLOAT => return Rops::PUT_FIELD_FLOAT;Type::BT_DOUBLE => return Rops::PUT_FIELD_DOUBLE;Type::BT_OBJECT => return Rops::PUT_FIELD_OBJECT;Type::BT_BOOLEAN => return Rops::PUT_FIELD_BOOLEAN;Type::BT_BYTE => return Rops::PUT_FIELD_BYTE;Type::BT_CHAR => return Rops::PUT_FIELD_CHAR;Type::BT_SHORT => return Rops::PUT_FIELD_SHORT;}
return Rops::throwBadType(&type);
}
pub fn opGetStatic(type: &TypeBearer) -> Rop{
match type.getBasicType(){Type::BT_INT => return Rops::GET_STATIC_INT;Type::BT_LONG => return Rops::GET_STATIC_LONG;Type::BT_FLOAT => return Rops::GET_STATIC_FLOAT;Type::BT_DOUBLE => return Rops::GET_STATIC_DOUBLE;Type::BT_OBJECT => return Rops::GET_STATIC_OBJECT;Type::BT_BOOLEAN => return Rops::GET_STATIC_BOOLEAN;Type::BT_BYTE => return Rops::GET_STATIC_BYTE;Type::BT_CHAR => return Rops::GET_STATIC_CHAR;Type::BT_SHORT => return Rops::GET_STATIC_SHORT;}
return Rops::throwBadType(&type);
}
pub fn opPutStatic(type: &TypeBearer) -> Rop{
match type.getBasicType(){Type::BT_INT => return Rops::PUT_STATIC_INT;Type::BT_LONG => return Rops::PUT_STATIC_LONG;Type::BT_FLOAT => return Rops::PUT_STATIC_FLOAT;Type::BT_DOUBLE => return Rops::PUT_STATIC_DOUBLE;Type::BT_OBJECT => return Rops::PUT_STATIC_OBJECT;Type::BT_BOOLEAN => return Rops::PUT_STATIC_BOOLEAN;Type::BT_BYTE => return Rops::PUT_STATIC_BYTE;Type::BT_CHAR => return Rops::PUT_STATIC_CHAR;Type::BT_SHORT => return Rops::PUT_STATIC_SHORT;}
return Rops::throwBadType(&type);
}
pub fn opInvokeStatic(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_STATIC, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opInvokeVirtual(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_VIRTUAL, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opInvokeSuper(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_SUPER, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opInvokeDirect(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_DIRECT, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opInvokeInterface(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_INTERFACE, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opInvokePolymorphic(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_POLYMORPHIC, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opInvokeCustom(meth: &Prototype) -> Rop{
return Rop::new(RegOps::INVOKE_CUSTOM, meth.getParameterFrameTypes(), &StdTypeList::THROWABLE);
}
pub fn opMarkLocal(type: &TypeBearer) -> Rop{
match type.getBasicFrameType(){Type::BT_INT => return Rops::MARK_LOCAL_INT;Type::BT_LONG => return Rops::MARK_LOCAL_LONG;Type::BT_FLOAT => return Rops::MARK_LOCAL_FLOAT;Type::BT_DOUBLE => return Rops::MARK_LOCAL_DOUBLE;Type::BT_OBJECT => return Rops::MARK_LOCAL_OBJECT;}
return Rops::throwBadType(&type);
}
pub fn new(&self){
}
pub fn throwBadType(type: &TypeBearer) -> Rop{
throw IllegalArgumentException::new("bad type: "+type);
}
pub fn throwBadTypes(types: &TypeList) -> Rop{
throw IllegalArgumentException::new("bad types: "+types);
}
}
