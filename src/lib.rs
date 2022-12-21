#![allow(warnings)]
#![allow(unused_imports)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(core_panic)]
#![feature(prelude_2024)]
#![cfg_attr(feature = "rustc-dep-of-std", no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", feature(no_core), no_core)]
#![feature(const_mut_refs)]
#![feature(const_option)]
#![feature(const_ptr_write)]
#![feature(ptr_as_uninit)]
#![feature(alloc_layout_extra)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(slice_ptr_get)]
#![feature(type_ascription)]
#[cfg(not(feature = "rustc-dep-of-std"))]
#[allow(unused_extern_crates)]
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
        pub use core::cmp;
        pub use core::prelude;
        pub use core::hash;
        pub use core::num;
        pub use core::mem;
        pub use core::clone;
        pub use core::marker;
        pub use core::clone::Clone;
        pub use core::marker::{Copy, Send, Sync};
        pub use core::option::Option;
        pub use core::panic;
        pub use core::write;
        pub use core::assert;
        pub use core::assert_eq;
        pub use core::debug_assert_eq;
        pub use core::ops::Drop;
        pub use core::cmp::Eq;
        pub use core::cmp::PartialEq;
        pub use core::fmt::Debug;
        pub use core::prelude::rust_2024;
        pub use core::intrinsics;
        pub use core::unreachable;

    }
}


pub use arch::*;
pub mod error;

#[cfg(all(target_os = "macos",
          target_arch = "aarch64"))]
#[path="arch/macos-aarch64/mod.rs"]
pub mod arch;
#[cfg(all(target_os = "macos",
          target_arch = "aarch64"))]
#[path="malloc/macos-aarch64/mod.rs"]
pub mod malloc;

#[cfg(all(target_os = "freebsd",
          target_arch = "x86_64"))]
#[path="arch/freebsd-x86_64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "macos",
          target_arch = "x86_64"))]
#[path="arch/macos-x86_64/mod.rs"]
pub mod arch;

#[cfg(all(target_os = "macos",
          target_arch = "x86_64"))]
#[path="malloc/macos-x86_64/mod.rs"]
pub mod malloc;

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
#[cfg(all(target_os = "linux",
target_arch = "x86_64"))]
#[path="malloc/linux/mod.rs"]
pub mod malloc;