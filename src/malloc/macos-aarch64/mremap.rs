
use crate::arch::*;
//pub const MREMAP_MAYMOVE: i32 = 1;


// call!(kty::__NR_mremap, addr, old_len, new_len, flags, new_addr)

use crate::malloc::munmap;

#[inline]
pub unsafe fn mremap(addr: *mut u8, oldsize: usize, newsize: usize) -> *mut u8 {

    let new_addr = syscall4(nr::MREMAP, addr as usize, oldsize, newsize, nr::MADV_REMOVE as usize);

    if new_addr < 0 {
        return core::ptr::null_mut();
    }

    ptr::null_mut as *mut u8

}