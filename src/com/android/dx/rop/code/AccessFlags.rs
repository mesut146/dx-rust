use crate::helper;
use crate::com::android::dx::util::Hex;

struct AccessFlags{
}
impl AccessFlags{
    pub const ACC_PUBLIC: i32 = 0x0001;
    pub const ACC_PRIVATE: i32 = 0x0002;
    pub const ACC_PROTECTED: i32 = 0x0004;
    pub const ACC_STATIC: i32 = 0x0008;
    pub const ACC_FINAL: i32 = 0x0010;
    pub const ACC_SYNCHRONIZED: i32 = 0x0020;
    pub const ACC_SUPER: i32 = 0x0020;
    pub const ACC_VOLATILE: i32 = 0x0040;
    pub const ACC_BRIDGE: i32 = 0x0040;
    pub const ACC_TRANSIENT: i32 = 0x0080;
    pub const ACC_VARARGS: i32 = 0x0080;
    pub const ACC_NATIVE: i32 = 0x0100;
    pub const ACC_INTERFACE: i32 = 0x0200;
    pub const ACC_ABSTRACT: i32 = 0x0400;
    pub const ACC_STRICT: i32 = 0x0800;
    pub const ACC_SYNTHETIC: i32 = 0x1000;
    pub const ACC_ANNOTATION: i32 = 0x2000;
    pub const ACC_ENUM: i32 = 0x4000;
    pub const ACC_CONSTRUCTOR: i32 = 0x10000;
    pub const ACC_DECLARED_SYNCHRONIZED: i32 = 0x20000;
    pub const CLASS_FLAGS: i32 = AccessFlags::ACC_PUBLIC|AccessFlags::ACC_FINAL|AccessFlags::ACC_SUPER|AccessFlags::ACC_INTERFACE|AccessFlags::ACC_ABSTRACT|AccessFlags::ACC_SYNTHETIC|AccessFlags::ACC_ANNOTATION|AccessFlags::ACC_ENUM;
    pub const INNER_CLASS_FLAGS: i32 = AccessFlags::ACC_PUBLIC|AccessFlags::ACC_PRIVATE|AccessFlags::ACC_PROTECTED|AccessFlags::ACC_STATIC|AccessFlags::ACC_FINAL|AccessFlags::ACC_INTERFACE|AccessFlags::ACC_ABSTRACT|AccessFlags::ACC_SYNTHETIC|AccessFlags::ACC_ANNOTATION|AccessFlags::ACC_ENUM;
    pub const FIELD_FLAGS: i32 = AccessFlags::ACC_PUBLIC|AccessFlags::ACC_PRIVATE|AccessFlags::ACC_PROTECTED|AccessFlags::ACC_STATIC|AccessFlags::ACC_FINAL|AccessFlags::ACC_VOLATILE|AccessFlags::ACC_TRANSIENT|AccessFlags::ACC_SYNTHETIC|AccessFlags::ACC_ENUM;
    pub const METHOD_FLAGS: i32 = AccessFlags::ACC_PUBLIC|AccessFlags::ACC_PRIVATE|AccessFlags::ACC_PROTECTED|AccessFlags::ACC_STATIC|AccessFlags::ACC_FINAL|AccessFlags::ACC_SYNCHRONIZED|AccessFlags::ACC_BRIDGE|AccessFlags::ACC_VARARGS|AccessFlags::ACC_NATIVE|AccessFlags::ACC_ABSTRACT|AccessFlags::ACC_STRICT|AccessFlags::ACC_SYNTHETIC|AccessFlags::ACC_CONSTRUCTOR|AccessFlags::ACC_DECLARED_SYNCHRONIZED;
    pub const CONV_CLASS: i32 = 1;
    pub const CONV_FIELD: i32 = 2;
    pub const CONV_METHOD: i32 = 3;
    pub fn new(&self)    {
    }
    pub fn classString(flags: i32) -> String    {
        return AccessFlags::humanHelper(flags, AccessFlags::CLASS_FLAGS, AccessFlags::CONV_CLASS);
    }
    pub fn innerClassString(flags: i32) -> String    {
        return AccessFlags::humanHelper(flags, AccessFlags::INNER_CLASS_FLAGS, AccessFlags::CONV_CLASS);
    }
    pub fn fieldString(flags: i32) -> String    {
        return AccessFlags::humanHelper(flags, AccessFlags::FIELD_FLAGS, AccessFlags::CONV_FIELD);
    }
    pub fn methodString(flags: i32) -> String    {
        return AccessFlags::humanHelper(flags, AccessFlags::METHOD_FLAGS, AccessFlags::CONV_METHOD);
    }
    pub fn isPublic(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_PUBLIC)!=0;
    }
    pub fn isProtected(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_PROTECTED)!=0;
    }
    pub fn isPrivate(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_PRIVATE)!=0;
    }
    pub fn isStatic(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_STATIC)!=0;
    }
    pub fn isConstructor(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_CONSTRUCTOR)!=0;
    }
    pub fn isInterface(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_INTERFACE)!=0;
    }
    pub fn isSynchronized(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_SYNCHRONIZED)!=0;
    }
    pub fn isAbstract(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_ABSTRACT)!=0;
    }
    pub fn isNative(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_NATIVE)!=0;
    }
    pub fn isAnnotation(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_ANNOTATION)!=0;
    }
    pub fn isDeclaredSynchronized(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_DECLARED_SYNCHRONIZED)!=0;
    }
    pub fn isEnum(flags: i32) -> boolean    {
        return (flags&AccessFlags::ACC_ENUM)!=0;
    }
    pub fn humanHelper(flags: i32, mask: i32, what: i32) -> String    {
        let sb: StringBuilder = StringBuilder::new(80);
        let extra: i32 = flags&~mask;
        flags&=mask;
        if (flags&AccessFlags::ACC_PUBLIC)!=0        {
            sb.append_String("|public");
        }        
        if (flags&AccessFlags::ACC_PRIVATE)!=0        {
            sb.append_String("|private");
        }        
        if (flags&AccessFlags::ACC_PROTECTED)!=0        {
            sb.append_String("|protected");
        }        
        if (flags&AccessFlags::ACC_STATIC)!=0        {
            sb.append_String("|static");
        }        
        if (flags&AccessFlags::ACC_FINAL)!=0        {
            sb.append_String("|final");
        }        
        if (flags&AccessFlags::ACC_SYNCHRONIZED)!=0        {
            if what==AccessFlags::CONV_CLASS            {
                sb.append_String("|super");
            }            else             {
                sb.append_String("|synchronized");
            }
        }        
        if (flags&AccessFlags::ACC_VOLATILE)!=0        {
            if what==AccessFlags::CONV_METHOD            {
                sb.append_String("|bridge");
            }            else             {
                sb.append_String("|volatile");
            }
        }        
        if (flags&AccessFlags::ACC_TRANSIENT)!=0        {
            if what==AccessFlags::CONV_METHOD            {
                sb.append_String("|varargs");
            }            else             {
                sb.append_String("|transient");
            }
        }        
        if (flags&AccessFlags::ACC_NATIVE)!=0        {
            sb.append_String("|native");
        }        
        if (flags&AccessFlags::ACC_INTERFACE)!=0        {
            sb.append_String("|interface");
        }        
        if (flags&AccessFlags::ACC_ABSTRACT)!=0        {
            sb.append_String("|abstract");
        }        
        if (flags&AccessFlags::ACC_STRICT)!=0        {
            sb.append_String("|strictfp");
        }        
        if (flags&AccessFlags::ACC_SYNTHETIC)!=0        {
            sb.append_String("|synthetic");
        }        
        if (flags&AccessFlags::ACC_ANNOTATION)!=0        {
            sb.append_String("|annotation");
        }        
        if (flags&AccessFlags::ACC_ENUM)!=0        {
            sb.append_String("|enum");
        }        
        if (flags&AccessFlags::ACC_CONSTRUCTOR)!=0        {
            sb.append_String("|constructor");
        }        
        if (flags&AccessFlags::ACC_DECLARED_SYNCHRONIZED)!=0        {
            sb.append_String("|declared_synchronized");
        }        
        if (extra!=0)||(sb.length()==0)        {
            sb.append_char('|');
            sb.append_String(Hex::u2(extra));
        }        
        return sb.substring_int(1);
    }
}
