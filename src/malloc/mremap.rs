
use crate::arch::{nr, syscall4};
pub const MREMAP_MAYMOVE: i32 = 1;


// call!(kty::__NR_mremap, addr, old_len, new_len, flags, new_addr)

#[inline]
pub unsafe fn mremap(addr: *mut u8, oldsize: usize, newsize: usize) -> *mut u8 {

    let new_addr = syscall4(nr::MREMAP, addr as usize, oldsize, newsize, MREMAP_MAYMOVE as usize);

    if (new_addr as i64) < 0 {
        return core::ptr::null_mut();
    }

    new_addr as *mut u8

}