use crate::malloc::bollabacator::slab::blocklist::Block;


use core::ptr::NonNull;



#[repr(C, align(16))]
pub struct BlockHeader {
    pub next: Option<Block>,
    pub size: usize,
}


impl BlockHeader {
    pub unsafe fn from_raw(
        ptr: NonNull<u8>,
        next: Option<Block>,
        size: usize,
    ) -> NonNull<BlockHeader> {
        let header = BlockHeader { next, size };
        let raw_ptr: NonNull<BlockHeader> = ptr.cast();
        core::ptr::write(ptr.as_ptr() as *mut BlockHeader, header);
        raw_ptr
    }
}