use crate::helper;
use crate::com::android::dx::cf::code::Frame;
use crate::com::android::dx::rop::type::TypeBearer;
use crate::com::android::dx::rop::cst::CstType;
use crate::com::android::dx::cf::code::ByteOps;
use crate::com::android::dx::rop::cst::CstCallSiteRef;
use crate::com::android::dx::cf::code::ReturnAddress;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::util::Hex;

struct ValueAwareMachine{
}
impl ValueAwareMachine{
    pub fn new(&self, prototype: &Prototype)    {
        super(prototype);

    }
    pub fn run(&self, frame: &Frame, offset: i32, opcode: i32)    {
        match opcode{ByteOps::NOP => ByteOps::IASTORE => ByteOps::POP => ByteOps::POP2 => ByteOps::IFEQ => ByteOps::IFNE => ByteOps::IFLT => ByteOps::IFGE => ByteOps::IFGT => ByteOps::IFLE => ByteOps::IF_ICMPEQ => ByteOps::IF_ICMPNE => ByteOps::IF_ICMPLT => ByteOps::IF_ICMPGE => ByteOps::IF_ICMPGT => ByteOps::IF_ICMPLE => ByteOps::IF_ACMPEQ => ByteOps::IF_ACMPNE => ByteOps::GOTO => ByteOps::RET => ByteOps::LOOKUPSWITCH => ByteOps::IRETURN => ByteOps::RETURN => ByteOps::PUTSTATIC => ByteOps::PUTFIELD => ByteOps::ATHROW => ByteOps::MONITORENTER => ByteOps::MONITOREXIT => ByteOps::IFNULL => ByteOps::IFNONNULL =>             {
                clearResult();
                break;
            }ByteOps::LDC => ByteOps::LDC2_W =>             {
                setResult((TypeBearer*)getAuxCst());
                break;
            }ByteOps::ILOAD => ByteOps::ISTORE =>             {
                setResult(arg(0));
                break;
            }ByteOps::IALOAD => ByteOps::IADD => ByteOps::ISUB => ByteOps::IMUL => ByteOps::IDIV => ByteOps::IREM => ByteOps::INEG => ByteOps::ISHL => ByteOps::ISHR => ByteOps::IUSHR => ByteOps::IAND => ByteOps::IOR => ByteOps::IXOR => ByteOps::IINC => ByteOps::I2L => ByteOps::I2F => ByteOps::I2D => ByteOps::L2I => ByteOps::L2F => ByteOps::L2D => ByteOps::F2I => ByteOps::F2L => ByteOps::F2D => ByteOps::D2I => ByteOps::D2L => ByteOps::D2F => ByteOps::I2B => ByteOps::I2C => ByteOps::I2S => ByteOps::LCMP => ByteOps::FCMPL => ByteOps::FCMPG => ByteOps::DCMPL => ByteOps::DCMPG => ByteOps::ARRAYLENGTH =>             {
                setResult(getAuxType());
                break;
            }ByteOps::DUP => ByteOps::DUP_X1 => ByteOps::DUP_X2 => ByteOps::DUP2 => ByteOps::DUP2_X1 => ByteOps::DUP2_X2 => ByteOps::SWAP =>             {
                clearResult();
                for(                let pattern: i32 = getAuxInt();;pattern!=0pattern>>=4)                {
                    let which: i32 = (pattern&0x0f)-1;
                    addResult(arg(which));
                }
                break;
            }ByteOps::JSR =>             {
                setResult(ReturnAddress::new(getAuxTarget()));
                break;
            }ByteOps::GETSTATIC => ByteOps::GETFIELD => ByteOps::INVOKEVIRTUAL => ByteOps::INVOKESTATIC => ByteOps::INVOKEINTERFACE =>             {
                let type: Type = ((TypeBearer*)getAuxCst()).getType();
                if type_renamed==Type::VOID                {
                    clearResult();
                }                else                 {
                    setResult(&type_renamed);
                }
                break;
            }ByteOps::INVOKESPECIAL =>             {
                let thisType: Type = arg(0).getType();
                if thisType.isUninitialized()                {
                    frame.makeInitialized(&thisType);
                }                
                let type: Type = ((TypeBearer*)getAuxCst()).getType();
                if type_renamed==Type::VOID                {
                    clearResult();
                }                else                 {
                    setResult(&type_renamed);
                }
                break;
            }ByteOps::INVOKEDYNAMIC =>             {
                let type: Type = ((CstCallSiteRef*)getAuxCst()).getReturnType();
                if type_renamed==Type::VOID                {
                    clearResult();
                }                else                 {
                    setResult(&type_renamed);
                }
                break;
            }ByteOps::NEW =>             {
                let type: Type = ((CstType*)getAuxCst()).getClassType();
                setResult(type_renamed.asUninitialized(offset));
                break;
            }ByteOps::NEWARRAY => ByteOps::CHECKCAST => ByteOps::MULTIANEWARRAY =>             {
                let type: Type = ((CstType*)getAuxCst()).getClassType();
                setResult(&type_renamed);
                break;
            }ByteOps::ANEWARRAY =>             {
                let type: Type = ((CstType*)getAuxCst()).getClassType();
                setResult(type_renamed.getArrayType());
                break;
            }ByteOps::INSTANCEOF =>             {
                setResult(&Type::INT);
                break;
            }        _ => {}        {
            throw RuntimeException::new("shouldn't happen: "+Hex::u1(opcode));
        }    }
    storeResults(&frame);
}
}
