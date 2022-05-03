// Copyright 2022 @infosechoudini. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 Linux.

use core::arch::asm;

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> usize {
    asm!("syscall" ,
          inout("rax") n,
          out("rcx") _ , out("r11") _,
     );
     n
     
}

    /* 
    unsafe {
     asm!("out 0x64, eax", in("eax") cmd);
          }
     */


#[inline(always)]
pub unsafe fn syscall1(mut n: usize, a1: usize) -> usize {

     asm!("syscall" ,
          inout("rax") n, in("rdi") a1,
          out("rcx") _ , out("r11") _,
     );
    n
}

#[inline(always)]
pub unsafe fn syscall2(mut n: usize, a1: usize, a2: usize) -> usize {

     asm!("syscall" ,
          inout("rax") n, in("rdi") a1, in("rsi") a2,
          out("rcx") _ , out("r11") _,
     );
    n
}

#[inline(always)]
pub unsafe fn syscall3(mut n: usize, a1: usize, a2: usize, a3: usize) -> usize {


     asm!("syscall" ,
          inout("rax") n, in("rdi") a1, in("rsi") a2, in("rdx") a3,
          out("rcx") _ , out("r11") _,
     );

    n
}

#[inline(always)]
pub unsafe fn syscall4(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize)
                       -> usize {
     asm!("syscall" ,
          inout("rax") n, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4,
          out("rcx") _ , out("r11") _,
     );
    n
}

#[inline(always)]
pub unsafe fn syscall5(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize)
                       -> usize {

     asm!("syscall" ,
          inout("rax") n, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5,
          out("rcx") _ , out("r11") _,
     );
    n
}

#[inline(always)]
pub unsafe fn syscall6(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize,
                       a6: usize)
                       -> usize {

     asm!("syscall" ,
          inout("rax") n, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5,
          in("r9") a6,
          out("rcx") _ , out("r11") _,
     );
    n
}