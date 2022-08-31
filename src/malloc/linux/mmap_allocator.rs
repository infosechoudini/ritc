

use alloc::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;
use core::ptr;
use crate::malloc::mmap::*;
use crate::syscall0;
use core::marker::Copy;
use core::mem;
use crate::malloc::mremap::mremap;
use crate::malloc::mmap::MmapError;
use crate::malloc::mmap;
use core::borrow::BorrowMut;
use core::mem::MaybeUninit;
use core::ptr::{null_mut, NonNull};
use core::sync::atomic::{AtomicU8, Ordering};
use core::result::Result::Ok;
use core::result::Result::Err;
use core::result::Result;
use core::default::Default;
use core::marker::Send;
use core::marker::Sync;
use core::option::Option::Some;
use core::ops::Drop;
use core::prelude::rust_2024;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::clone::Clone;
use core::write;
use core::assert;
use core::assert_eq;
use core::debug_assert_eq;
use core::debug_assert;
use core::hint;
use core::panic;
use core::option::Option::None;
use core::cmp::Ord;
use core::option::Option;
use core::prelude::rust_2024::derive;
use core::alloc::{AllocError, Allocator};
use core::intrinsics;
use core::cell::UnsafeCell;
use crate::malloc::const_init::ConstInit;
use super::munmap;
use core::mem::drop;


pub struct MmapAllocator;


unsafe impl Sync for MmapAllocator { }
unsafe impl Send for MmapAllocator { }

impl Drop for MmapAllocator {
  fn drop(&mut self) {

    core::mem::forget(self)

  }
}


impl ConstInit for MmapAllocator{
    const INIT: MmapAllocator = MmapAllocator;
}


fn alloc_error(layout: Layout) -> ! {
    panic!("allocation failed: {:?}", layout);
}


pub fn round_up(value: usize, increment: usize) -> usize {
    if value == 0 {
        return 0;
    }
    increment * ((value - 1) / increment + 1)
}

impl MmapAllocator{

    pub const INIT: Self = <Self as ConstInit>::INIT;


    #[inline]
    pub const fn new() -> Self {
        MmapAllocator
    }

    #[inline]
    pub fn align_up(&self, addr: usize, align: usize) -> usize {
        let remainder = addr % align;
        if remainder == 0 {
            addr // addr already aligned
        } else {
            addr - remainder + align
        }
    }

    #[inline]
    unsafe fn allocate_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
        let raw_ptr = if zeroed { self.alloc_zeroed_impl(layout) } else { self.alloc_impl(layout) };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    #[inline(always)]
    pub unsafe fn alloc_impl(&self, layout: Layout) -> *mut u8 {

        match layout.size() {
            0 => ptr::null_mut(),
            size => {
                let size = self.align_up(layout.size(), layout.align()) ;

                mmap(
                    ptr::null_mut(),
                    size
                )
            }
        }
        
    }

    #[inline]
    pub unsafe fn dealloc_impl(&self, ptr: *mut u8, layout: Layout) {
        munmap::munmap(ptr, layout);
    }


    #[inline]
    pub unsafe fn alloc_zeroed_impl(&self, layout: Layout) -> *mut u8 {


        match layout.size() {
            0 => ptr::null_mut(),
            size => {
                let ptr = mmap(
                    ptr::null_mut(),
                    layout.size()
                );
        
                if !ptr.is_null() {
                    ptr::write_bytes(ptr, 0, layout.size());
                }
        
                ptr
            }
        }


    }

    #[inline]
    pub unsafe fn realloc_impl(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {

        mremap(ptr, layout.size(), new_size) 
    }

    // SAFETY: Same as `Allocator::grow`
    #[inline]
    unsafe fn grow_impl(
        &self,
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

    #[inline]
    pub unsafe fn page_size() -> usize {

        let ret = syscall0(SC_PAGE_SIZE as usize);
        ret
    }

    pub fn as_ptr(&mut self) -> *mut u8 {

        let mut return_self = self;
        

        &*return_self as *const MmapAllocator as *mut u8

    }

}


unsafe impl GlobalAlloc for MmapAllocator {

    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc_impl(layout)
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc_impl(ptr, layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.alloc_zeroed_impl(layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.realloc_impl(ptr, layout, new_size)
    }
}

unsafe impl Allocator for MmapAllocator{
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>{

        unsafe {            
            self.allocate_impl(layout, false)
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
            self.allocate_impl(layout, true)
        }

    }
    #[inline]
    unsafe fn grow(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 
        self.grow_impl(ptr, old_layout, new_layout, false)
     }

    #[inline]
    unsafe fn grow_zeroed(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 

        self.grow_impl(ptr, old_layout, new_layout, true)

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
