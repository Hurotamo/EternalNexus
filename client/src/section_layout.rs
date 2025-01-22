pub const IMAGE_BASE: usize = 0x400000;

// Define the sections as constants
pub const VIRT_HDRS: usize = IMAGE_BASE;
pub const RAW_HDRS: usize = 0x00;
pub const RSIZE_HDRS: usize = 0x400;
pub const HDRS_VSIZE: usize = 0x2F8;
pub const HDRS_END: usize = VIRT_HDRS + HDRS_VSIZE;

pub const VIRT_TEXT: usize = IMAGE_BASE + 0x1000;
pub const RAW_TEXT: usize = 0x400;
pub const RSIZE_TEXT: usize = 0x2FE200;
pub const TEXT_VSIZE: usize = 0x2FE131;
pub const TEXT_END: usize = VIRT_TEXT + TEXT_VSIZE;

pub const VIRT_RDATA: usize = IMAGE_BASE + 0x300000;
pub const RAW_RDATA: usize = 0x2FE600;
pub const RSIZE_RDATA: usize = 0x5D000;
pub const RDATA_VSIZE: usize = 0x5CEBA;
pub const RDATA_END: usize = VIRT_RDATA + RDATA_VSIZE;

pub const VIRT_DATA: usize = IMAGE_BASE + 0x35D000;
pub const RAW_DATA: usize = 0x35B600;
pub const RSIZE_DATA: usize = 0x14200;
pub const DATA_VSIZE: usize = 0x1B52BB8;
pub const DATA_END: usize = VIRT_DATA + DATA_VSIZE;

pub const VIRT_RSRC: usize = IMAGE_BASE + 0x1EB0000;
pub const RAW_RSRC: usize = 0x36F800;
pub const RSIZE_RSRC: usize = 0xBE00;
pub const RSRC_VSIZE: usize = 0xBCFC;
pub const RSRC_END: usize = VIRT_RSRC + RSRC_VSIZE;

pub const VIRT_ETERNAL_NEXUS: usize = IMAGE_BASE + 0x1EBC000;
pub const RAW_ETERNAL_NEXUS: usize = 0x37B600;
pub const RSIZE_ETERNAL_NEXUS: usize = 0x2000;
pub const ETERNAL_NEXUS_VSIZE: usize = 0x2000;
pub const ETERNAL_NEXUS_END: usize = VIRT_ETERNAL_NEXUS + ETERNAL_NEXUS_VSIZE;

// Struct to hold section information
pub struct SectionLayout {
    pub virt_hdrs: usize,
    pub raw_hdrs: usize,
    pub rsize_hdrs: usize,
    pub hdrs_vsize: usize,
    pub hdrs_end: usize,
    pub virt_text: usize,
    pub raw_text: usize,
    pub rsize_text: usize,
    pub text_vsize: usize,
    pub text_end: usize,
    pub virt_rdata: usize,
    pub raw_rdata: usize,
    pub rsize_rdata: usize,
    pub rdata_vsize: usize,
    pub rdata_end: usize,
    pub virt_data: usize,
    pub raw_data: usize,
    pub rsize_data: usize,
    pub data_vsize: usize,
    pub data_end: usize,
    pub virt_rsrc: usize,
    pub raw_rsrc: usize,
    pub rsize_rsrc: usize,
    pub rsrc_vsize: usize,
    pub rsrc_end: usize,
    pub virt_eternal_nexus: usize,
    pub raw_eternal_nexus: usize,
    pub rsize_eternal_nexus: usize,
    pub eternal_nexus_vsize: usize,
    pub eternal_nexus_end: usize,
}

// Initialize the section layout
pub const SECTION_LAYOUT: SectionLayout = SectionLayout {
    virt_hdrs: VIRT_HDRS,
    raw_hdrs: RAW_HDRS,
    rsize_hdrs: RSIZE_HDRS,
    hdrs_vsize: HDRS_VSIZE,
    hdrs_end: HDRS_END,
    virt_text: VIRT_TEXT,
    raw_text: RAW_TEXT,
    rsize_text: RSIZE_TEXT,
    text_vsize: TEXT_VSIZE,
    text_end: TEXT_END,
    virt_rdata: VIRT_RDATA,
    raw_rdata: RAW_RDATA,
    rsize_rdata: RSIZE_RDATA,
    rdata_vsize: RDATA_VSIZE,
    rdata_end: RDATA_END,
    virt_data: VIRT_DATA,
    raw_data: RAW_DATA,
    rsize_data: RSIZE_DATA,
    data_vsize: DATA_VSIZE,
    data_end: DATA_END,
    virt_rsrc: VIRT_RSRC,
    raw_rsrc: RAW_RSRC,
    rsize_rsrc: RSIZE_RSRC,
    rsrc_vsize: RSRC_VSIZE,
    rsrc_end: RSRC_END,
    virt_eternal_nexus: VIRT_ETERNAL_NEXUS,
    raw_eternal_nexus: RAW_ETERNAL_NEXUS,
    rsize_eternal_nexus: RSIZE_ETERNAL_NEXUS,
    eternal_nexus_vsize: ETERNAL_NEXUS_VSIZE,
    eternal_nexus_end: ETERNAL_NEXUS_END,
};
