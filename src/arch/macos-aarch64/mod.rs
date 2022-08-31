
/*
#define __NR3264_mmap 222
__SC_3264(__NR3264_mmap, sys_mmap2, sys_mmap)
*/


/* 
        mov X0, #1     // 1 = StdOut
        adr X1, helloworld // string to print
        mov X2, #13     // length of our string
        mov X16, #4     // MacOS write system call
        svc 0  

*/

/*
    /* syscall write(int fd, const void *buf, size_t count) */
    mov     x0, #1      /* fd := STDOUT_FILENO */
    ldr     x1, =msg    /* buf := msg */
    ldr     x2, =len    /* count := len */
    mov     w8, #64     /* write is syscall #64 */
    svc     #0          /* invoke syscall */
*/

use core::arch::asm;

pub mod nr;

#[inline]
pub unsafe fn syscall0(mut id: usize) -> isize {
        let ret;

            asm!(
                "svc 0",
                in("x16") id,
                out("x0") ret,
                options(nostack),
            );
            
        ret
     
}

#[inline]
pub unsafe fn syscall1(mut id: usize, a1: usize) -> isize {

    let ret;

    asm!(
        "svc #0",
        in("x16") id,
        inlateout("x0") a1 => ret,
        options(nostack),
    );
    
ret
}

#[inline]
pub unsafe fn syscall2(mut id: usize, a1: usize, a2: usize) -> isize {

    let ret;

    asm!(
        "svc 0",
        in("x16") id,
        inlateout("x0") a1 => ret,
        in("x1") a2,
        options(nostack),
    );
    
ret
}

#[inline]
pub unsafe fn syscall3(mut id: usize, a1: usize, a2: usize, a3: usize) -> isize {

        let ret;

            asm!(
                "svc 0",
                in("x16") id ,
                inlateout("x0") a1 => ret,
                in("x1") a2,
                in("x2") a3,
                options(nostack),
            );

        ret
}

#[inline]
pub unsafe fn syscall4(mut id: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize)
                       -> isize {
            let ret;

            asm!(
                "svc 0",
                in("x16") id,
                inlateout("x0") a1 => ret,
                in("x1") a2,
                in("x2") a3,
                in("x3") a4,
                options(nostack),
            );
            
        ret
}

#[inline]
pub unsafe fn syscall5(mut id: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize)
                       -> isize {

        let ret;

        asm!(
            "svc 0",
            in("x16") id,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            options(nostack),
        );
        
    ret
}

#[inline]
pub unsafe fn syscall6(mut id: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: isize,
                       a6: usize)
                       -> isize {


        let ret;

        //"r"(x8), "0"(x0), "r"(x1), "r"(x2), "r"(x3), "r"(x4), "r"(x5))
        asm!(
            "svc 0",
            in("x16") id ,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            in("x5") a6,
            options(nostack),
        );
        
    ret
}
