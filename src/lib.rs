#![allow(warnings)]
#![allow(unused_imports)]
#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(decl_macro)]
#![feature(core_panic)]
#![cfg_attr(not(feature = "use_libc"), feature(asm))]
#![no_std]

#[cfg(test)]
extern crate std;

pub use arch::*;
pub mod os;
pub mod macros;
pub mod error;
//pub mod io;
pub mod sys;
//pub mod thread;
pub mod sys_common;
pub mod malloc;
//pub mod ffi;

#[cfg(all(target_os = "freebsd",
          target_arch = "x86_64"))]
#[path="arch/freebsd-x86_64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "macos",
          target_arch = "x86_64"))]
#[path="arch/macos-x86_64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "aarch64"))]
#[path="arch/linux-aarch64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "arm"))]
#[path="arch/linux-armeabi/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "mips"))]
#[path="arch/linux-mips/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "mips64"))]
#[path="arch/linux-mips64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "powerpc"))]
#[path="arch/linux-powerpc/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "powerpc64"))]
#[path="arch/linux-powerpc64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
          target_arch = "sparc64"))]
#[path="arch/linux-sparc64/mod.rs"]
pub mod arch;


#[cfg(all(target_os = "linux",
          target_arch = "x86"))]
#[path="arch/linux-x86/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "linux",
target_arch = "x86_64"))]
#[path="arch/linux-x86_64/mod.rs"]
pub mod arch;
