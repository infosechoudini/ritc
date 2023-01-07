#![feature(slice_ptr_get)]
#![feature(allocator_api)]
use ritc::malloc::bollabacator::slab::allocator::{SlabAllocator, MIN_BLOCKS};
use core::ptr::null_mut;
use std::alloc::{Allocator, Layout};

#[test_log::test]
fn test_realloc() {
    unsafe {

        let mut allocator = SlabAllocator::new();


        // Allocate a block of size 128 bytes
        let layout = Layout::from_size_align_unchecked(128, 16);
        let ptr1 = allocator.allocate(layout).unwrap();
        assert_ne!(ptr1.as_mut_ptr(), null_mut());

        // Reallocate the block to 256 bytes
        let ptr2 = allocator.realloc_impl(ptr1.as_mut_ptr(), layout, 256);
        assert_ne!(ptr2, null_mut());
        assert_eq!(ptr2 as usize, ptr1.as_mut_ptr() as usize);


        // Reallocate the block to 512 bytes
        let ptr3 = allocator.realloc_impl(ptr2, layout, 512);
        assert_ne!(ptr3, null_mut());
        assert_eq!(ptr3, ptr2);

        // Reallocate the block to 128 bytes
        let ptr4 = allocator.realloc_impl(ptr3, layout, 128);
        assert_ne!(ptr4, null_mut());
        assert_eq!(ptr4, ptr3);
    }
}

/* 

#[test_log::test]
fn test_slab_allocator() {
    let mut allocator = SlabAllocator::new();

    // Allocate a block of size 128 bytes
    let ptr = allocator.allocate(128).unwrap();
    assert_ne!(ptr.as_mut_ptr(), null_mut());

    // Deallocate the block
    allocator.deallocate(ptr);

    // Allocate a block of size 1024 bytes
    let ptr = allocator.allocate(1024).unwrap();
    assert_ne!(ptr.as_mut_ptr(), null_mut());

    // Deallocate the block
    allocator.deallocate(ptr);

    // Allocate a block of size 1025 bytes
    let ptr = allocator.allocate(1025);
    assert_eq!(ptr.is_err(), false);
}


#[test_log::test]
fn test_merge_empty_slabs() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut allocator = SlabAllocator::new();

    // Allocate a block of size 128 bytes
    let ptr1 = allocator.allocate(128).unwrap();
    assert_ne!(ptr1.as_mut_ptr(), null_mut());

    // Allocate a block of size 256 bytes
    let ptr2 = allocator.allocate(256).unwrap();
    assert_ne!(ptr2.as_mut_ptr(), null_mut());

    // Allocate a block of size 512 bytes
    let ptr3 = allocator.allocate(512).unwrap();
    assert_ne!(ptr3.as_mut_ptr(), null_mut());

    // Deallocate the block
    allocator.deallocate(ptr1);

    // Deallocate the block
    allocator.deallocate(ptr2);

    // Verify that the empty slabs were merged into a single slab
    assert_eq!(allocator.slabs.len(), 1);
    assert_eq!(allocator.slabs[0].0, MIN_BLOCKS);
    assert_eq!(allocator.slabs[0].1.len(), MIN_BLOCKS);

    // Deallocate the block
    allocator.deallocate(ptr3);

    // Verify that the empty slabs were merged into a single slab
    assert_eq!(allocator.slabs.len(), 1);
}

*/