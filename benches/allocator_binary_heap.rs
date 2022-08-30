#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(trusted_len)]
#![allow(unused_imports)]

use std::collections::BinaryHeap;
use ritc::malloc::mmap_allocator::MmapAllocator;
use core::alloc::Allocator;
use core::alloc::GlobalAlloc;
use rand::seq::SliceRandom;
use criterion::{black_box, criterion_group, criterion_main, Criterion};


#[global_allocator]
static ALLOCATOR: MmapAllocator = MmapAllocator::new();


fn bench_find_smallest_1000(c: &mut Criterion) {
    let mut rng = crate::bench_rng();
    let mut vec: Vec<u32> = (0..100_000).collect();
    vec.shuffle(&mut rng);

    c.bench_function("bench_find_smallest_1000", |b| 

    b.iter(|| {
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

    }));
}

/* 
fn bench_peek_mut_deref_mut(c: &mut Criterion) {
    let mut bheap = BinaryHeap::from(vec![42]);
    let vec: Vec<u32> = (0..1_000_000).collect();

    c.bench_function("bench_peek_mut_deref_mut", |b| 


    b.iter(|| {
        let vec = black_box(&vec);
        let mut peek_mut = bheap.peek_mut().unwrap();
        // The compiler shouldn't be able to optimize away the `sift_down`
        // assignment in `PeekMut`'s `DerefMut` implementation since
        // the loop might not run.
        for &i in vec.iter() {
            *peek_mut = i;
        }
        // Remove the already minimal overhead of the sift_down
        std::mem::forget(peek_mut);
    }));
}

fn bench_from_vec(c: &mut Criterion) {
    let mut rng = crate::bench_rng();
    let mut vec: Vec<u32> = (0..100_000).collect();
    vec.shuffle(&mut rng);

    c.bench_function("bench_from_vec", |b| 

    b.iter(|| BinaryHeap::from(vec.clone())));
}

fn bench_into_sorted_vec(c: &mut Criterion) {
    let bheap: BinaryHeap<i32> = (0..10_000).collect();

    c.bench_function("bench_into_sorted_vec", |b| 


    b.iter(|| bheap.clone().into_sorted_vec()));
}

fn bench_push(c: &mut Criterion) {
    let mut bheap = BinaryHeap::with_capacity(50_000);
    let mut rng = crate::bench_rng();
    let mut vec: Vec<u32> = (0..50_000).collect();
    vec.shuffle(&mut rng);

    c.bench_function("bench_push", |b| 

    b.iter(|| {
        for &i in vec.iter() {
            bheap.push(i);
        }
        black_box(&mut bheap);
        bheap.clear();
    }));
}

fn bench_pop(c: &mut Criterion) {
    let mut bheap = BinaryHeap::with_capacity(10_000);

    c.bench_function("bench_pop", |b| 

    b.iter(|| {
        bheap.extend((0..10_000).rev());
        black_box(&mut bheap);
        while let Some(elem) = bheap.pop() {
            black_box(elem);
        }
    }));
}


criterion_group!(benches, bench_pop, bench_push, bench_from_vec, bench_peek_mut_deref_mut, bench_into_sorted_vec, bench_find_smallest_1000);
*/

criterion_group!(benches, bench_find_smallest_1000);

criterion_main!(benches);




/// Returns a `rand::Rng` seeded with a consistent seed.
///
/// This is done to avoid introducing nondeterminism in benchmark results.
pub fn bench_rng() -> rand_xorshift::XorShiftRng {
    const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    rand::SeedableRng::from_seed(SEED)
}

