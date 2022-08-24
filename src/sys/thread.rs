use core::ffi::CStr;
//use crate::io;
use core::mem;
//use core::num::NonZeroUsize;
//use core::ptr;
//use crate::sys::{os, stack_overflow};
//use core::time::Duration;
use crate::sys_common::pthread;
use core::marker::Copy;
extern crate alloc;
use alloc::boxed::Box;
use crate::arch::{
    syscall0,
    syscall4,
    syscall5,
    syscall6,

};

const SYS_SCHED_YIELD: usize = 24; 
const SYS_PRCTL: usize = 157;
const PR_SET_NAME: usize = 15;
const PR_GET_NAME: usize = 16;


#[derive(Clone, Copy)]
pub struct Thread {
    pub id: u64,
}


// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub fn new() -> Option<Thread> {
        let p = pthread::Pthread::default().create();
        Some(Thread { id: p.thread })

    }

    pub fn yield_now(self) {
        unsafe {
            let ret = syscall0(SYS_SCHED_YIELD);
            debug_assert_eq!(ret, 0);
        }

    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn set_name(self, name: &str) -> Self{
        // pthread wrapper only appeared in glibc 2.12, so we use syscall
        // directly.
        unsafe {
            let name_usize = name.as_ptr() as usize;
            let ret = syscall5(SYS_PRCTL, PR_SET_NAME, name_usize, 0, 0, 0);
            assert_eq!(ret, 0);
        }

        self
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn get_name<'a>(self) -> usize {
        // pthread wrapper only appeared in glibc 2.12, so we use syscall
        // directly.
        unsafe {
            let ret = syscall5(SYS_PRCTL, PR_GET_NAME, 0, 0, 0, 0);
            assert_ne!(ret, usize::MAX);
            return ret;
        }
    }


    /*
    #[cfg(not(target_os = "espidf"))]
    pub fn sleep(dur: Duration) {
        let mut secs = dur.as_secs();
        let mut nsecs = dur.subsec_nanos() as _;

        // If we're awoken with a signal then the return value will be -1 and
        // nanosleep will fill in `ts` with the remaining time.
        unsafe {
            while secs > 0 || nsecs > 0 {
                let mut ts = libc::timespec {
                    tv_sec: cmp::min(libc::time_t::MAX as u64, secs) as libc::time_t,
                    tv_nsec: nsecs,
                };
                secs -= ts.tv_sec as u64;
                let ts_ptr = &mut ts as *mut _;
                if libc::nanosleep(ts_ptr, ts_ptr) == -1 {
                    assert_eq!(os::errno(), libc::EINTR);
                    secs += ts.tv_sec as u64;
                    nsecs = ts.tv_nsec;
                } else {
                    nsecs = 0;
                }
            }
        }
    }

    pub fn join(self) {
        unsafe {
            let ret = libc::pthread_join(self.id, ptr::null_mut());
            mem::forget(self);
            assert!(ret == 0, "failed to join thread: {}", io::Error::from_raw_os_error(ret));
        }
    }
    */

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn into_id(self) -> u64 {
        let id = self.id;
        mem::forget(self);
        id
    }
}

/*

impl Drop for Thread {
    fn drop(&mut self) {
        let mut p = pthread::Pthread::default();
        p.thread = self.id;
        let ret = p.destroy();
        debug_assert_eq!(ret, 0);
    }
}

*/
