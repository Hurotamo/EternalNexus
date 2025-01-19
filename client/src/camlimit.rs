pub const IMAGE_BASE: usize = 0x400000;

// Define the sections as constants
pub const VIRT_HDRS: usize = IMAGE_BASE;
pub const RAW_HDRS: usize = 0x00;
pub const RSIZE_HDRS: usize = 0x400;
pub const HDRS_VSIZE: usize = 0x2F8;
pub const HDRS_END: usize = VIRT_HDRS + HDRS_VSIZE;

// Additional sections can be defined similarly
