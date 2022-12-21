
use core::alloc::Layout;

use crate::arch::{nr, syscall4};
pub const MREMAP_MAYMOVE: i32 = 1;


// call!(kty::__NR_mremap, addr, old_len, new_len, flags, new_addr)
use crate::malloc::munmap::munmap;
use crate::malloc::mmap::mmap;
use core::ptr;

#[inline]
pub unsafe fn mremap(addr: *mut u8, oldlayout: Layout, newsize: usize) -> *mut u8 {

    munmap(addr, oldlayout);

    let ret = mmap(ptr::null_mut(), newsize);

    ret

}