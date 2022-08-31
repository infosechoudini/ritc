use crate::arch::*;


pub const FLAGS_COMMON: usize = nr::MAP_ANONYMOUS | nr::MAP_PRIVATE ;
pub const PROTECT_COMMON: usize = nr::PROT_READ | nr::PROT_WRITE;


pub unsafe fn mmap(ptr: *mut u8, len: usize) -> *mut u8 {

    let out_addr = syscall6(nr::MMAP, ptr as usize, len as usize, PROTECT_COMMON as usize, FLAGS_COMMON as usize, usize::MAX, 0 as usize);

    if out_addr < 0 {
        return core::ptr::null_mut()
    }

    out_addr as *mut u8

}