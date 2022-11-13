pub const MAX_APP_NUM: usize = 16;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const APP_BASE_ADDRESS: usize = 0x80400000;
#[cfg(feature = "D1")]
pub const APP_BASE_ADDRESS: usize = 0x40400000;
