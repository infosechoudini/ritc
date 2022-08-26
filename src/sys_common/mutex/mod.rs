use crate::sys::locks as imp;
use crate::malloc::allocator::{HeapGrower, SyscallHeapGrower, ToyHeap, ToyHeapOverflowError, round_up};
use crate::malloc::mmap::{self, mmap, MmapError};
use core::ptr::{null_mut, NonNull};
use core::marker::Sized;
use crate::sys::locks::futex::Mutex;

/// An OS-based mutual exclusion lock, meant for use in static variables.
///
/// This mutex has a const constructor ([`StaticMutex::new`]), does not
/// implement `Drop` to cleanup resources, and causes UB when used reentrantly.
///
/// This mutex does not implement poisoning.
///
/// This is a wrapper around `imp::Mutex` that does *not* call `init()` and
/// `destroy()`.
/*pub struct StaticMutex<T: Send + Sync> {
    mutex: imp::Mutex<T>,
}

impl <T: Send + Sync> StaticMutex<T>{
    /// Creates a new mutex for use.
    pub const fn new(t: T) -> Self {
        Self{
            mutex: imp::Mutex::new(t)
        }
    }

    /// Calls raw_lock() and then returns an RAII guard to guarantee the mutex
    /// will be unlocked.
    ///
    /// It is undefined behaviour to call this function while locked by the
    /// same thread.
    #[inline]
    pub unsafe fn lock(&self) -> StaticMutexGuard<T> {
        let x = self.mutex.lock();
        StaticMutexGuard{
            mutex: imp::Mutex::new(x.to_owned()),
        }
    }
}
*/

#[must_use]
pub struct StaticMutexGuard<T: Send + Sync>{
    mutex: Mutex<T>,
}

impl <T: Send + Sync> StaticMutexGuard<T> {
    pub fn new(t: T) -> Self {
        Self{
            mutex: imp::Mutex::new(t),
        }
    }
    pub fn stats(&self) {}
    pub fn alloc(&self) {}
    pub fn dealloc(&self) {}

}


impl <T: Send + Sync> Drop for StaticMutexGuard<T>{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.mutex.unlock();
        }
    }
}

/// An OS-based mutual exclusion lock.
///
/// This mutex does *not* have a const constructor, cleans up its resources in
/// its `Drop` implementation, may safely be moved (when not borrowed), and
/// does not cause UB when used reentrantly.
///
/// This mutex does not implement poisoning.
///
/// This is either a wrapper around `Box<imp::Mutex>` or `imp::Mutex`,
/// depending on the platform. It is boxed on platforms where `imp::Mutex` may
/// not be moved.
pub struct MovableMutex<T: Send + Sync>{
    mutex: imp::Mutex<T>,
}


impl <T: Send + Sync> MovableMutex<T> {
    /// Creates a new mutex.
    pub fn new(t: T) -> Self {
        let mut mutex = imp::Mutex::from(imp::Mutex::new(t));
        unsafe { mutex.init() };
        Self{
            mutex: mutex,
        }
    }

    pub(super) fn raw(&self) -> &imp::Mutex<T> {
        &&self.mutex
    }

    /// Locks the mutex blocking the current thread until it is available.
    #[inline]
    pub fn raw_lock(&self) -> &Mutex<T>{
        unsafe { self.mutex.lock() }
    }

    /// Attempts to lock the mutex without blocking, returning whether it was
    /// successfully acquired or not.
    #[inline]
    pub fn try_lock(&self) -> bool {
        unsafe { self.mutex.try_lock() }
    }

    /// Unlocks the mutex.
    ///
    /// Behavior is undefined if the current thread does not actually hold the
    /// mutex.
    #[inline]
    pub unsafe fn raw_unlock(&self) {
        self.mutex.unlock();
    }
}

impl <T: Send + Sync> Drop for MovableMutex<T> {
    fn drop(&mut self) {
        unsafe { self.mutex.destroy() };
    }
}
