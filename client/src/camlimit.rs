pub const IMAGE_BASE: usize = 0x400000;

// Define the sections as constants
pub const VIRT_HDRS: usize = IMAGE_BASE;
pub const RAW_HDRS: usize = 0x00;
pub const RSIZE_HDRS: usize = 0x400;
pub const HDRS_VSIZE: usize = 0x2F8;
pub const HDRS_END: usize = VIRT_HDRS + HDRS_VSIZE;

// Define the camera zoom limit
pub const CAMERA_ZOOM_LIMIT: f64 = 30.0;

// Define a struct to hold the section offsets
pub struct SectionOffsets {
    pub virt_hdrs: usize,
    pub raw_hdrs: usize,
    pub rsize_hdrs: usize,
    pub hdrs_vsize: usize,
    pub hdrs_end: usize,
}

// Initialize the section offsets
pub const SECTION_OFFSETS: SectionOffsets = SectionOffsets {
    virt_hdrs: VIRT_HDRS,
    raw_hdrs: RAW_HDRS,
    rsize_hdrs: RSIZE_HDRS,
    hdrs_vsize: HDRS_VSIZE,
    hdrs_end: HDRS_END,
};
