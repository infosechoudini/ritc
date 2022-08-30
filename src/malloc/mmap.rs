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

use crate::syscall0;
use crate::syscall6;


/// Some unix constants and an mmap function using assembly.
///
/// The constants were taken from sys/mman.h, and have been tested on osx and
/// linux.

//============================================================
// System call code
#[cfg(target_os = "macos")]
const SYS_MMAP: i64 = 0x2000000 + 197;
#[cfg(target_os = "linux")]
const SYS_MMAP: i64 = 0x09;

//============================================================
// Flags for protection

// These seem to be the same across linux and OS X
pub const PROT_READ: i32 = 0x01; // [MC2] pages can be read
pub const PROT_WRITE: i32 = 0x02; // [MC2] pages can be written
const _PROT_EXEC: i32 = 0x04;

//============================================================
// Flags contain sharing type and options.

//Sharing types; choose one.
const _MAP_SHARED: u64 = 0x0001; // [MF|SHM] share changes
pub const MAP_PRIVATE: i32 = 0x0002; // [MF|SHM] changes are private

// Other flags
const _MAP_FIXED: u64 = 0x0010; // [MF|SHM] interpret addr exactly
const _MAP_RENAME: u64 = 0x0020; // Sun: rename private pages to file
const _MAP_NORESERVE: u64 = 0x0040; // Sun: don't reserve needed swap area
const _MAP_RESERVED: u64 = 0x0080; // previously unimplemented MAP_INHERIT
const _MAP_NOEXTEND: u64 = 0x0100; // for MAP_FILE, don't change file size
const _MAP_HASSEMAPHORE: u64 = 0x0200; // region may contain semaphores
const _MAP_NOCACHE: u64 = 0x0400; // don't cache pages for this mapping
const _MAP_JIT: u64 = 0x0800; // Allocate a region that will be used for JIT purposes
pub const MAP_STACK: u64 = 0x020000;
pub const MAP_HUGETLB: u64 = 0x040000;
pub const MAP_HUGE_2MB: u64 =  1_409_286_144;

pub const MAP_ANONYMOUS: i32 = 0x0020;
pub const MAP_FAILED: i128 = 0xffffffffffffffff;
pub const ENOMEM: i32 = 12;
pub const MAP_POPULATE: i32 = 0x08000;
// Mapping type
const _MAP_FILE: u64 = 0x0000; // map from file (default)
#[cfg(target_os = "macos")]
pub const MAP_ANON: u64 = 0x1000; // allocated from memory, swap space
#[cfg(target_os = "linux")]
pub const MAP_ANON: i32 = 0x20;
pub const PROT_EXEC: i32 = 4;

const _MAP_ANONYMOUS: i32 = MAP_ANON;
pub const FLAG_COMMON: u64 = MAP_ANON as u64 | MAP_PRIVATE as u64 | MAP_POPULATE as u64 ;
pub const PROTECT_COMMON: u64 = PROT_WRITE  as u64 | PROT_READ as u64 ;



#[rust_2024::derive(Debug)]
pub struct MmapError {
    code: i64,
}

#[cfg(all(not(feature = "use_libc"), target_os = "linux"))]
#[inline]
pub unsafe fn mmap(
    addr: *mut u8,
    len: usize,
) -> *mut u8 {

    let out_addr = syscall6(SYS_MMAP as usize, addr as usize, len as usize, PROTECT_COMMON as usize, FLAG_COMMON as usize, usize::MAX, 0 as usize);

    if out_addr == usize::MAX || out_addr == usize::MIN {
        //let err =  MmapError { code: (-out_addr) };
        return core::ptr::null_mut()
    }

    out_addr as *mut u8
}

pub const SC_PAGE_SIZE: i32 = 30; // 30i32

#[inline]
pub unsafe fn page_size() -> usize {

    let ret = syscall0(SC_PAGE_SIZE as usize);
    ret
}


#[cfg(all(not(feature = "use_libc"), target_os = "macos"))]
pub unsafe fn mmap(
    addr: *mut u8,
    len: usize,
    prot: u64,
    flags: u64,
    fd: u64,
    offset: i64,
) -> Result<*mut u8, MmapError> {
    let addr = addr as i64;
    let out_addr: i64;
    let err: i64;

    asm!(
        r"
        // Make a syscall, using the parameters in the registers
        syscall
        // osx sets the carry bit if there's an error. If that happens, we jump
        // to label 1
        jc 1f
        // Set edx to 0 to indicate no error
        mov edx, 0
        // Jump to label 2 to finish this
        jmp 2f
1:
        // There was an error. Set edx to 1 to indicate that.
        mov edx, 1
2:
    ",
    inout("eax") SYS_MMAP => out_addr,
    in("edi") addr,
    in("esi") len,
    inout("edx") prot => err,
    in("r10d") flags,
    in("r8d") fd,
    in("r9d") offset,
    );

    if err != 0 {
        return Err(MmapError { code: out_addr });
    }

    Ok(out_addr as *mut u8)
}
