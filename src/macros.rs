#[macro_export]
macro_rules! syscall {
    ($nr:ident)
        => ( ::librs::syscall0(
                ::librs::nr::$nr) );

    ($nr:ident, $a1:expr)
        => ( ::librs::syscall1(
                ::librs::nr::$nr,
                $a1 as usize) );

    ($nr:ident, $a1:expr, $a2:expr)
        => ( ::librs::syscall2(
                ::librs::nr::$nr,
                $a1 as usize, $a2 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr)
        => ( ::librs::syscall3(
                ::librs::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::librs::syscall4(
                ::librs::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::librs::syscall5(
                ::librs::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::librs::syscall6(
                ::librs::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr)
        => ( ::librs::syscall7(
                ::librs::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize,
                $a7 as usize) );
}