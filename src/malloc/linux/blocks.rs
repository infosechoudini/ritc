use core::ptr;


#[derive(Clone, Copy)]
pub struct Block{
    pub free: bool,
    pub block_size: usize,
    pub block_start: *mut u8,
    pub block_address: *mut u8,
    pub next: Option<usize>
}

unsafe impl Sync for Block{}
unsafe impl Send for Block{}



impl Block{
    pub const fn new(size: usize) -> Self {
        Block {
            free: true, 
            block_size: size, 
            block_start: ptr::null_mut(),
            block_address: ptr::null_mut(), 
            next: None,
        }
        
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn end_addr(&self) -> usize {
        self.start_addr() + self.block_size
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {

        let mut b = Block {
            free: self.free, 
            block_start: self.block_start,
            block_size: self.block_size, 
            block_address: self.block_address, 
            next: None,
        };

        &b as *const _ as *mut u8

    }

    pub unsafe fn as_block(mut_block: &mut *mut u8) -> Self {

        let b = (*mut_block as *mut Block);

        Block { free: (*b).free, block_size: (*b).block_size, block_address: (*b).block_address, next: (*b).next, block_start: (*b).block_start }

    }

    
}

