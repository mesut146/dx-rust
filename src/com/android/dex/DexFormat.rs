use crate::helper;

let static VERSION_FOR_API_10000: String = "040";
let static VERSION_FOR_API_28: String = "039";
let static VERSION_FOR_API_26: String = "038";
let static VERSION_FOR_API_24: String = "037";
let static VERSION_FOR_API_13: String = "035";
let static VERSION_CURRENT: String = DexFormat::VERSION_FOR_API_28;
let static DEX_IN_JAR_NAME: String = "classes.dex";
let static MAGIC_PREFIX: String = "dex\n";
let static MAGIC_SUFFIX: String = "\0";
struct DexFormat{
}
impl DexFormat{
    pub const API_SPACES_IN_SIMPLE_NAME: i32 = 10000;
    pub const API_CONST_METHOD_HANDLE: i32 = 28;
    pub const API_METHOD_HANDLES: i32 = 26;
    pub const API_DEFINE_INTERFACE_METHODS: i32 = 24;
    pub const API_INVOKE_INTERFACE_METHODS: i32 = 24;
    pub const API_INVOKE_STATIC_INTERFACE_METHODS: i32 = 21;
    pub const API_NO_EXTENDED_OPCODES: i32 = 13;
    pub const API_CURRENT: i32 = DexFormat::API_CONST_METHOD_HANDLE;
    pub const ENDIAN_TAG: i32 = 0x12345678;
    pub const MAX_MEMBER_IDX: i32 = 0xFFFF;
    pub const MAX_TYPE_IDX: i32 = 0xFFFF;
    pub fn new(&self)    {
    }
    pub fn magicToApi(magic: &Vec<i8>) -> i32    {
        if magic.len()!=8        {
            return -1;
        }        
        if (magic[0]!='d')||(magic[1]!='e')||(magic[2]!='x')||(magic[3]!='\n')||(magic[7]!='\0')        {
            return -1;
        }        
        let version: String = ""+(magic[4] as char)+(magic[5] as char)+(magic[6] as char);
        if version.equals(&DexFormat::VERSION_FOR_API_13)        {
            return DexFormat::API_NO_EXTENDED_OPCODES;
        }        else         if version.equals(&DexFormat::VERSION_FOR_API_24)        {
            return DexFormat::API_DEFINE_INTERFACE_METHODS;
        }        else         if version.equals(&DexFormat::VERSION_FOR_API_26)        {
            return DexFormat::API_METHOD_HANDLES;
        }        else         if version.equals(&DexFormat::VERSION_FOR_API_28)        {
            return DexFormat::API_CONST_METHOD_HANDLE;
        }        else         if version.equals(&DexFormat::VERSION_FOR_API_10000)        {
            return DexFormat::API_SPACES_IN_SIMPLE_NAME;
        }        else         if version.equals(&DexFormat::VERSION_CURRENT)        {
            return DexFormat::API_CURRENT;
        }        
        return -1;
    }
    pub fn apiToMagic(targetApiLevel: i32) -> String    {
        let version: String;
        if targetApiLevel>=DexFormat::API_CURRENT        {
            version=DexFormat::VERSION_CURRENT;
        }        else         if targetApiLevel>=DexFormat::API_SPACES_IN_SIMPLE_NAME        {
            version=DexFormat::VERSION_FOR_API_10000;
        }        else         if targetApiLevel>=DexFormat::API_CONST_METHOD_HANDLE        {
            version=DexFormat::VERSION_FOR_API_28;
        }        else         if targetApiLevel>=DexFormat::API_METHOD_HANDLES        {
            version=DexFormat::VERSION_FOR_API_26;
        }        else         if targetApiLevel>=DexFormat::API_DEFINE_INTERFACE_METHODS        {
            version=DexFormat::VERSION_FOR_API_24;
        }        else         {
            version=DexFormat::VERSION_FOR_API_13;
        }
        return DexFormat::MAGIC_PREFIX+version+DexFormat::MAGIC_SUFFIX;
    }
    pub fn isSupportedDexMagic(magic: &Vec<i8>) -> boolean    {
        let api: i32 = DexFormat::magicToApi(&magic);
        return api>0;
    }
}
