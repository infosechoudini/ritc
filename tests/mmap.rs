use ritc::malloc::mmap::*;

#[test]
fn test_mmap() {
    let ptr = unsafe {
        mmap(
            // Address
            core::ptr::null_mut(),
            // Amount of memory to allocate
            8
        )
    };

    assert_ne!(ptr, core::ptr::null_mut());
}

