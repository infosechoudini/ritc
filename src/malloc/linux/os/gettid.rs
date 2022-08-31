use crate::arch::syscall0;
use crate::arch::nr::GETTID;

pub fn get_tid() -> usize {

    unsafe {
        syscall0(GETTID) 
    }

}