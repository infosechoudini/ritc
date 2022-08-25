use ritc::sys::thread::Thread;
use core::str;
extern crate alloc;
use alloc::string;
use core::ffi::c_char;

#[test]
fn create_drop_threads() {
    let t = Thread::new().unwrap();
    let t_2 = Thread::new().unwrap();

    drop(t);

    drop(t_2);
}


#[test]
fn yield_thread() {
    let t = Thread::new().unwrap();
    t.yield_now();
}

#[test]
fn get_name() {
    unsafe {
        let thread_name = "Test\0";

        let mut t = Thread::new().unwrap();


        t.set_name(thread_name);

        let name_raw = t.get_name() as *const &[u8];

        //let raw = name_raw;

        //let len = strlen(raw);

        //let slice = core::slice::from_raw_parts_mut(raw as *mut u8, len);

        let name = name_raw as *const String;

        println!("NAME: {:#?}", name);    

    }
}


#[inline]
unsafe fn strlen(p: *const &[u8]) -> usize {
    let mut n = 0;
    while **p.offset(n as isize) != [0] {
        n += 1;
    }
    n
}