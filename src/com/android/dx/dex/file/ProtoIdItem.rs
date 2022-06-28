use crate::helper;
use crate::com::android::dx::dex::file::StringIdsSection;
use crate::com::android::dx::rop::cst::CstString;
use crate::com::android::dx::rop::type::Prototype;
use crate::com::android::dex::SizeOf;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::dex::file::MixedItemSection;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::rop::type::StdTypeList;
use crate::com::android::dx::dex::file::TypeIdsSection;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::dex::file::OffsettedItem;
use crate::com::android::dx::dex::file::TypeListItem;
use crate::com::android::dx::rop::type::Type;

struct ProtoIdItem{
    pub prototype: Prototype,
    pub shortForm: CstString,
    pub parameterTypes: TypeListItem,
}
impl ProtoIdItem{
    pub fn new(&self, prototype: &Prototype)    {
        if prototype==None        {
            throw NullPointerException::new("prototype == null");
        }        
        self->prototype=prototype;
        self->shortForm=ProtoIdItem::makeShortForm(&prototype);
        let parameters: StdTypeList = prototype.getParameterTypes();
        self->parameterTypes=if (parameters.size()==0) { None } else { TypeListItem::new(&parameters) };
            }
            pub fn makeShortForm(prototype: &Prototype) -> CstString            {
                let parameters: StdTypeList = prototype.getParameterTypes();
                let size: i32 = parameters.size();
                let sb: StringBuilder = StringBuilder::new(size+1);
                sb.append_char(ProtoIdItem::shortFormCharFor(prototype.getReturnType()));
                for(                let i: i32 = 0;;i<sizei += 1)                {
                    sb.append_char(ProtoIdItem::shortFormCharFor(parameters.getType(i)));
                }
                return CstString::new(sb.toString());
            }
            pub fn shortFormCharFor(type: &Type) -> char            {
                let descriptorChar: char = type.getDescriptor().charAt(0);
                if descriptorChar=='['                {
                    return 'L';
                }                
                return descriptorChar;
            }
            pub fn itemType(&self) -> ItemType            {
                return ItemType::TYPE_PROTO_ID_ITEM;
            }
            pub fn writeSize(&self) -> i32            {
                return SizeOf::PROTO_ID_ITEM;
            }
            pub fn addContents(&self, file: &DexFile)            {
                let stringIds: StringIdsSection = file.getStringIds();
                let typeIds: TypeIdsSection = file.getTypeIds();
                let typeLists: MixedItemSection = file.getTypeLists();
                typeIds.intern_Type(self.prototype.getReturnType());
                stringIds.intern_CstString(&self.shortForm);
                if self.parameterTypes!=None                {
                    self.parameterTypes=typeLists.intern(&self.parameterTypes);
                }                
            }
            pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)            {
                let shortyIdx: i32 = file.getStringIds().indexOf(&self.shortForm);
                let returnIdx: i32 = file.getTypeIds().indexOf_Type(self.prototype.getReturnType());
                let paramsOff: i32 = OffsettedItem::getAbsoluteOffsetOr0(&self.parameterTypes);
                if out.annotates()                {
                    let sb: StringBuilder = StringBuilder::new();
                    sb.append_String(self.prototype.getReturnType().toHuman());
                    sb.append_String(" proto(");
                    let params: StdTypeList = self.prototype.getParameterTypes();
                    let size: i32 = params.size();
                    for(                    let i: i32 = 0;;i<sizei += 1)                    {
                        if i!=0                        {
                            sb.append_String(", ");
                        }                        
                        sb.append_String(params.getType(i).toHuman());
                    }
                    sb.append_String(")");
                    out.annotate_int_String(0, indexString()+' '+sb.toString());
                    out.annotate_int_String(4, "  shorty_idx:      "+Hex::u4(shortyIdx)+" // "+self.shortForm.toQuoted());
                    out.annotate_int_String(4, "  return_type_idx: "+Hex::u4(returnIdx)+" // "+self.prototype.getReturnType().toHuman());
                    out.annotate_int_String(4, "  parameters_off:  "+Hex::u4(paramsOff));
                }                
                out.writeInt(shortyIdx);
                out.writeInt(returnIdx);
                out.writeInt(paramsOff);
            }
}
