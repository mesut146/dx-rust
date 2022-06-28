use crate::helper;
use crate::com::android::dex::TableOfContents;
use crate::com::android::dx::io::DexIndexPrinter;
use crate::com::android::dex::Dex;
use crate::com::android::dex::TableOfContents::Section;
use crate::com::android::dex::Dex::Section;

struct DexIndexPrinter{
    pub dex: Dex,
    pub tableOfContents: TableOfContents,
}
impl DexIndexPrinter{
    pub fn new(&self, file: &File)    {
        self->dex=Dex::new(&file);
        self->tableOfContents=self.dex.getTableOfContents();
    }
    pub fn printMap(&self)    {
        for section in         {
            if !=-1            {
                System::out.println_String("section "+Integer::toHexString()+" off="+Integer::toHexString()+" size="+Integer::toHexString()+" byteCount="+Integer::toHexString());
            }            
        }
    }
    pub fn printStrings(&self)    {
        let index: i32 = 0;
        for string in self.dex.strings()        {
            System::out.println_String("string "+index+": "+string);
            index += 1;
        }
    }
    pub fn printTypeIds(&self)    {
        let index: i32 = 0;
        for type in self.dex.typeIds()        {
            System::out.println_String("type "+index+": "+self.dex.strings().get(&type_renamed));
            index += 1;
        }
    }
    pub fn printProtoIds(&self)    {
        let index: i32 = 0;
        for protoId in self.dex.protoIds()        {
            System::out.println_String("proto "+index+": "+protoId);
            index += 1;
        }
    }
    pub fn printFieldIds(&self)    {
        let index: i32 = 0;
        for fieldId in self.dex.fieldIds()        {
            System::out.println_String("field "+index+": "+fieldId);
            index += 1;
        }
    }
    pub fn printMethodIds(&self)    {
        let index: i32 = 0;
        for methodId in self.dex.methodIds()        {
            System::out.println_String("methodId "+index+": "+methodId);
            index += 1;
        }
    }
    pub fn printTypeLists(&self)    {
        if ==-1        {
            System::out.println_String("No type lists");
            return;
        }        
        let in: Section = self.dex.open();
        for(        let i: i32 = 0;;i<i += 1)        {
            let size: i32 = in_renamed.readInt();
            System::out.print_String("Type list i="+i+", size="+size+", elements=");
            for(            let t: i32 = 0;;t<sizet += 1)            {
                System::out.print_String(" "+self.dex.typeNames().get(in_renamed.readShort() as i32));
            }
            if size%2==1            {
                in_renamed.readShort();
            }            
            System::out.println();
        }
    }
    pub fn printClassDefs(&self)    {
        let index: i32 = 0;
        for classDef in self.dex.classDefs()        {
            System::out.println_String("class def "+index+": "+classDef);
            index += 1;
        }
    }
    pub fn main(args: &Vec<String>)    {
        let indexPrinter: DexIndexPrinter = DexIndexPrinter::new(File::new(args[0]));
        indexPrinter.printMap();
        indexPrinter.printStrings();
        indexPrinter.printTypeIds();
        indexPrinter.printProtoIds();
        indexPrinter.printFieldIds();
        indexPrinter.printMethodIds();
        indexPrinter.printTypeLists();
        indexPrinter.printClassDefs();
    }
}
