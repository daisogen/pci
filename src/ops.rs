// Operations on given addresses

macro_rules! toaddress {
    ($e:expr) => {
        super::comm::Address::fromraw($e as u32)
    };
}

pub extern "C" fn get_bar(address: usize, bar: usize) -> usize {
    let addr = toaddress!(address);
    let header_type = super::comm::read_common_header(&addr).header_type;
    if header_type != 0 {
        return 0;
    }

    super::comm::read_header0(&addr).bar[bar] as usize
}

pub extern "C" fn set_bus_master(address: usize, value: usize) {
    let addr = toaddress!(address);
    let mut header = super::comm::read_common_header(&addr);
    if value == 1 {
        header.command |= 1 << 2; // Bus master
    } else {
        header.command &= !(1 << 2);
    }

    super::comm::write_common_header(&addr, &header);
}
