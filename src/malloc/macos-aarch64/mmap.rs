use crate::arch::*;


pub const FLAGS_COMMON: usize = nr::MAP_ANONYMOUS | nr::MAP_PRIVATE ;
pub const PROTECT_COMMON: usize = nr::PROT_READ | nr::PROT_WRITE;


pub unsafe fn mmap(ptr: *mut u8, len: usize) -> *mut u8 {

    let out_addr = syscall6(nr::MMAP , ptr as usize, len, PROTECT_COMMON , FLAGS_COMMON, -1, 0);

    if out_addr < 0 {
        return core::ptr::null_mut()
    }

    out_addr as *mut u8

}