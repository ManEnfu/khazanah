pub type KhzResult = u32;

pub const KHZ_OK: KhzResult = 0;
pub const KHZ_ERR_NULL: KhzResult = 1;
pub const KHZ_ERR_CSTRING: KhzResult = 2;
pub const KHZ_ERR_XML: KhzResult = 100;
pub const KHZ_ERR_OTHER: KhzResult = u32::MAX;
