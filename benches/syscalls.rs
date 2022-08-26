use criterion::{criterion_group, criterion_main, Criterion, black_box};
use core::ptr;
use core::mem;

pub fn syscall_get_pid(c: &mut Criterion){

    c.bench_function("syscall get_pid", |b| {
        b.iter(|| {
            unsafe {
                ritc::syscall0(ritc::nr::GETPID)
            }
        })
    });
}

pub fn libc_get_pid(c: &mut Criterion){

    c.bench_function("libc get_pid", |b| {
        b.iter(|| {
            unsafe {
                libc::syscall(libc::SYS_getpid)
            }
        })
    });
}

pub fn syscall_static_string(c: &mut Criterion) {
    static MESSAGE: &'static str = "Hello, world!";

    c.bench_function("syscall write string", | b | {
        b.iter( || {

            unsafe {
                ritc::syscall!(WRITE, 4, black_box(MESSAGE.as_ptr()), MESSAGE.len());              
            }
        })
    });
}

pub fn libc_static_string(c: &mut Criterion) {
    static MESSAGE: &'static str = "Hello, world!";

    c.bench_function("libc write string", | b | {
        b.iter( || {

            unsafe {
                libc::write(4, black_box(MESSAGE.as_ptr() as *const libc::c_void), MESSAGE.len());              
            }
        })
    });
}

pub fn ritc_create_thread(c: &mut Criterion){


    c.bench_function("ritc create_thread", |b| {
        b.iter(|| {

            let t = ritc::sys_common::pthread::Pthread::default().create();
            t.destroy();
        })
    });
}

pub fn libc_create_thread(c: &mut Criterion){

    c.bench_function("libc create_thread", |b| {
        b.iter(|| {
            unsafe {

                let mut thread = 0;
                libc::pthread_create(&mut thread, // out-argument for pthread_t
                    ptr::null(),                  // in-argument of pthread_attr_t
                    thread_start,                  // thread body (as a C callback)
                    mem::transmute(&"junk"),         // thread argument (requires a cast)
                );
            }
        })
    });
}

extern "C" fn thread_start(main: *mut libc::c_void) -> *mut libc::c_void {
    ptr::null_mut()
}

criterion_group!{name=benches;
        config = Criterion::default();
        targets=syscall_get_pid, libc_get_pid, syscall_static_string, libc_static_string
    }

criterion_main!(benches);