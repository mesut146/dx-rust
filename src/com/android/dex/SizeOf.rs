use crate::helper;

struct SizeOf{
}
impl SizeOf{
    pub const UBYTE: i32 = 1;
    pub const USHORT: i32 = 2;
    pub const UINT: i32 = 4;
    pub const SIGNATURE: i32 = SizeOf::UBYTE*20;
    pub const HEADER_ITEM: i32 = (8*SizeOf::UBYTE)+SizeOf::UINT+SizeOf::SIGNATURE+(20*SizeOf::UINT);
    pub const STRING_ID_ITEM: i32 = SizeOf::UINT;
    pub const TYPE_ID_ITEM: i32 = SizeOf::UINT;
    pub const TYPE_ITEM: i32 = SizeOf::USHORT;
    pub const PROTO_ID_ITEM: i32 = SizeOf::UINT+SizeOf::UINT+SizeOf::UINT;
    pub const MEMBER_ID_ITEM: i32 = SizeOf::USHORT+SizeOf::USHORT+SizeOf::UINT;
    pub const CLASS_DEF_ITEM: i32 = 8*SizeOf::UINT;
    pub const MAP_ITEM: i32 = SizeOf::USHORT+SizeOf::USHORT+SizeOf::UINT+SizeOf::UINT;
    pub const TRY_ITEM: i32 = SizeOf::UINT+SizeOf::USHORT+SizeOf::USHORT;
    pub const CALL_SITE_ID_ITEM: i32 = SizeOf::UINT;
    pub const METHOD_HANDLE_ITEM: i32 = SizeOf::USHORT+SizeOf::USHORT+SizeOf::USHORT+SizeOf::USHORT;
    pub fn new(&self)    {
    }
}
