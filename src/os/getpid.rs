use crate::{syscall0};
use crate::nr::GETPID;



pub fn getpid() -> usize {
    unsafe {
        syscall0(GETPID)
    }
}