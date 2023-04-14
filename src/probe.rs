use super::comm::{Address, NBUS, NFUNCS, NSLOT};

pub fn probe() {
    for bus in 0..NBUS {
        for slot in 0..NSLOT {
            check_device(bus as u8, slot as u8);
        }
    }
}

const VENDOR_OFFSET: u8 = 0;
const BAD_VENDOR: u16 = 0xFFFF;

fn check_device(bus: u8, slot: u8) {
    // Does this device exist?
    let addr: Address = Address {
        bus: bus,
        slot: slot,
        func: 0,
    };

    let vendor_id = super::comm::read(&addr, VENDOR_OFFSET) as u16;
    if vendor_id == BAD_VENDOR {
        return;
    }

    let header = super::comm::read_common_header(&addr);
    let multifunction = (header.header_type & (1 << 7)) != 0;

    let mut locked = super::DEVICES.get().unwrap().lock().unwrap();
    locked.insert(addr, header);

    if multifunction {
        for func in 1..NFUNCS {
            let addr = Address {
                bus: bus,
                slot: slot,
                func: func as u8,
            };

            let vendor_id = super::comm::read(&addr, VENDOR_OFFSET) as u16;
            if vendor_id != BAD_VENDOR {
                let header = super::comm::read_common_header(&addr);
                locked.insert(addr, header);
            }
        }
    }
}
