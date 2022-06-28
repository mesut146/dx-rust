use crate::helper;
use crate::com::android::dex::DexFormat;
use crate::com::android::dex::Dex;
use crate::com::android::dex::DexException;
use crate::com::android::dex::TableOfContents::Section;
use crate::com::android::dex::SizeOf;
use crate::com::android::dex::Dex::Section;

struct TableOfContents{
    pub header: Section,
    pub stringIds: Section,
    pub typeIds: Section,
    pub protoIds: Section,
    pub fieldIds: Section,
    pub methodIds: Section,
    pub classDefs: Section,
    pub callSiteIds: Section,
    pub methodHandles: Section,
    pub mapList: Section,
    pub typeLists: Section,
    pub annotationSetRefLists: Section,
    pub annotationSets: Section,
    pub classDatas: Section,
    pub codes: Section,
    pub stringDatas: Section,
    pub debugInfos: Section,
    pub annotations: Section,
    pub encodedArrays: Section,
    pub annotationsDirectories: Section,
    pub sections: Vec<Section>,
    pub apiLevel: i32,
    pub checksum: i32,
    pub signature: Vec<i8>,
    pub fileSize: i32,
    pub linkSize: i32,
    pub linkOff: i32,
    pub dataSize: i32,
    pub dataOff: i32,
}
impl TableOfContents{
    pub fn new(&self)    {
        self.signature=new byte[20];
    }
    pub fn readFrom(&self, dex: &Dex)    {
        readHeader(dex.open(0));
        readMap(dex.open());
        computeSizesFromOffsets();
    }
    pub fn readHeader(&self, headerIn: &Dex.Section)    {
        let magic: Vec<i8> = headerIn.readByteArray(8);
        if !DexFormat::isSupportedDexMagic(&magic)        {
            let msg: String = String::format_String_Object[]("Unexpected magic: [0x%02x, 0x%02x, 0x%02x, 0x%02x, "+"0x%02x, 0x%02x, 0x%02x, 0x%02x]", magic[0], magic[1], magic[2], magic[3], magic[4], magic[5], magic[6], magic[7]);
            throw DexException::new(&msg);
        }        
        self.apiLevel=DexFormat::magicToApi(&magic);
        self.checksum=headerIn.readInt();
        self.signature=headerIn.readByteArray(20);
        self.fileSize=headerIn.readInt();
        let headerSize: i32 = headerIn.readInt();
        if headerSize!=SizeOf::HEADER_ITEM        {
            throw DexException::new("Unexpected header: 0x"+Integer::toHexString(headerSize));
        }        
        let endianTag: i32 = headerIn.readInt();
        if endianTag!=DexFormat::ENDIAN_TAG        {
            throw DexException::new("Unexpected endian tag: 0x"+Integer::toHexString(endianTag));
        }        
        self.linkSize=headerIn.readInt();
        self.linkOff=headerIn.readInt();
        =headerIn.readInt();
        if ==0        {
            throw DexException::new("Cannot merge dex files that do not contain a map");
        }        
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        =headerIn.readInt();
        self.dataSize=headerIn.readInt();
        self.dataOff=headerIn.readInt();
    }
    pub fn readMap(&self, in: &Dex.Section)    {
        let mapSize: i32 = in.readInt();
        let previous: Section = None;
        for(        let i: i32 = 0;;i<mapSizei += 1)        {
            let type: i16 = in.readShort();
            in.readShort();
            let section: Section = getSection(type_renamed);
            let size: i32 = in.readInt();
            let offset: i32 = in.readInt();
            if (!=0&&!=size)||(!=-1&&!=offset)            {
                throw DexException::new("Unexpected map value for 0x"+Integer::toHexString(type_renamed));
            }            
            =size;
            =offset;
            if previous!=None&&>            {
                throw DexException::new("Map is unsorted at "+previous+", "+section);
            }            
            previous=section;
        }
        Arrays::sort_Object[](&self.sections);
    }
    pub fn computeSizesFromOffsets(&self)    {
        let end: i32 = self.dataOff+self.dataSize;
        for(        let i: i32 = self.sections.len()-1;;i>=0i -= 1)        {
            let section: Section = self.sections[i];
            if ==-1            {
                continue;
            }            
            if >end            {
                throw DexException::new("Map is unsorted at "+section);
            }            
            =end-;
            end=;
        }
    }
    pub fn getSection(&self, type: i16) -> Section    {
        for section in self.sections        {
            if ==type            {
                return section;
            }            
        }
        throw IllegalArgumentException::new("No such map item: "+type);
    }
    pub fn writeHeader(&self, out: &Dex.Section, api: i32)    {
        out.write_byte[](DexFormat::apiToMagic(api).getBytes_String("UTF-8"));
        out.writeInt(self.checksum);
        out.write_byte[](&self.signature);
        out.writeInt(self.fileSize);
        out.writeInt(SizeOf::HEADER_ITEM);
        out.writeInt(DexFormat::ENDIAN_TAG);
        out.writeInt(self.linkSize);
        out.writeInt(self.linkOff);
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt();
        out.writeInt(self.dataSize);
        out.writeInt(self.dataOff);
    }
    pub fn writeMap(&self, out: &Dex.Section)    {
        let count: i32 = 0;
        for section in self.sections        {
            if section.exists()            {
                count += 1;
            }            
        }
        out.writeInt(count);
        for section in self.sections        {
            if section.exists()            {
                out.writeShort();
                out.writeShort(0 as i16);
                out.writeInt();
                out.writeInt();
            }            
        }
    }
}
