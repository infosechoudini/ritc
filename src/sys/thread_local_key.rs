#![allow(dead_code)] // not used on all platforms

use crate::sys_common::pthread;

pub type Key = pthread::Pthread;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> u64 {
    let mut key = pthread::Pthread::default();
    key.create();
    assert_eq!(key.thread, 0);
    key.thread
}


#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let r = libc::pthread_setspecific(key, value as *mut _);
    debug_assert_eq!(r, 0);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    libc::pthread_getspecific(key) as *mut u8
}

#[inline]
pub unsafe fn destroy(self, key: Key) {
    let r = libc::pthread_key_delete(key);
    debug_assert_eq!(r, 0);
}


#[inline]
pub fn requires_synchronized_create() -> bool {
    false
}
