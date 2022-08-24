#![allow(dead_code)] // not used on all platforms

use crate::arch::{syscall5, syscall0, syscall3, syscall2};
use crate::arch::nr::GETTID;


const CLONE_VM: usize             = 0x100;
const CLONE_FS: usize             = 0x200;
const CLONE_FILES: usize          = 0x400;
const CLONE_SIGHAND: usize        = 0x800;
const CLONE_PTRACE: usize         = 0x2000;
const CLONE_VFORK: usize          = 0x4000;
const CLONE_PARENT: usize         = 0x8000;
const CLONE_THREAD: usize         = 0x10000;
const CLONE_NEWNS: usize          = 0x20000;
const CLONE_SYSVSEM: usize        = 0x40000;
const CLONE_SETTLS: usize         = 0x80000;
const CLONE_PARENT_SETTID: usize  = 0x100000;
const CLONE_CHILD_CLEARTID: usize = 0x200000;
const CLONE_UNTRACED: usize       = 0x800000;
const CLONE_CHILD_SETTID: usize   = 0x1000000;
const CLONE_STOPPED: usize        = 0x2000000;
const CLONE_NEWUTS: usize         = 0x4000000;
const CLONE_NEWIPC: usize         = 0x8000000;

const CLONE_FLAGS: usize = CLONE_VM | CLONE_FS| CLONE_SIGHAND | CLONE_SYSVSEM | CLONE_THREAD;

const SYS_CLONE: usize = 56;
const SYS_TGKILL: usize = 234;
const SYS_TKILL: usize = 200;
const SIGCHLD: usize = 17;
const SIGTERM: usize = 15;
const SIGABRT: usize = 6;
/* 
pub type Key = libc::pthread_key_t;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let mut key = 0;
    assert_eq!(libc::pthread_key_create(&mut key, mem::transmute(dtor)), 0);
    key
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let r = libc::pthread_setspecific(key, value as *mut _);
    debug_assert_eq!(r, 0);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    libc::pthread_getspecific(key) as *mut u8
}

#[inline]
pub unsafe fn destroy(key: Key) {
    let r = libc::pthread_key_delete(key);
    debug_assert_eq!(r, 0);
}

#[inline]
pub fn requires_synchronized_create() -> bool {
    false
}

*/

#[derive(Default, Clone)]
pub struct Pthread{
    pub thread: u64,
}

impl Pthread {

    pub fn create (&mut self) -> Pthread {
        let tid = Pthread::get_tid();
        let new_thread = self.clone(tid);
        new_thread
    }

    pub fn get_tid() -> usize {

        unsafe {
            syscall0(GETTID) 
        }

    }

    pub fn get_specific(key: u64) -> Pthread {
        Pthread { thread: key }
    }

    pub fn set_specific(key: u64) -> Pthread {
        Pthread { thread: key }
    }

    pub fn destroy(self) -> usize {
        unsafe { 
            let child_thread = self.thread as usize;
            // TKILL instead of TGKILL due to MUSL implementation of pthread_kill
            let ret = syscall2(SYS_TKILL, child_thread, SIGABRT);
            ret
        }
    }


    /*
    (clone, 4, CLONE_CHILD_SETTID | CLONE_CHILD_CLEARTID | SIGCHLD, 0,    
                  NULL, &THREAD_SELF->tid)
                   */
    fn clone (&mut self, tid: usize) -> Pthread {
        unsafe {
            // syscall5(sys_clone, clone_attr, 0, null, tid)
            let ret = syscall5(SYS_CLONE, 4, CLONE_FLAGS, 0, 0, tid);
            Pthread { thread: ret as u64 }
        }
    }
}


