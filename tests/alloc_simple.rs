#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(trusted_len)]
#![allow(unused_imports)]

#[global_allocator]
static ALLOCATOR: MmapAllocator = MmapAllocator::new();

use core::alloc::{Allocator, Layout, GlobalAlloc};
use std::alloc::Global;
use ritc::malloc::mmap_allocator::MmapAllocator;
extern crate alloc;
use alloc::vec::Vec;
use std::thread;
use std::collections::BinaryHeap;
use rand::seq::SliceRandom;


#[test]
fn allocator_twice() {
    unsafe {
        let layout = Layout::from_size_align(400000 * 8, 8).unwrap();
        let item = Global.allocate(layout.clone()).unwrap();
        Global.deallocate(item.as_non_null_ptr(), layout);

        let item = Global.allocate(layout.clone()).unwrap();
        Global.deallocate(item.as_non_null_ptr(), layout);

    }

}

#[test]
fn allocator_thread() {

    let mut vec_hold = Vec::new();
    for x in 0..5 {
        let thread_box = Box::new(x);
        let thread = thread::spawn( move || {
            thread_box;
        });

        vec_hold.push(thread);
    }

    assert_eq!(vec_hold.len(), 5)

}


#[test]
fn box_heap_find_smallest_1000() {
    let mut rng = test_rng();
    let mut vec: Vec<u32> = (0..100_000).collect();
    vec.shuffle(&mut rng);


    for _ in 0..1000{

        let mut iter = vec.iter().copied();
        let mut heap: BinaryHeap<_> = iter.by_ref().take(1000).collect();

        for x in iter {
            let mut max = heap.peek_mut().unwrap();
            // This comparison should be true only 1% of the time.
            // Unnecessary `sift_down`s will degrade performance
            if x < *max {
                *max = x;
            }
        }

    }
}



pub fn test_rng() -> rand_xorshift::XorShiftRng {
    const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    rand::SeedableRng::from_seed(SEED)
}
