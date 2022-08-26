#![cfg(any(
    target_os = "linux",
    target_os = "android",
    all(target_os = "emscripten", target_feature = "atomics")
))]

use core::sync::atomic::AtomicU32;
use core::time::Duration;
use crate::os::oserror::*;
use core::ptr::null;
use core::sync::atomic::Ordering::Relaxed;
use core::sync::atomic::AtomicI32;
use core::option::Option::Some;
use core::option::Option;
use core::result::Result::Err;
use core::result::Result::Ok;
use core::marker::Sync;
use core::sync;
use core::prelude::rust_2024;


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


pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub const CLOCK_MONOTONIC: usize = 1;
pub const FUTEX_WAIT_BITSET: usize = 9;
pub const FUTEX_PRIVATE_FLAG: usize = 128;
pub const FUTEX_WAKE: usize = 1;
pub const FUTEX_FD: i32 = 2;
pub const SYS_FUTEX: usize = 202;
pub const ETIMEDOUT: i32 = 110;
pub const EINTR: i32 = 4;

use crate::arch::{
    syscall0,
    syscall3,
    syscall5,
    syscall6,
};

#[must_use]
#[rust_2024::derive(Debug, Clone, Copy)]
pub struct FutexCall {
	uaddr: *const AtomicI32,
	futex_op: i32,
	val: i32,
	timeout: *const timespec,
	uaddr2: *const AtomicI32,
	val3: i32,
}

impl FutexCall {
	#[inline]
	pub const fn new() -> Self {
		Self {
			uaddr: null(),
			futex_op: 0,
			val: 0 ,
			timeout: null(),
			uaddr2: null(),
			val3: 0,
		}
	}

	#[inline]
	pub fn uaddr(self, uaddr: *const AtomicI32) -> Self {
		Self { uaddr, ..self }
	}

	#[inline]
	pub fn futex_op(self, futex_op: i32) -> Self {
		Self { futex_op, ..self }
	}

	#[inline]
	pub fn val(self, val: i32) -> Self {
		Self { val, ..self }
	}

	#[inline]
	pub fn timeout(self, timeout: *const timespec) -> Self {
		Self { timeout, ..self }
	}

	#[inline]
	pub fn val2(self, val2: i32) -> Self {
		Self {
			timeout: val2 as *const _,
			..self
		}
	}

	#[inline]
	pub fn uaddr2(self, uaddr2: *const AtomicI32) -> Self {
		Self { uaddr2, ..self }
	}

	#[inline]
	pub fn val3(self, val3: i32) -> Self {
		Self { val3, ..self }
	}

	#[inline]
	pub unsafe fn call(self) -> Result<usize, Error> {
		let result = syscall6(
			SYS_FUTEX,
			self.uaddr as usize,
			self.futex_op as usize,
			self.val as usize,
			self.timeout as usize,
			self.uaddr2 as usize,
			self.val3 as usize,
		);

        match Error::demux(result) {
            Ok(value) => return Ok(value),
            Err(value) => Err(value),
        } 
	}


}

pub struct Futex {
    pub addr: usize,
}


impl Futex {

    pub fn new() -> Result<Futex, Error>{

        let mut futexcall = FutexCall::new();
        futexcall.futex_op(FUTEX_FD);
        let addr = unsafe { futexcall.call() } ;

        if addr.is_ok(){
            Ok(Futex {
                addr: addr.unwrap(),
            })
        } else {
            Err(addr.err().unwrap())
        }
        


    }


    pub fn clone (&mut self, tid: usize) -> Futex {
        unsafe {
            // syscall5(sys_clone, clone_attr, 0, null, tid)
            let ret = syscall5(SYS_CLONE, 4, CLONE_FLAGS, 0, 0, tid);
            Futex { addr: ret }
        }
    }

}

/// Wait for a futex_wake operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {


    // Calculate the timeout as an absolute timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec =
        timeout.and_then(|d| Some(timespec{tv_sec: CLOCK_MONOTONIC as i64, tv_nsec: d.as_nanos() as i64 }));


    let f = futex.load(sync::atomic::Ordering::Relaxed);
    loop {
        // Use FUTEX_WAIT_BITSET rather than FUTEX_WAIT to be able to give an
        // absolute time rather than a relative time.
        let r = unsafe {
            syscall6(
                SYS_FUTEX as usize,
                f as usize,
                FUTEX_WAIT_BITSET as usize | FUTEX_PRIVATE_FLAG as usize,
                expected as usize,
                timespec.as_ref().map_or(null(), |t| &t ) as usize,
                null::<u32>() as usize, // This argument is unused for FUTEX_WAIT_BITSET.
                !0usize,         // A full bitmask, to make it behave like a regular FUTEX_WAIT.
            )
        };


        if r < 0 {
            let errno = -(r as i32);
            match errno {
                ETIMEDOUT => return false,
                EINTR => continue,
                _ => return true,
            }
        }
    }
}

/// Wake up one thread that's blocked on futex_wait on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    unsafe {
        syscall3(
            SYS_FUTEX as usize,
            futex as *const AtomicU32 as usize ,
            FUTEX_WAKE as usize | FUTEX_PRIVATE_FLAG as usize ,
            1,
        ) > 0
    }
}

/// Wake up all threads that are waiting on futex_wait on this futex.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn futex_wake_all(futex: &AtomicU32) {
    unsafe {
        syscall3(
            SYS_FUTEX as usize ,
            futex as *const AtomicU32 as usize ,
            FUTEX_WAKE as usize | FUTEX_PRIVATE_FLAG as usize ,
            i32::MAX as usize ,
        );
    }
}

