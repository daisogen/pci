#![no_std]
#![no_main]

mod comm;
mod open;
mod probe;
mod structures;

use comm::Address;
use spin::Mutex; // TODO
use std::collections::HashMap;
use structures::CommonHeader;

lazy_static::lazy_static! {
    static ref DEVICES: Mutex<HashMap<Address, CommonHeader>> = Mutex::new(HashMap::new());
}

#[no_mangle]
fn main() {
    probe::probe();
    std::daisogen::pd_set("pci_query", open::query as u64);
    std::daisogen::pd_set("pci_ready", 1);
    std::daisogen::yld();
}
