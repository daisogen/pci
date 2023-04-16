#[repr(C, packed)]
#[derive(Copy, Clone)]
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

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Header0 {
    pub common: CommonHeader,
    pub bar: [u32; 6],
    pub cardbus_cis: u32,
    pub subsystem_vendor: u16,
    pub subsystem_id: u16,
    pub expansion_rom_base: u32,
    pub capabilities: u8,
    pub reserved0: u8,
    pub reserved1: u16,
    pub reserved2: u32,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_grant: u8,
    pub max_latency: u8,
}
