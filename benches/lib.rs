#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(trusted_len)]
#![allow(unused_imports)]
#![feature(test)]

extern crate test; 


mod hash;

use ritc::malloc::mmap_allocator::MmapAllocator;
use rand::prelude::*;

#[global_allocator]
static ALLOCATOR: MmapAllocator = MmapAllocator::new();


/// Returns a `rand::Rng` seeded with a consistent seed.
///
/// This is done to avoid introducing nondeterminism in benchmark results.
pub fn bench_rng() -> rand_xorshift::XorShiftRng {
    const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    rand::SeedableRng::from_seed(SEED)
}
