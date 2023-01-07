#![no_std]
use crate::malloc::{bollabacator::slab::blockheader::BlockHeader, mremap::mremap};

use alloc::{vec::Vec, boxed::Box};
use core::{mem, ptr::{self, NonNull}};
use alloc::alloc::{GlobalAlloc, Layout};
use core::alloc::{AllocError, Allocator};
use core::intrinsics;
use log::info;
use log::debug;

use crate::malloc::mmap::{mmap, page_size};

use super::blocklist::BlockList;


pub const BLOCK_SIZE: usize = 256; // block size in bytes
pub const MIN_BLOCKS: usize = 2; // minimum number of blocks per slab
pub const MAX_BLOCKS: usize = 16; // maximum number of blocks per slab
pub const MAGIC_SIZE: usize = 352; // possible header size


unsafe impl Sync for SlabAllocator { }
unsafe impl Send for SlabAllocator { }

#[derive(Clone, Copy)]
pub struct SlabAllocator {
    pub stack_size: usize,
    pub stack_pointer: NonNull<u8>,
    // (index, slab_pointer, blocks)
    pub slabs: (NonNull<BlockHeader>, BlockList),
}

impl SlabAllocator {

    /// Grows heap based on size needed
    /// Sends pointer to new heap
    pub unsafe fn grow_heap(&self, size: usize) -> Result<*mut u8, ()> {
        Ok(mremap(self.as_mut().stack_pointer.as_mut(), self.stack_size, size))
    }

    pub fn new() -> Self {

        unsafe {

            debug!("Got Page Size");

            let stack_pointer = NonNull::new_unchecked(mmap(ptr::null_mut(), 256));

            let block_list = BlockList::default();

            debug!("Got Stack Pointer");

            let new_slab = BlockHeader::from_raw(stack_pointer, block_list.first, 256);

            debug!("Created Slab Allocator");

            Self { 
                slabs: (new_slab, block_list),
                stack_size: BLOCK_SIZE,
                stack_pointer: stack_pointer,
            }
        }

    }

    /// Calculate the minimum size of a block to be allocated for the given layout.
    pub fn block_size(&self, layout: Layout) -> usize {
        // We align everything to 16 bytes, and all blocks are at least 16 bytes.
        // Its pretty wasteful, but easy!
        let aligned_layout = layout
            .align_to(16)
            .expect("Whoa, serious memory issues")
            .pad_to_align();

        aligned_layout.size()
    }

    pub unsafe fn alloc_impl(&mut self, layout: Layout) -> Result<NonNull<[u8]>, ()> {
        let needed_size = self.block_size(layout);

        if let Some(range) = self.slabs.1.pop_size(needed_size) {
            let ret_ptr = NonNull::new(range.start.as_ptr());
            return Ok(NonNull::slice_from_raw_parts(ret_ptr.unwrap(), needed_size));
        }

        let growth = self.grow_heap(needed_size);

        let (ptr) = match growth {
            Err(_) => return Err(()),
            Ok(res) => res,
        };

        let raw_ptr = NonNull::new(ptr).unwrap();

        if layout.size() == needed_size {
            return Ok(NonNull::slice_from_raw_parts(raw_ptr, needed_size));
        }

        let free_ptr = NonNull::new_unchecked(ptr.add(needed_size));
        if layout.size() >= needed_size + BlockList::header_size() {
            self.slabs.1.add_block(free_ptr, layout.size() - needed_size);
        } else {
            // Uh-oh. We have a bit of extra free memory, but not enough to add
            // a header and call it a new free block. This could happen if our
            // page size was not a multiple of 16. Weird.
            //
            // We have two choices here: we could return null, indicating memory
            // allocation failure, or we could leak it, and log it if possible.
            //
            // Leaking it is relatively safe, and should be uncommon; at most
            // once per page, and the only memory leaked would be smaller than
            // `header_size()`, so let's do that. Preferably, we would log it
            // too, but that would require a logging implementation that does
            // not rely on `std` or on allocation, which is not easily
            // available.
            //
            // This is not generally expected, so we add a debug_assert here.
            debug_assert!(
                false,
                "Unexpected memory left over. Is page_size a multiple of header size?"
            );
        }

        Ok(NonNull::slice_from_raw_parts(raw_ptr, needed_size))

    }

    /// Deallocate (or "free") a memory block.
    ///
    /// # Safety
    ///
    /// This is very unsafe. See GlobalAlloc for details.
    pub unsafe fn dealloc_impl(&mut self, ptr: *mut u8, layout: Layout) {
        let size = self.block_size(layout);
        self.slabs.1.add_block(NonNull::new_unchecked(ptr), size);
    }

    #[inline]
    unsafe fn allocate_impl(&mut self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
        let raw_ptr = if zeroed { self.alloc_zeroed_impl(layout) } else { self.alloc_impl(layout).unwrap().as_mut_ptr() };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }


    #[inline]
    pub unsafe fn alloc_zeroed_impl(&mut self, layout: Layout) -> *mut u8 {


        match layout.size() {
            0 => ptr::null_mut(),
            size => {
                let ptr = self.alloc_impl(layout).unwrap();
        
                ptr::write_bytes(ptr.as_mut_ptr(), 0, layout.size());

                ptr.as_mut_ptr()

            }
        }
    }

    #[inline]
    pub unsafe fn realloc_impl(&mut self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {

        let new_layout = Layout::from_size_align(new_size, layout.align()).unwrap();

        let new_ptr = self.alloc_impl(new_layout).unwrap().as_mut_ptr();

        ptr::copy_nonoverlapping(ptr, new_ptr, layout.size());

        new_ptr
    }

        // SAFETY: Same as `Allocator::grow`
        #[inline]
        unsafe fn grow_impl(
            &mut self,
            ptr: NonNull<u8>,
            old_layout: Layout,
            new_layout: Layout,
            zeroed: bool,
        ) -> Result<NonNull<[u8]>, AllocError> {
            debug_assert!(
                new_layout.size() >= old_layout.size(),
                "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
            );
    
            match old_layout.size() {
                0 => self.allocate_impl(new_layout, zeroed),
    
                // SAFETY: `new_size` is non-zero as `old_size` is greater than or equal to `new_size`
                // as required by safety conditions. Other conditions must be upheld by the caller
                old_size if old_layout.align() == new_layout.align() => unsafe {
                    let new_size = new_layout.size();
    
                    // `realloc` probably checks for `new_size >= old_layout.size()` or something similar.
                    intrinsics::assume(new_size >= old_layout.size());
    
                    let raw_ptr = self.realloc(ptr.as_ptr(), old_layout, new_size);
                    let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                    if zeroed {
                        raw_ptr.add(old_size).write_bytes(0, new_size - old_size);
                    }
                    Ok(NonNull::slice_from_raw_parts(ptr, new_size))
                },
    
                // SAFETY: because `new_layout.size()` must be greater than or equal to `old_size`,
                // both the old and new memory allocation are valid for reads and writes for `old_size`
                // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
                // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
                // for `dealloc` must be upheld by the caller.
                old_size => unsafe {
                    let new_ptr = self.allocate_impl(new_layout, zeroed)?;
                    ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), old_size);
                    self.deallocate(ptr, old_layout);
                    Ok(new_ptr)
                },
            }
        }
    
        #[inline]
        unsafe fn shrink_impl(
            &self,
            ptr: NonNull<u8>,
            old_layout: Layout,
            new_layout: Layout,
        ) -> Result<NonNull<[u8]>, AllocError> {
            debug_assert!(
                new_layout.size() <= old_layout.size(),
                "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
            );
    
            match new_layout.size() {
                // SAFETY: conditions must be upheld by the caller
                0 => unsafe {
                    self.deallocate(ptr, old_layout);
                    Ok(NonNull::slice_from_raw_parts(new_layout.dangling(), 0))
                },
    
                // SAFETY: `new_size` is non-zero. Other conditions must be upheld by the caller
                new_size if old_layout.align() == new_layout.align() => unsafe {
                    // `realloc` probably checks for `new_size <= old_layout.size()` or something similar.
                    intrinsics::assume(new_size <= old_layout.size());
    
                    let raw_ptr = self.realloc(ptr.as_ptr(), old_layout, new_size);
                    let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                    Ok(NonNull::slice_from_raw_parts(ptr, new_size))
                },
    
                // SAFETY: because `new_size` must be smaller than or equal to `old_layout.size()`,
                // both the old and new memory allocation are valid for reads and writes for `new_size`
                // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
                // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
                // for `dealloc` must be upheld by the caller.
                new_size => unsafe {
                    let new_ptr = self.allocate(new_layout)?;
                    ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), new_size);
                    self.deallocate(ptr, old_layout);
                    Ok(new_ptr)
                },
            }
        }

        pub fn as_ptr(&self) -> *mut u8 {

            let mut return_self = self;
            
    
            &*return_self as *const SlabAllocator as *mut u8
    
        }

        pub fn as_mut(&self) -> SlabAllocator {
            SlabAllocator { stack_size: self.stack_size, stack_pointer: self.stack_pointer, slabs: self.slabs }
        }
}



unsafe impl GlobalAlloc for SlabAllocator {

    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.as_mut().alloc_impl(layout).unwrap().as_mut_ptr()
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.as_mut().dealloc_impl(ptr, layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.as_mut().alloc_zeroed_impl(layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.as_mut().realloc_impl(ptr, layout, new_size)
    }

    
}

unsafe impl Allocator for SlabAllocator{
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>{

        unsafe {            
            self.as_mut().allocate_impl(layout, false)
        }

    }
    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout){

        self.dealloc(ptr.as_ptr(), layout)

    }

    #[inline]
    fn allocate_zeroed(
        &self, 
        layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> {

        unsafe {
            self.as_mut().allocate_impl(layout, true)
        }

    }
    #[inline]
    unsafe fn grow(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 
        self.as_mut().grow_impl(ptr, old_layout, new_layout, false)
     }

    #[inline]
    unsafe fn grow_zeroed(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 

        self.as_mut().grow_impl(ptr, old_layout, new_layout, true)

     }


    #[inline]
    unsafe fn shrink(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 
        
        self.shrink_impl(ptr, old_layout, new_layout)

     }
}
