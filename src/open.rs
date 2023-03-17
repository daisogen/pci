use std::boxed::Box;
use std::vec::Vec;

#[no_mangle]
pub extern "C" fn query(class: u8, subclass: u8) -> u64 {
    let mut ret: Box<Vec<u32>> = Box::new(Vec::new());

    let locked = super::DEVICES.lock();
    for (i, j) in &*locked {
        if j.class == class && j.subclass == subclass {
            (*ret).push(i.raw(0));
            std::println!("One");
        }
    }

    Box::leak(ret) as *const Vec<u32> as u64
}
