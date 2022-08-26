use ritc::malloc::mmap::*;

#[test]
fn test_mmap() {
    let ptr = unsafe {
        mmap(
            // Address
            core::ptr::null_mut(),
            // Amount of memory to allocate
            8,
            // We want read/write access to this memory
            PROT_WRITE | PROT_READ,
            // Mapping flags; MAP_ANON says fd should not be 0
            MAP_ANON | MAP_PRIVATE,
            // The file descriptor we want memory mapped. We don't want a memory
            // mapped file, so 0 it is.
            0,
            0,
        )
    };

    assert!(ptr.is_ok(), "Error: {:?}", ptr.unwrap_err());
}

#[test]
fn test_mmap_err() {
    let ptr = unsafe {
        mmap(
            // Address
            core::ptr::null_mut(),
            // Amount of memory to allocate
            8,
            // We want read/write access to this memory
            PROT_WRITE | PROT_READ,
            // Mapping flags; we use 0 for now
            0,
            // The file descriptor we want memory mapped. Without MAP_ANON, this should be set.
            // This should be an error.
            0,
            0,
        )
    };

    assert!(ptr.is_err());
}
