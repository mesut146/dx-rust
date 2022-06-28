use crate::helper;
use crate::com::android::dx::rop::code::RegOps;
use crate::com::android::dx::rop::code::RegisterSpec;
use crate::com::android::dx::rop::code::Insn;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::rop::code::RegisterSpecList;
use crate::com::android::dx::rop::code::SourcePosition;
use crate::com::android::dx::rop::code::Rop;

struct Insn{
    pub opcode: Rop,
    pub position: SourcePosition,
    pub result: RegisterSpec,
    pub sources: RegisterSpecList,
}
impl Insn{
    pub fn new(&self, opcode: &Rop, position: &SourcePosition, result: &RegisterSpec, sources: &RegisterSpecList)    {
        if opcode==None        {
            throw NullPointerException::new("opcode == null");
        }        
        if position==None        {
            throw NullPointerException::new("position == null");
        }        
        if sources==None        {
            throw NullPointerException::new("sources == null");
        }        
        self->opcode=opcode;
        self->position=position;
        self->result=result;
        self->sources=sources;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        return (self==other);
    }
    pub fn hashCode(&self) -> i32    {
        return System::identityHashCode(self);
    }
    pub fn toString(&self) -> String    {
        return toStringWithInline(getInlineString());
    }
    pub fn toHuman(&self) -> String    {
        return toHumanWithInline(getInlineString());
    }
    pub fn getInlineString(&self) -> String    {
        return None;
    }
    pub fn getOpcode(&self) -> Rop    {
        return self.opcode;
    }
    pub fn getPosition(&self) -> SourcePosition    {
        return self.position;
    }
    pub fn getResult(&self) -> RegisterSpec    {
        return self.result;
    }
    pub fn getLocalAssignment(&self) -> RegisterSpec    {
        let assignment: RegisterSpec;
        if self.opcode.getOpcode()==RegOps::MARK_LOCAL        {
            assignment=self.sources.get(0);
        }        else         {
            assignment=self.result;
        }
        if assignment==None        {
            return None;
        }        
        let localItem: LocalItem = assignment.getLocalItem();
        if localItem==None        {
            return None;
        }        
        return assignment;
    }
    pub fn getSources(&self) -> RegisterSpecList    {
        return self.sources;
    }
    pub fn canThrow(&self) -> boolean    {
        return self.opcode.canThrow();
    }
    pub fn getCatches(&self) -> TypeList;
    pub fn accept(&self, visitor: &Visitor);
    pub fn withAddedCatch(&self, type: &Type) -> Insn;
    pub fn withRegisterOffset(&self, delta: i32) -> Insn;
    pub fn withSourceLiteral(&self) -> Insn    {
        return self;
    }
    pub fn copy(&self) -> Insn    {
        return withRegisterOffset(0);
    }
    pub fn equalsHandleNulls(a: &Object, b: &Object) -> boolean    {
        return (a==b)||((a!=None)&&a.equals(&b));
    }
    pub fn contentEquals(&self, b: &Insn) -> boolean    {
        return self.opcode==b.getOpcode()&&self.position.equals(b.getPosition())&&(getClass()==b.getClass())&&Insn::equalsHandleNulls(&self.result, b.getResult())&&Insn::equalsHandleNulls(&self.sources, b.getSources())&&StdTypeList::equalContents(getCatches(), b.getCatches());
    }
    pub fn withNewRegisters(&self, result: &RegisterSpec, sources: &RegisterSpecList) -> Insn;
    pub fn toStringWithInline(&self, extra: &String) -> String    {
        let sb: StringBuilder = StringBuilder::new(80);
        sb.append_String("Insn{");
        sb.append_Object(&self.position);
        sb.append_char(' ');
        sb.append_Object(&self.opcode);
        if extra!=None        {
            sb.append_char(' ');
            sb.append_String(&extra);
        }        
        sb.append_String(" :: ");
        if self.result!=None        {
            sb.append_Object(&self.result);
            sb.append_String(" <- ");
        }        
        sb.append_Object(&self.sources);
        sb.append_char('}');
        return sb.toString();
    }
    pub fn toHumanWithInline(&self, extra: &String) -> String    {
        let sb: StringBuilder = StringBuilder::new(80);
        sb.append_Object(&self.position);
        sb.append_String(": ");
        sb.append_String(self.opcode.getNickname());
        if extra!=None        {
            sb.append_String("(");
            sb.append_String(&extra);
            sb.append_String(")");
        }        
        if self.result==None        {
            sb.append_String(" .");
        }        else         {
            sb.append_String(" ");
            sb.append_String(self.result.toHuman());
        }
        sb.append_String(" <-");
        let sz: i32 = self.sources.size();
        if sz==0        {
            sb.append_String(" .");
        }        else         {
            for(            let i: i32 = 0;;i<szi += 1)            {
                sb.append_String(" ");
                sb.append_String(self.sources.get(i).toHuman());
            }
        }
        return sb.toString();
    }
}
