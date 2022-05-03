#[macro_use]
extern crate librs;

#[cfg(target_os = "linux")]
#[test]
fn static_string() {
    static MESSAGE: &'static str = "Hello, world!";

    unsafe {
        assert_eq!(syscall!(WRITE, 4, MESSAGE.as_ptr(), MESSAGE.len()) as isize,
                   -9)
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[test]
fn getpid() {
    unsafe {
        assert!(0 < librs::syscall0(librs::nr::GETPID));
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[test]
fn getpid_macro() {
    unsafe {
        assert!(0 < syscall!(GETPID));
    }
}