use ritc::malloc::allocator::*;
use core::alloc::Layout;
use core::ptr::*;

#[test]
fn test_basic() {
    let toy_heap = ToyHeap::default();
    let mut allocator = RawAlloc::new(toy_heap);

    const BLOCKS: usize = 3;

    let layouts: [Layout; BLOCKS] = [
        Layout::from_size_align(64, 16).unwrap(),
        Layout::from_size_align(64, 16).unwrap(),
        Layout::from_size_align(224, 16).unwrap(),
    ];

    let pointers: [*mut u8; BLOCKS] = unsafe {
        let mut pointers = [null_mut(); BLOCKS];
        for (i, &l) in layouts.iter().enumerate() {
            pointers[i] = allocator.alloc(l);
            let (validity, _stats) = allocator.stats();
            assert!(validity.is_valid());
        }
        pointers
    };

    for i in 0..BLOCKS - 1 {
        let l = layouts[i];
        let expected = unsafe { pointers[i].add(l.size()) };
        let found = pointers[i + 1];
        assert_eq!(expected, found);
    }

    let toy_heap = &allocator.grower;
    let page_size = toy_heap.page_size;
    // Toy heap should be the same size as the blocks requested
    let total_allocated: usize = layouts.iter().map(|l| l.size()).sum();
    let page_space = round_up(total_allocated, page_size);

    assert_eq!(toy_heap.size, page_space);

    ////////////////////////////////////////////////////////////
    // Deallocation

    // Deallocate the second pointer
    unsafe { allocator.dealloc(pointers[1], layouts[1]) };
    let (validity, _stats) = allocator.stats();
    assert!(validity.is_valid());

    // Check that the block list is as expected
    let mut iter = allocator.blocks.iter();
    let first = iter.next();
    assert!(first.is_some());

    let first = first.expect("This should not be null");
    assert_eq!(first.size(), layouts[1].size());
    let next_exists = iter.next().is_some();
    println!("dealloc: {}", allocator.blocks);
    // We should still have the remainder left over from the last page
    // allocation
    assert!(next_exists);

    // The block list now has 1 64-byte block on it
    println!("post-alloc: {}", allocator.blocks);
    ////////////////////////////////////////////////////////////
    // Allocation with a block list
    unsafe {
        // Allocate 112 bytes, more than fits in the block on the block list
        let newp = allocator.alloc(Layout::from_size_align(112, 16).unwrap());
        let (validity, _stats) = allocator.stats();
        assert!(validity.is_valid());
        assert_eq!(
            newp,
            pointers[2].add(round_up(layouts[2].size(), page_size))
        );
        println!("p112: {}", allocator.blocks);

        // Allocate 32 bytes, which should fit in the block
        let p32 = allocator.alloc(Layout::from_size_align(32, 16).unwrap());
        let (validity, _stats) = allocator.stats();
        assert!(validity.is_valid());
        // The algorithm returns the second half of the block
        assert_eq!(p32, pointers[1].add(32));

        // We should now still have 32 bytes in 1 block in the block list (plus page leftovers)

        // Allocate 8 bytes and another 16 bytes, which should both fit in the block
        // and completely consume it - because the 8 bytes should expand to 16
        println!("p32: {}", allocator.blocks);
        let p8 = allocator.alloc(Layout::from_size_align(16, 4).unwrap());
        let (validity, _stats) = allocator.stats();
        assert!(validity.is_valid());
        println!("p8: {}", allocator.blocks);
        let p16 = allocator.alloc(Layout::from_size_align(8, 1).unwrap());
        let (validity, _stats) = allocator.stats();
        assert!(validity.is_valid());
        // The algorithm returns the second half of the block
        println!("p16: {}", allocator.blocks);
        assert_eq!(p8, pointers[1].add(16));
        assert_eq!(p16, pointers[1]);
        println!("done: {}", allocator.blocks);
    };
}
