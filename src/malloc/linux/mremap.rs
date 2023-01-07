
use crate::arch::{nr, syscall4};
pub const MREMAP_MAYMOVE: usize = 1;
pub const MREMAP_FIXED: usize = 2;
pub const MREMAP_DONTUNMAP: usize = 4;
use core::ptr;

use super::mmap::mmap;

// call!(kty::__NR_mremap, addr, old_len, new_len, flags, new_addr)

#[inline]
pub unsafe fn mremap(addr: *mut u8, oldsize: usize, newsize: usize) -> *mut u8 {


    let new_addr = syscall4(nr::MREMAP, addr as usize, oldsize, newsize, MREMAP_MAYMOVE);

    /* 
    if usize::MAX - 50 < new_addr {
        return core::ptr::null_mut();
    }
    */

    new_addr as *mut u8
}