use crate::helper;
use crate::com::android::dx::dex::file::DexFile;
use crate::com::android::dx::util::AnnotatedOutput;

struct Section{
    pub name: String,
    pub file: DexFile,
    pub alignment: i32,
    pub fileOffset: i32,
    pub prepared: boolean,
}
impl Section{
    pub fn validateAlignment(alignment: i32)    {
        if (alignment<=0)||(alignment&(alignment-1))!=0        {
            throw IllegalArgumentException::new("invalid alignment");
        }        
    }
    pub fn new(&self, name: &String, file: &DexFile, alignment: i32)    {
        if file==None        {
            throw NullPointerException::new("file == null");
        }        
        Section::validateAlignment(alignment);
        self->name=name;
        self->file=file;
        self->alignment=alignment;
        self->fileOffset=-1;
        self->prepared=false;
    }
    pub fn getFile(&self) -> DexFile    {
        return self.file;
    }
    pub fn getAlignment(&self) -> i32    {
        return self.alignment;
    }
    pub fn getFileOffset(&self) -> i32    {
        if self.fileOffset<0        {
            throw RuntimeException::new("fileOffset not set");
        }        
        return self.fileOffset;
    }
    pub fn setFileOffset(&self, fileOffset: i32) -> i32    {
        if fileOffset<0        {
            throw IllegalArgumentException::new("fileOffset < 0");
        }        
        if self->fileOffset>=0        {
            throw RuntimeException::new("fileOffset already set");
        }        
        let mask: i32 = self.alignment-1;
        fileOffset=(fileOffset+mask)&~mask;
        self->fileOffset=fileOffset;
        return fileOffset;
    }
    pub fn writeTo(&self, out: &AnnotatedOutput)    {
        throwIfNotPrepared();
        align(&out);
        let cursor: i32 = out.getCursor();
        if self.fileOffset<0        {
            self.fileOffset=cursor;
        }        else         if self.fileOffset!=cursor        {
            throw RuntimeException::new("alignment mismatch: for "+self+", at "+cursor+", but expected "+self.fileOffset);
        }        
        if out.annotates()        {
            if self.name!=None            {
                out.annotate_int_String(0, "\n"+self.name+":");
            }            else             if cursor!=0            {
                out.annotate_int_String(0, "\n");
            }            
        }        
        writeTo0(&out);
    }
    pub fn getAbsoluteOffset(&self, relative: i32) -> i32    {
        if relative<0        {
            throw IllegalArgumentException::new("relative < 0");
        }        
        if self.fileOffset<0        {
            throw RuntimeException::new("fileOffset not yet set");
        }        
        return self.fileOffset+relative;
    }
    pub fn getAbsoluteItemOffset(&self, item: &Item) -> i32;
    pub fn prepare(&self)    {
        throwIfPrepared();
        prepare0();
        self.prepared=true;
    }
    pub fn items(&self) -> Collection<? extends Item>;
    pub fn prepare0(&self);
    pub fn writeSize(&self) -> i32;
    pub fn throwIfNotPrepared(&self)    {
        if !self.prepared        {
            throw RuntimeException::new("not prepared");
        }        
    }
    pub fn throwIfPrepared(&self)    {
        if self.prepared        {
            throw RuntimeException::new("already prepared");
        }        
    }
    pub fn align(&self, out: &AnnotatedOutput)    {
        out.alignTo(self.alignment);
    }
    pub fn writeTo0(&self, out: &AnnotatedOutput);
    pub fn getName(&self) -> String    {
        return self.name;
    }
}
