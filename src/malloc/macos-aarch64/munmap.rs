use crate::arch::*;
use core::alloc::Layout;


// call!(kty::__NR_munmap, addr, len) as k_int

#[inline]
pub unsafe fn munmap(ptr: *mut u8, layout: Layout){
    syscall2(nr::MUNMAP, ptr as usize, layout.size());
}