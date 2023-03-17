use super::structures::CommonHeader;

pub const NBUS: usize = 256;
pub const NSLOT: usize = 32;
pub const NFUNCS: usize = 8;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Address {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
}

impl Address {
    pub fn raw(&self, off: u8) -> u32 {
        let address: u32 = 1 << 31; // Enable bit
        let address = address | ((self.bus as u32) << 16);
        let address = address | ((self.slot as u32) << 11);
        let address = address | ((self.func as u32) << 8);
        let address = address | ((off as u32) & 0xFC);
        address
    }
}

pub fn read(addr: &Address, off: u8) -> u32 {
    std::asm::out32(CONFIG_ADDRESS, addr.raw(off));
    std::asm::in32(CONFIG_DATA)
}

pub fn read_common_header(addr: &Address) -> CommonHeader {
    // Read the 4 dwords
    let mut words: [u32; 4] = [0; 4];
    for i in 0..4 {
        words[i] = read(addr, (i * 4) as u8);
    }
    unsafe { std::mem::transmute::<[u32; 4], CommonHeader>(words) }
}
