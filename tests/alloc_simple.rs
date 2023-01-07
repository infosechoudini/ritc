#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(trusted_len)]
#![allow(unused_imports)]
#![feature(strict_provenance)]
#![feature(layout_for_ptr)]
#![feature(extend_one)]

//#[global_allocator]
static ALLOCATOR: MmapAllocator = MmapAllocator::INIT;


use test_log::test;

use core::alloc::{Allocator, Layout, GlobalAlloc};
use std::alloc::Global;
use std::mem::MaybeUninit;
use libc::realloc;
use ritc::malloc::mmap_allocator::MmapAllocator;
use ritc::malloc::blocks::Block;
extern crate alloc;
use alloc::vec::Vec;
use std::thread;
use std::collections::BinaryHeap;
use rand::seq::SliceRandom;

#[test]
fn allocator_alloc_dealloc() {
    unsafe {
        let layout = Layout::from_size_align(40, 8).unwrap();

        println!("LAYOUT {:?} {:?}", layout.size(), layout.align());

        let alloc_zero = ALLOCATOR.allocate_zeroed(layout.clone()).unwrap();

        println!("ALLOCATE ZEROED POINTER ADDRESS: {:#?}", &alloc_zero.as_mut_ptr().addr());

        assert_ne!(alloc_zero.as_mut_ptr(), core::ptr::null_mut());

        println!("SIZE: {}", alloc_zero.len());

        assert_eq!(alloc_zero.len(), layout.size());

        assert_eq!(core::ptr::read(alloc_zero.as_mut_ptr()), 0);


        ALLOCATOR.deallocate(alloc_zero.as_non_null_ptr(), layout);

        let alloc_reg = ALLOCATOR.allocate(layout.clone()).unwrap();

        let alloc_reg2 = ALLOCATOR.allocate(layout.clone()).unwrap();


        println!("ALLOCATE POINTER ADDRESS: {:#?}", &alloc_reg.as_mut_ptr().addr());

        println!("SIZE: {}", alloc_reg.len());

        assert_eq!(alloc_reg.len(), layout.size());

        assert_ne!(alloc_reg.as_mut_ptr().addr(), alloc_reg2.as_mut_ptr().addr());

        let diff = alloc_reg2.as_mut_ptr() as usize - alloc_reg.as_mut_ptr() as usize;

        println!("DIFF: {}", diff);

        assert_eq!(diff, layout.size());

        assert_eq!(alloc_zero.as_mut_ptr().addr(), alloc_reg.as_mut_ptr().addr());

        ALLOCATOR.deallocate(alloc_reg.as_non_null_ptr(), layout);


        let alloc_reg = ALLOCATOR.allocate(layout.clone()).unwrap();

        assert_eq!(alloc_reg.len() , layout.size());




        let new_size = 512;

        let new_layout = Layout::from_size_align(new_size, layout.align()).unwrap();

        let realloc_reg = ALLOCATOR.grow(alloc_reg.as_non_null_ptr(), layout, new_layout).unwrap();

        assert_ne!(core::ptr::null_mut(), realloc_reg.as_mut_ptr());

        let len = realloc_reg.len();

        assert_eq!(new_size, len);
        


    }

}



/* 
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

*/

pub fn test_rng() -> rand_xorshift::XorShiftRng {
    const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    rand::SeedableRng::from_seed(SEED)
}
