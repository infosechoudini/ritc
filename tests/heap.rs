#![allow(unused_imports)]
#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]


use core::alloc::{Layout, GlobalAlloc};
use core::alloc::Allocator;
extern crate alloc;
use alloc::vec::Vec;
use std::alloc::Global;
use test_log::test;

use ritc::malloc::mmap_allocator::MmapAllocator;

//#[global_allocator]
static ALLOCATOR: MmapAllocator =  MmapAllocator::INIT;

/// Issue #45955 and #62251.
#[test]
fn alloc_system_overaligned_request() {
    check_overalign_requests()
}


fn check_overalign_requests() {
    for &align in &[4, 8, 16, 32] {
        // less than and bigger than `MIN_ALIGN`
        for &size in &[align / 2, align - 1] {
            // size less than alignment
            let iterations = 128;
            unsafe {
                let pointers: Vec<_> = (0..iterations)
                    .map(|_| {
                        ALLOCATOR.allocate(Layout::from_size_align(size, align).unwrap()).unwrap()
                    })
                    .collect();
                for &ptr in &pointers {
                    assert_eq!(
                        (ptr.as_non_null_ptr().as_ptr() as usize) % align,
                        0,
                        "Got a pointer less aligned than requested"
                    )
                }

                // Clean up
                for &ptr in &pointers {
                    ALLOCATOR.deallocate(
                        ptr.as_non_null_ptr(),
                        Layout::from_size_align(size, align).unwrap(),
                    )
                }
            }
        }
    }
}
