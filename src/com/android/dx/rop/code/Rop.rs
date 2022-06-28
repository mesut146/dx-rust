use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::type::TypeList;
use crate::com::android::dx::rop::type::Type;
use crate::com::android::dx::rop::code::Rop;
use crate::com::android::dx::util::Hex;

struct Rop{
    pub opcode: i32,
    pub result: Type,
    pub sources: TypeList,
    pub exceptions: TypeList,
    pub branchingness: i32,
    pub isCallLike: boolean,
    pub nickname: String,
}
impl Rop{
    pub const BRANCH_MIN: i32 = 1;
    pub const BRANCH_NONE: i32 = 1;
    pub const BRANCH_RETURN: i32 = 2;
    pub const BRANCH_GOTO: i32 = 3;
    pub const BRANCH_IF: i32 = 4;
    pub const BRANCH_SWITCH: i32 = 5;
    pub const BRANCH_THROW: i32 = 6;
    pub const BRANCH_MAX: i32 = 6;
    pub fn new(&self, opcode: i32, result: &Type, sources: &TypeList, exceptions: &TypeList, branchingness: i32, isCallLike: boolean, nickname: &String)    {
        if result==None        {
            throw NullPointerException::new("result == null");
        }        
        if sources==None        {
            throw NullPointerException::new("sources == null");
        }        
        if exceptions==None        {
            throw NullPointerException::new("exceptions == null");
        }        
        if (branchingness<Rop::BRANCH_MIN)||(branchingness>Rop::BRANCH_MAX)        {
            throw IllegalArgumentException::new("invalid branchingness: "+branchingness);
        }        
        if (exceptions.size()!=0)&&(branchingness!=Rop::BRANCH_THROW)        {
            throw IllegalArgumentException::new("exceptions / branchingness "+"mismatch");
        }        
        self->opcode=opcode;
        self->result=result;
        self->sources=sources;
        self->exceptions=exceptions;
        self->branchingness=branchingness;
        self->isCallLike=isCallLike;
        self->nickname=nickname;
    }
    pub fn new(&self, opcode: i32, result: &Type, sources: &TypeList, exceptions: &TypeList, branchingness: i32, nickname: &String)    {
        this(opcode,result,sources,exceptions,branchingness,false,nickname);

    }
    pub fn new(&self, opcode: i32, result: &Type, sources: &TypeList, branchingness: i32, nickname: &String)    {
        this(opcode,result,sources,StdTypeList.EMPTY,branchingness,false,nickname);

    }
    pub fn new(&self, opcode: i32, result: &Type, sources: &TypeList, nickname: &String)    {
        this(opcode,result,sources,StdTypeList.EMPTY,Rop.BRANCH_NONE,false,nickname);

    }
    pub fn new(&self, opcode: i32, result: &Type, sources: &TypeList, exceptions: &TypeList, nickname: &String)    {
        this(opcode,result,sources,exceptions,Rop.BRANCH_THROW,false,nickname);

    }
    pub fn new(&self, opcode: i32, sources: &TypeList, exceptions: &TypeList)    {
        this(opcode,Type.VOID,sources,exceptions,Rop.BRANCH_THROW,true,null);

    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if self==other        {
            return true;
        }        
        if !(//other instanceof Rop)        {
            return false;
        }        
        let rop: Rop = (Rop*)other;
        return (self.opcode==)&&(self.branchingness==)&&(self.result==)&&self.sources.equals(&)&&self.exceptions.equals(&);
    }
    pub fn hashCode(&self) -> i32    {
        let h: i32 = (self.opcode*31)+self.branchingness;
        h=(h*31)+self.result.hashCode();
        h=(h*31)+self.sources.hashCode();
        h=(h*31)+self.exceptions.hashCode();
        return h;
    }
    pub fn toString(&self) -> String    {
        let sb: StringBuilder = StringBuilder::new(40);
        sb.append_String("Rop{");
        sb.append_String(RegOps::opName(self.opcode));
        if self.result!=Type::VOID        {
            sb.append_String(" ");
            sb.append_Object(&self.result);
        }        else         {
            sb.append_String(" .");
        }
        sb.append_String(" <-");
        let sz: i32 = self.sources.size();
        if sz==0        {
            sb.append_String(" .");
        }        else         {
            for(            let i: i32 = 0;;i<szi += 1)            {
                sb.append_char(' ');
                sb.append_Object(self.sources.getType(i));
            }
        }
        if self.isCallLike        {
            sb.append_String(" call");
        }        
        sz=self.exceptions.size();
        if sz!=0        {
            sb.append_String(" throws");
            for(            let i: i32 = 0;;i<szi += 1)            {
                sb.append_char(' ');
                let one: Type = self.exceptions.getType(i);
                if one==Type::THROWABLE                {
                    sb.append_String("<any>");
                }                else                 {
                    sb.append_Object(self.exceptions.getType(i));
                }
            }
        }        else         {
            match self.branchingness{Rop::BRANCH_NONE =>                 sb.append_String(" flows");                break;Rop::BRANCH_RETURN =>                 sb.append_String(" returns");                break;Rop::BRANCH_GOTO =>                 sb.append_String(" gotos");                break;Rop::BRANCH_IF =>                 sb.append_String(" ifs");                break;Rop::BRANCH_SWITCH =>                 sb.append_String(" switches");                break;            _ => {}            sb.append_String(" "+Hex::u1(self.branchingness));            break;        }
    }
    sb.append_char('}');
    return sb.toString();
}
pub fn getOpcode(&self) -> i32{
    return self.opcode;
}
pub fn getResult(&self) -> Type{
    return self.result;
}
pub fn getSources(&self) -> TypeList{
    return self.sources;
}
pub fn getExceptions(&self) -> TypeList{
    return self.exceptions;
}
pub fn getBranchingness(&self) -> i32{
    return self.branchingness;
}
pub fn isCallLike(&self) -> boolean{
    return self.isCallLike;
}
pub fn isCommutative(&self) -> boolean{
    match self.opcode{RegOps::AND => RegOps::OR => RegOps::XOR => RegOps::ADD => RegOps::MUL =>         return true;    _ => {}    return false;}
}
pub fn getNickname(&self) -> String{
if self.nickname!=None{
    return self.nickname;
}
return toString();
}
pub fn canThrow(&self) -> boolean{
return (self.exceptions.size()!=0);
}
}
