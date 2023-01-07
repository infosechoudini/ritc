use core::result::Result::Ok;
use core::result::Result::Err;
use core::result::Result;
use core::prelude::rust_2024;
use core::arch::asm;
use core::hint;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::clone::Clone;
use core::write;
use core::assert;
use core::assert_eq;
use core::debug_assert_eq;
use crate::malloc::os::oserror;
use crate::arch::*;
use log::{trace, info, warn};


/// Some unix constants and an mmap function using assembly.
///
/// The constants were taken from sys/mman.h, and have been tested on osx and
/// linux.

//============================================================
// System call code
#[cfg(target_os = "macos")]
const SYS_MMAP: usize = 0x2000000 + 197;
#[cfg(target_os = "linux")]
const SYS_MMAP: usize = 0x09;

//============================================================
// Flags for protection

// These seem to be the same across linux and OS X
pub const PROT_READ: i32 = 0x01; // [MC2] pages can be read
pub const PROT_WRITE: i32 = 0x02; // [MC2] pages can be written
const _PROT_EXEC: i32 = 0x04;

//============================================================
// Flags contain sharing type and options.

//Sharing types; choose one.
const _MAP_SHARED: usize = 0x0001; // [MF|SHM] share changes
pub const MAP_PRIVATE: i32 = 0x0002; // [MF|SHM] changes are private

// Other flags
const _MAP_FIXED: usize = 0x0010; // [MF|SHM] interpret addr exactly
const _MAP_RENAME: usize = 0x0020; // Sun: rename private pages to file
const _MAP_NORESERVE: usize = 0x0040; // Sun: don't reserve needed swap area
const _MAP_RESERVED: usize = 0x0080; // previously unimplemented MAP_INHERIT
const _MAP_NOEXTEND: usize = 0x0100; // for MAP_FILE, don't change file size
const _MAP_HASSEMAPHORE: usize = 0x0200; // region may contain semaphores
const _MAP_NOCACHE: usize = 0x0400; // don't cache pages for this mapping
const _MAP_JIT: usize = 0x0800; // Allocate a region that will be used for JIT purposes
pub const MAP_STACK: usize = 0x020000;
pub const MAP_HUGETLB: usize = 0x040000;
pub const MAP_HUGE_2MB: usize =  1_409_286_144;

pub const MAP_ANONYMOUS: i32 = 0x0020;
pub const MAP_FAILED: i128 = 0xffffffffffffffff;
pub const ENOMEM: i32 = 12;
pub const MAP_POPULATE: i32 = 0x08000;
pub const MAP_NONBLOCK: usize =       0x10000;
// Mapping type
const _MAP_FILE: usize = 0x0000; // map from file (default)
#[cfg(target_os = "macos")]
pub const MAP_ANON: usize = 0x1000; // allocated from memory, swap space
#[cfg(target_os = "linux")]
pub const MAP_ANON: i32 = 0x20;
pub const PROT_EXEC: i32 = 4;

const _MAP_ANONYMOUS: i32 = MAP_ANON;
pub const FLAG_COMMON: usize = MAP_ANON as usize | MAP_PRIVATE as usize | MAP_POPULATE as usize   ;
pub const PROTECT_COMMON: usize = PROT_WRITE  as usize | PROT_READ as usize ;
pub const MADV_SEQUENTIAL: usize = 2;
pub const MADV_WILLNEED: usize = 3;
pub const MADV_RANDOM: usize = 1;

#[inline]
pub unsafe fn mmap(
    addr: *mut u8,
    len: usize,
) -> *mut u8 {

    let out_addr = syscall6(SYS_MMAP, addr as usize, len, PROTECT_COMMON, FLAG_COMMON, -1, 0);


    let out_check = oserror::Error::demux(out_addr);


    match out_check.is_ok() {
        true => {
            return out_addr as *mut u8
        }
        false => {
            return core::ptr::null_mut();
        }

    }

}

pub const SC_PAGE_SIZE: i32 = 30; // 30i32

#[inline]
pub unsafe fn page_size() -> usize {

    let ret = syscall0(SC_PAGE_SIZE as usize);
    ret
}

