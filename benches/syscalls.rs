use criterion::{criterion_group, criterion_main, Criterion, black_box};


pub fn syscall_get_pid(c: &mut Criterion){

    c.bench_function("syscall get_pid", |b| {
        b.iter(|| {
            unsafe {
                librs::syscall0(librs::nr::GETPID)
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
                librs::syscall!(WRITE, 4, black_box(MESSAGE.as_ptr()), MESSAGE.len());              
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

criterion_group!(benches, syscall_get_pid, libc_get_pid, syscall_static_string, libc_static_string);

criterion_main!(benches);