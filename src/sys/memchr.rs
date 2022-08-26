use core::mem;
use core::cmp;
use core::marker::Copy;
use core::option::Option::Some;
use core::option::Option;
use core::result::Result::Err;
use core::result::Result::Ok;
use core::marker::Sync;
use core::marker::Send;
use core::option::Option::None;

const LO_U64: u64 = 0x0101010101010101;
const HI_U64: u64 = 0x8080808080808080;

// Use truncation.
const LO_USIZE: usize = LO_U64 as usize;
const HI_USIZE: usize = HI_U64 as usize;
const USIZE_BYTES: usize = mem::size_of::<usize>();


#[inline]
pub fn memchr(x: u8, text: &[u8]) -> Option<usize> {
    // Fast path for small slices
    if text.len() < 2 * USIZE_BYTES {
        return text.iter().position(|elt| *elt == x);
    }

    memchr_general_case(x, text)
}

fn memchr_general_case(x: u8, text: &[u8]) -> Option<usize> {
    // Scan for a single byte value by reading two `usize` words at a time.
    //
    // Split `text` in three parts
    // - unaligned initial part, before the first word aligned address in text
    // - body, scan by 2 words at a time
    // - the last remaining part, < 2 word size

    // search up to an aligned boundary
    let len = text.len();
    let ptr = text.as_ptr();
    let mut offset = ptr.align_offset(USIZE_BYTES);

    if offset > 0 {
        offset = cmp::min(offset, len);
        if let Some(index) = text[..offset].iter().position(|elt| *elt == x) {
            return Some(index);
        }
    }

    // search the body of the text
    let repeated_x = repeat_byte(x);
    while offset <= len - 2 * USIZE_BYTES {
        // SAFETY: the while's predicate guarantees a distance of at least 2 * usize_bytes
        // between the offset and the end of the slice.
        unsafe {
            let u = *(ptr.add(offset) as *const usize);
            let v = *(ptr.add(offset + USIZE_BYTES) as *const usize);

            // break if there is a matching byte
            let zu = contains_zero_byte(u ^ repeated_x);
            let zv = contains_zero_byte(v ^ repeated_x);
            if zu || zv {
                break;
            }
        }
        offset += USIZE_BYTES * 2;
    }

    // Find the byte after the point the body loop stopped.
    text[offset..].iter().position(|elt| *elt == x).map(|i| offset + i)
}


#[inline]
fn repeat_byte(b: u8) -> usize {
    (b as usize) << 8 | b as usize
}

#[inline]
fn contains_zero_byte(x: usize) -> bool {
    x.wrapping_sub(LO_USIZE) & !x & HI_USIZE != 0
}