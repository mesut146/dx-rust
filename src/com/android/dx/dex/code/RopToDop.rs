use crate::helper;
use crate::com::android::dx::rop::code::Rops;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::code::ThrowingCstInsn;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::cst::CstProtoRef;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::dex::code::Dops;
use crate::com::android::dx::rop::cst::CstFieldRef;

let static MAP: Option<HashMap<Rop,Dop>> = Node;
struct RopToDop{
}
impl RopToDop{
    pub fn new(&self)    {
    }
    pub fn dopFor(insn: &Insn) -> Dop    {
        let rop: Rop = insn.getOpcode();
        let result: Dop = RopToDop::MAP.get(&rop);
        if result!=None        {
            return result;
        }        
        match rop.getOpcode(){RegOps::MOVE_EXCEPTION =>             return Dops::MOVE_EXCEPTION;RegOps::INVOKE_STATIC =>             return Dops::INVOKE_STATIC;RegOps::INVOKE_VIRTUAL =>             return Dops::INVOKE_VIRTUAL;RegOps::INVOKE_SUPER =>             return Dops::INVOKE_SUPER;RegOps::INVOKE_DIRECT =>             return Dops::INVOKE_DIRECT;RegOps::INVOKE_INTERFACE =>             return Dops::INVOKE_INTERFACE;RegOps::INVOKE_POLYMORPHIC =>             return Dops::INVOKE_POLYMORPHIC;RegOps::INVOKE_CUSTOM =>             return Dops::INVOKE_CUSTOM;RegOps::NEW_ARRAY =>             return Dops::NEW_ARRAY;RegOps::FILLED_NEW_ARRAY =>             return Dops::FILLED_NEW_ARRAY;RegOps::FILL_ARRAY_DATA =>             return Dops::FILL_ARRAY_DATA;RegOps::MOVE_RESULT =>             {
                let resultReg: RegisterSpec = insn.getResult();
                if resultReg==None                {
                    return Dops::NOP;
                }                else                 {
                    match resultReg.getBasicType(){Type::BT_INT => Type::BT_FLOAT => Type::BT_BOOLEAN => Type::BT_BYTE => Type::BT_CHAR => Type::BT_SHORT =>                         return Dops::MOVE_RESULT;Type::BT_LONG => Type::BT_DOUBLE =>                         return Dops::MOVE_RESULT_WIDE;Type::BT_OBJECT =>                         return Dops::MOVE_RESULT_OBJECT;                    _ => {}                    {
                        throw RuntimeException::new("Unexpected basic type");
                    }                }
            }
        }RegOps::GET_FIELD =>         {
            let ref: CstFieldRef = (CstFieldRef*)((ThrowingCstInsn*)insn).getConstant();
            let basicType: i32 = ref_renamed.getBasicType();
            match basicType{Type::BT_BOOLEAN =>                 return Dops::IGET_BOOLEAN;Type::BT_BYTE =>                 return Dops::IGET_BYTE;Type::BT_CHAR =>                 return Dops::IGET_CHAR;Type::BT_SHORT =>                 return Dops::IGET_SHORT;Type::BT_INT =>                 return Dops::IGET;            }
            break;
        }RegOps::PUT_FIELD =>         {
            let ref: CstFieldRef = (CstFieldRef*)((ThrowingCstInsn*)insn).getConstant();
            let basicType: i32 = ref_renamed.getBasicType();
            match basicType{Type::BT_BOOLEAN =>                 return Dops::IPUT_BOOLEAN;Type::BT_BYTE =>                 return Dops::IPUT_BYTE;Type::BT_CHAR =>                 return Dops::IPUT_CHAR;Type::BT_SHORT =>                 return Dops::IPUT_SHORT;Type::BT_INT =>                 return Dops::IPUT;            }
            break;
        }RegOps::GET_STATIC =>         {
            let ref: CstFieldRef = (CstFieldRef*)((ThrowingCstInsn*)insn).getConstant();
            let basicType: i32 = ref_renamed.getBasicType();
            match basicType{Type::BT_BOOLEAN =>                 return Dops::SGET_BOOLEAN;Type::BT_BYTE =>                 return Dops::SGET_BYTE;Type::BT_CHAR =>                 return Dops::SGET_CHAR;Type::BT_SHORT =>                 return Dops::SGET_SHORT;Type::BT_INT =>                 return Dops::SGET;            }
            break;
        }RegOps::PUT_STATIC =>         {
            let ref: CstFieldRef = (CstFieldRef*)((ThrowingCstInsn*)insn).getConstant();
            let basicType: i32 = ref_renamed.getBasicType();
            match basicType{Type::BT_BOOLEAN =>                 return Dops::SPUT_BOOLEAN;Type::BT_BYTE =>                 return Dops::SPUT_BYTE;Type::BT_CHAR =>                 return Dops::SPUT_CHAR;Type::BT_SHORT =>                 return Dops::SPUT_SHORT;Type::BT_INT =>                 return Dops::SPUT;            }
            break;
        }RegOps::CONST =>         {
            let cst: Constant = ((ThrowingCstInsn*)insn).getConstant();
            if //cst instanceof CstType            {
                return Dops::CONST_CLASS;
            }            else             if //cst instanceof CstString            {
                return Dops::CONST_STRING;
            }            else             if //cst instanceof CstMethodHandle            {
                return Dops::CONST_METHOD_HANDLE;
            }            else             if //cst instanceof CstProtoRef            {
                return Dops::CONST_METHOD_TYPE;
            }            else             {
                throw RuntimeException::new("Unexpected constant type");
            }
        }    }
    throw RuntimeException::new("unknown rop: "+rop);
}
}
