use crate::arch::{nr, syscall2};
use core::alloc::Layout;


// call!(kty::__NR_munmap, addr, len) as k_int

#[inline]
pub unsafe fn munmap(addr: *mut u8, layout: Layout){
    syscall2(nr::MUNMAP, addr as usize, layout.size());
}