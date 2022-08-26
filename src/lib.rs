#![allow(warnings)]
#![allow(unused_imports)]
#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(core_panic)]
#![feature(prelude_2024)]
#![cfg_attr(feature = "rustc-dep-of-std", no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", feature(no_core), no_core)]



#[cfg(not(feature = "rustc-dep-of-std"))]
#[allow(unused_extern_crates)]
#[macro_use]
extern crate core;

#[cfg(not(feature = "rustc-dep-of-std"))]
#[allow(unused_extern_crates)]
extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod macros;



cfg_if! {
    if #[cfg(feature = "rustc-dep-of-std")] {
        pub use core;
        pub use core::iter;
        pub use core::ops;
        pub use core::option;
        pub use core::fmt;
        pub use core::hash;
        pub use core::num;
        pub use core::mem;
        pub use core::clone::Clone;
        pub use core::marker::{Copy, Send, Sync};
        pub use core::option::Option;
        pub use core::panic;
        pub use core::arch;
    }
}


pub use arch::*;
pub mod os;
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
