#[macro_export]
macro_rules! syscall {
    ($nr:ident)
        => (::ritc::syscall0(
                ::ritc::nr::$nr) );

    ($nr:ident, $a1:expr)
        => ( ::ritc::syscall1(
                ::ritc::nr::$nr,
                $a1 as usize) );

    ($nr:ident, $a1:expr, $a2:expr)
        => ( ::ritc::syscall2(
                ::ritc::nr::$nr,
                $a1 as usize, $a2 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr)
        => ( ::ritc::syscall3(
                ::ritc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::ritc::syscall4(
                ::ritc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::ritc::syscall5(
                ::ritc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::ritc::syscall6(
                ::ritc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr)
        => ( ::ritc::syscall7(
                ::ritc::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize,
                $a7 as usize) );
}