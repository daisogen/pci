#![feature(daisogen_api)]
#![feature(once_cell)]

mod comm;
mod probe;
mod query;
mod structures;

use comm::Address;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use structures::CommonHeader;

pub static DEVICES: OnceLock<Mutex<HashMap<Address, CommonHeader>>> = OnceLock::new();

fn main() {
    DEVICES.get_or_init(|| Mutex::new(HashMap::new()));

    probe::probe();
    //std::daisogen::pd_set("pci_query_class", query::query_class as u64);
    std::daisogen::pd_set("pci_query_vendor", query::query_vendor as u64);
    std::daisogen::pd_set("pci_ready", 1);
    std::daisogen::yld();
}
