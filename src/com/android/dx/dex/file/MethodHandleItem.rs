use crate::helper;
use crate::com::android::dx::rop::cst::Constant;
use crate::com::android::dx::dex::file::MethodIdsSection;
use crate::com::android::dx::rop::cst::CstBaseMethodRef;
use crate::com::android::dx::util::AnnotatedOutput;
use crate::com::android::dx::util::Hex;
use crate::com::android::dx::rop::cst::CstInterfaceMethodRef;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::FieldIdsSection;
use crate::com::android::dx::dex::file::MethodHandlesSection;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::rop::cst::CstMethodHandle;
use crate::com::android::dx::rop::cst::CstFieldRef;

struct MethodHandleItem{
    pub ITEM_SIZE: i32,
    pub methodHandle: CstMethodHandle,
}
impl MethodHandleItem{
    pub fn new(&self, methodHandle: &CstMethodHandle)    {
        self->methodHandle=methodHandle;
    }
    pub fn itemType(&self) -> ItemType    {
        return ItemType::TYPE_METHOD_HANDLE_ITEM;
    }
    pub fn writeSize(&self) -> i32    {
        return self.ITEM_SIZE;
    }
    pub fn addContents(&self, file: &DexFile)    {
        let methodHandles: MethodHandlesSection = file.getMethodHandles();
        methodHandles.intern(&self.methodHandle);
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        let targetIndex: i32 = getTargetIndex(&file);
        let mhType: i32 = self.methodHandle.getMethodHandleType();
        if out.annotates()        {
            out.annotate_int_String(0, indexString()+' '+self.methodHandle.toString());
            let typeComment: String = " // "+CstMethodHandle::getMethodHandleTypeName(mhType);
            out.annotate_int_String(2, "type:     "+Hex::u2(mhType)+typeComment);
            out.annotate_int_String(2, "reserved: "+Hex::u2(0));
            let targetComment: String = " // "+self.methodHandle.getRef().toString();
            if self.methodHandle.isAccessor()            {
                out.annotate_int_String(2, "fieldId:  "+Hex::u2(targetIndex)+targetComment);
            }            else             {
                out.annotate_int_String(2, "methodId: "+Hex::u2(targetIndex)+targetComment);
            }
            out.annotate_int_String(2, "reserved: "+Hex::u2(0));
        }        
        out.writeShort(mhType);
        out.writeShort(0);
        out.writeShort(getTargetIndex(&file));
        out.writeShort(0);
    }
    pub fn getTargetIndex(&self, file: &DexFile) -> i32    {
        let ref: Constant = self.methodHandle.getRef();
        if self.methodHandle.isAccessor()        {
            let fieldIds: FieldIdsSection = file.getFieldIds();
            return fieldIds.indexOf((CstFieldRef*)ref_renamed);
        }        else         if self.methodHandle.isInvocation()        {
            if //ref instanceof CstInterfaceMethodRef            {
                ref_renamed=((CstInterfaceMethodRef*)ref_renamed).toMethodRef();
            }            
            let methodIds: MethodIdsSection = file.getMethodIds();
            return methodIds.indexOf((CstBaseMethodRef*)ref_renamed);
        }        else         {
            throw IllegalStateException::new("Unhandled invocation type");
        }
    }
}
