use crate::helper;
use crate::com::android::dx::dex::file::ItemType;
use crate::com::android::dx::dex::file::Section;
use crate::com::android::dex::util::ExceptionWithContext;
use crate::com::android::dx::dex::file::OffsettedItem;
use crate::com::android::dx::util::AnnotatedOutput;

struct OffsettedItem{
    pub alignment: i32,
    pub writeSize: i32,
    pub addedTo: Section,
    pub offset: i32,
}
impl OffsettedItem{
    pub fn getAbsoluteOffsetOr0(item: &OffsettedItem) -> i32    {
        if item==None        {
            return 0;
        }        
        return item.getAbsoluteOffset();
    }
    pub fn new(&self, alignment: i32, writeSize: i32)    {
        Section::validateAlignment(alignment);
        if writeSize<-1        {
            throw IllegalArgumentException::new("writeSize < -1");
        }        
        self->alignment=alignment;
        self->writeSize=writeSize;
        self->addedTo=None;
        self->offset=-1;
    }
    pub fn equals(&self, other: &Object) -> boolean    {
        if self==other        {
            return true;
        }        
        let otherItem: OffsettedItem = (OffsettedItem*)other;
        let thisType: ItemType = itemType();
        let otherType: ItemType = otherItem.itemType();
        if thisType!=otherType        {
            return false;
        }        
        return (compareTo0(&otherItem)==0);
    }
    pub fn compareTo(&self, other: &OffsettedItem) -> i32    {
        if self==other        {
            return 0;
        }        
        let thisType: ItemType = itemType();
        let otherType: ItemType = other.itemType();
        if thisType!=otherType        {
            return thisType.compareTo(&otherType);
        }        
        return compareTo0(&other);
    }
    pub fn setWriteSize(&self, writeSize: i32)    {
        if writeSize<0        {
            throw IllegalArgumentException::new("writeSize < 0");
        }        
        if self->writeSize>=0        {
            throw UnsupportedOperationException::new("writeSize already set");
        }        
        self->writeSize=writeSize;
    }
    pub fn writeSize(&self) -> i32    {
        if self.writeSize<0        {
            throw UnsupportedOperationException::new("writeSize is unknown");
        }        
        return self.writeSize;
    }
    pub fn writeTo(&self, file: &DexFile, out: &AnnotatedOutput)    {
        out.alignTo(self.alignment);
        try        {
            if self.writeSize<0            {
                throw UnsupportedOperationException::new("writeSize is unknown");
            }            
            out.assertCursor(getAbsoluteOffset());
        }        catch(        let ex: RuntimeException)        {
            throw ExceptionWithContext::withContext(&ex, "...while writing "+self);
        }
        writeTo0(&file, &out);
    }
    pub fn getRelativeOffset(&self) -> i32    {
        if self.offset<0        {
            throw RuntimeException::new("offset not yet known");
        }        
        return self.offset;
    }
    pub fn getAbsoluteOffset(&self) -> i32    {
        if self.offset<0        {
            throw RuntimeException::new("offset not yet known");
        }        
        return self.addedTo.getAbsoluteOffset(self.offset);
    }
    pub fn place(&self, addedTo: &Section, offset: i32) -> i32    {
        if addedTo==None        {
            throw NullPointerException::new("addedTo == null");
        }        
        if offset<0        {
            throw IllegalArgumentException::new("offset < 0");
        }        
        if self->addedTo!=None        {
            throw RuntimeException::new("already written");
        }        
        let mask: i32 = self.alignment-1;
        offset=(offset+mask)&~mask;
        self->addedTo=addedTo;
        self->offset=offset;
        place0(&addedTo, offset);
        return offset;
    }
    pub fn getAlignment(&self) -> i32    {
        return self.alignment;
    }
    pub fn offsetString(&self) -> String    {
        return '['+Integer::toHexString(getAbsoluteOffset())+']';
    }
    pub fn toHuman(&self) -> String;
    pub fn compareTo0(&self, other: &OffsettedItem) -> i32    {
        throw UnsupportedOperationException::new("unsupported");
    }
    pub fn place0(&self, addedTo: &Section, offset: i32)    {
    }
    pub fn writeTo0(&self, file: &DexFile, out: &AnnotatedOutput);
}
