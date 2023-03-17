#[repr(C, packed)]
pub struct CommonHeader {
    pub vendor: u16,
    pub device: u16,
    pub command: u16,
    pub status: u16,
    pub rev: u8,
    pub progif: u8,
    pub subclass: u8,
    pub class: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
}
