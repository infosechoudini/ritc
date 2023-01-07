

use alloc::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;
use core::ptr;
use crate::malloc::mmap::*;
use crate::syscall0;
use core::marker::Copy;
use core::mem;
use crate::malloc::mremap::mremap;
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
use crate::malloc::blocks::*;
use alloc::vec::Vec;

use log::{trace, info, warn};


pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
pub const ALIGN: usize = 4096;


#[derive(Debug)]
pub struct MmapAllocator{
    blocks: UnsafeCell<[(*mut u8, usize); 1000]>,
    first_block: UnsafeCell<Block>,    
    second_block: UnsafeCell<Block>,
    third_block: UnsafeCell<Block>,
    startup: bool,

}


unsafe impl Sync for MmapAllocator { }
unsafe impl Send for MmapAllocator { }

impl Drop for MmapAllocator {
  fn drop(&mut self) {

    core::mem::forget(self)

  }
}

/* 
const fn gen_array<const N: usize>() -> [Block; N] {
    let mut res = [Block::new(0); N];
    
    let mut i = 0;
    while i < N as i32 {
        res[i as usize];
        i += 1;
    }
    
    res
}
*/

const fn gen_array<const N: usize>() -> [(*mut u8, usize); N] {
    let mut res = [(ptr::null_mut(), 0); N];
    
    let mut i = 0;
    while i < N as i32 {
        res[i as usize];
        i += 1;
    }
    
    res
}


impl ConstInit for MmapAllocator{
    const INIT: MmapAllocator = MmapAllocator{
        blocks: UnsafeCell::new(gen_array()),
        first_block: UnsafeCell::new(Block::new(0)),
        second_block: UnsafeCell::new(Block::new(0)),
        third_block: UnsafeCell::new(Block::new(0)),
        startup: false,
    };
}


fn alloc_error(layout: Layout) -> ! {
    panic!("allocation failed: {:?}", layout);
}


#[inline]
pub fn round_up(value: usize, increment: usize) -> usize {
    if value == 0 {
        return 0;
    }
    increment * ((value - 1) / increment + 1)
}


pub fn concat_arrays<T, const M: usize, const N: usize>(a: [T; M], b: [T; N]) -> [T; M + N] {
    let mut result = std::mem::MaybeUninit::uninit();
    let dest = result.as_mut_ptr() as *mut T;
    unsafe {
        std::ptr::copy_nonoverlapping(a.as_ptr(), dest, M);
        std::ptr::copy_nonoverlapping(b.as_ptr(), dest.add(M), N);
        std::mem::forget(a);
        std::mem::forget(b);
        result.assume_init()
    }
}

impl MmapAllocator{

    pub const INIT: Self = <Self as ConstInit>::INIT;

    #[inline]
    pub fn fix_layout(&self, mut addr: usize, mut align: usize) ->  Layout {

        info!("Fixed Original Layout Addr: {} Align: {}", addr, align);

        let addr = self.align_up_size(addr, align);

        info!("Fixed Layout Addr: {} Align: {}", addr, align);

        assert_eq!(addr % align , 0 , "Not Good");

        Layout::from_size_align(addr, align).unwrap()
    }

    // Returns address that's aligned
    #[inline]
    pub fn align_up(&self, addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }

    // Align downwards. Returns the greatest x with alignment `align`
    // so that x <= addr. The alignment must be a power of 2.
    #[inline]
    pub fn align_down_size(&self, size: usize, align: usize) -> usize {
        if align.is_power_of_two() {
            size & !(align - 1)
        } else if align == 0 {
            size
        } else {
            panic!("`align` must be a power of 2");
        }
    }

    #[inline]
    pub fn align_up_size(&self, size: usize, align: usize) -> usize {
        self.align_down_size(size + align - 1, align)
    }

    pub unsafe fn add_free_block(&mut self, size: usize) -> (*mut u8, usize) {


        info!("Adding New Free Block");

        // roundup to nearest page size default is 4096
        let new_heap = round_up(HEAP_SIZE, ALIGN);

        info!("New Heap Block Size: {}", new_heap);

        // allocate the memory with syscall mmap to the rounded value
        let ptr = mmap(ptr::null_mut(),  new_heap);

        // return pointer to that location and the memory block size
        (ptr, new_heap)

    }

    pub unsafe fn find_free_block(&mut self, addr: usize) -> *mut u8 {

        // ensure that block size is greater than address
        if (*self.first_block.get()).block_size > addr {
            trace!("FOUND FREE BLOCK");

            // get Block mut
            let bl = self.first_block.get_mut();

            let ret_addr = bl.block_address;

            // set block address with new offset
            bl.block_address = bl.block_address.add(addr);

            // subtrack address size from block size remainder
            bl.block_size -= addr;

            // return our new memory address
            return ret_addr
        }
        
        // Allocate more memory by adding free block
        let (new_block_ptr, block_size) = self.add_free_block(addr);

        self.first_block.get_mut().block_start = new_block_ptr;

        // get block mut block address and set new free block pointer to address
        self.first_block.get_mut().block_address = new_block_ptr.add(core::mem::size_of::<Block>());

        let ret_addr = self.first_block.get_mut().block_address;

        self.first_block.get_mut().block_address = new_block_ptr.add(addr);


        if block_size == addr {
            // return block_size as remainder of free block for future use
            self.first_block.get_mut().block_size = 0;
        } else {
            // return block_size as remainder of free block for future use
            self.first_block.get_mut().block_size = block_size - addr;
        }

        // return pointer to new address
        ret_addr
        
    }




    #[inline]
    unsafe fn allocate_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {

        match layout.size() {
            // SAFETY: conditions must be upheld by the caller
            0 => unsafe {
                Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0))
            },
            size => {
                let raw_ptr = if zeroed { self.alloc_zeroed_impl(layout) } else { self.alloc_impl(layout) };
                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
            }
        }
    }

    #[inline]
    pub unsafe fn alloc_impl(&self, mut layout: Layout) -> *mut u8 {

        // if layout size is 0, nothing to do
        if layout.size() == 0 {
            return ptr::null_mut();
        }

        // Do alloc 
        self.as_ptr().as_mut().unwrap().find_free_block(layout.size())

    }

    #[inline]
    pub unsafe fn dealloc_impl(&self, ptr: *mut u8, layout: Layout) {

        // get mut Block 
        let bl = &mut self.first_block.get().as_mut().unwrap();

        // if pointer is at top of block, then we can claim the space
        // if we cant, then it falls through
        if bl.block_address == ptr  {

            // cast our new pointer to block.block_address minus the memory from the dealloc(ptr, layout)
            bl.block_address = bl.block_address.sub(layout.size());

            // add the freed up size
            bl.block_size += layout.size();

        }

        if bl.block_start != ptr {
            // remove from memory and claim it back 
            munmap::munmap(ptr, layout);
        }


    }


    #[inline]
    pub unsafe fn alloc_zeroed_impl(&self, layout: Layout) -> *mut u8 {
        
        match layout.size() {
            0 => ptr::null_mut(),
            size => {
                let raw_ptr = self.alloc_impl(layout);
                if !raw_ptr.is_null() {
                    core::ptr::write_bytes(raw_ptr, 0x00, size);
                }
                raw_ptr
            }
        }
    }

    #[inline]
    pub unsafe fn realloc_impl(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {

        trace!{"Realloc"};

        let new_addr = self.as_ptr().as_mut().unwrap().find_free_block(new_size);

        new_addr
        
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

    pub fn as_ptr(&self) -> *mut MmapAllocator {

        let mut return_self = self;
        

        &*return_self as *const MmapAllocator as *mut MmapAllocator

    }

    pub unsafe fn as_mut(&self) -> MmapAllocator {

        let r = self.as_ptr().as_mut().unwrap();

       MmapAllocator { blocks: UnsafeCell::new(*r.blocks.get_mut()), first_block: UnsafeCell::new(*r.first_block.get_mut()), second_block: UnsafeCell::new(*r.second_block.get_mut()), third_block: UnsafeCell::new(*r.third_block.get_mut()), startup: self.startup}
        
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
        trace!("RUNNING ALLOCATE ZEROED");


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
        trace!("RUNNING GROW");
        self.grow_impl(ptr, old_layout, new_layout, false)
     }

    #[inline]
    unsafe fn grow_zeroed(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 
        trace!("RUNNING GROW ZEROED");

        self.grow_impl(ptr, old_layout, new_layout, true)

     }

    #[inline]
    unsafe fn shrink(
        &self, 
        ptr: NonNull<u8>, 
        old_layout: Layout, 
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> { 
        trace!("RUNNING SHRINK");
        self.shrink_impl(ptr, old_layout, new_layout)

     }
}
