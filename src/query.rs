//use std::boxed::Box;
use std::vec::Vec;

/*#[no_mangle]
pub extern "C" fn query_class(class: u8, subclass: u8) -> u64 {
    let mut ret: Box<Vec<u32>> = Box::new(Vec::new());

    let locked = super::DEVICES.lock().unwrap();
    for (i, j) in &*locked {
        if j.class == class && j.subclass == subclass {
            (*ret).push(i.raw(0));
            std::println!("One");
        }
    }

    Box::leak(ret) as *const Vec<u32> as u64
}*/

#[no_mangle]
pub extern "C" fn query_vendor(vendor: usize, device: usize) -> usize {
    let mut ret: Vec<u32> = vec![];

    let locked = super::DEVICES.get().unwrap().lock().unwrap();
    for (i, j) in &*locked {
        if j.vendor == vendor as u16 && j.device == device as u16 {
            ret.push(i.raw(0));
        }
    }

    std::daisogen::alloc_caller_serialized(&ret)
}
