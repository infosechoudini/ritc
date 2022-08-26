#![allow(dead_code)] // not used on all platforms

pub mod attr;
use core::result::Result::Err;
use crate::arch::{syscall5, syscall0, syscall3, syscall2, syscall4};
use crate::arch::nr::GETTID;
use crate::os::oserror::Error;
use core::result::Result::Ok;
use core::result::Result;
use core::prelude::rust_2024;


const CLONE_VM: usize             = 0x100;
const CLONE_FS: usize             = 0x200;
const CLONE_FILES: usize          = 0x400;
const CLONE_SIGHAND: usize        = 0x800;
const CLONE_PTRACE: usize         = 0x2000;
const CLONE_VFORK: usize          = 0x4000;
const CLONE_PARENT: usize         = 0x8000;
const CLONE_THREAD: usize         = 0x10000;
const CLONE_NEWNS: usize          = 0x20000;
const CLONE_SYSVSEM: usize        = 0x40000;
const CLONE_SETTLS: usize         = 0x80000;
const CLONE_PARENT_SETTID: usize  = 0x100000;
const CLONE_CHILD_CLEARTID: usize = 0x200000;
const CLONE_UNTRACED: usize       = 0x800000;
const CLONE_CHILD_SETTID: usize   = 0x1000000;
const CLONE_STOPPED: usize        = 0x2000000;
const CLONE_NEWUTS: usize         = 0x4000000;
const CLONE_NEWIPC: usize         = 0x8000000;

const CLONE_FLAGS: usize = CLONE_VM | CLONE_FS| CLONE_SIGHAND | CLONE_SYSVSEM | CLONE_THREAD;

const CLONE_FLAGS2: usize = CLONE_NEWUTS | CLONE_SIGHAND;

const SYS_CLONE: usize = 56;
const SYS_CLONE3: usize = 435;
const SYS_TGKILL: usize = 234;
const SYS_TKILL: usize = 200;
const SIGCHLD: usize = 17;
const SIGTERM: usize = 15;
const SIGABRT: usize = 6;
const SYS_RT_SIGPROCMASK: usize = 14;
const SIG_UNBLOCK: usize = 1;

#[rust_2024::derive(Default, Clone)]
pub struct PthreadBase {
    pub pthread: Pthread,
    pub attr: attr::PthreadAttr,
}

#[rust_2024::derive(Default, Clone)]
pub struct Pthread{
    pub thread: u64,
}


#[rust_2024::derive(Default, Clone)]
pub struct CloneArgs {
    pub flags: usize,
    pub pidfd: usize,
    pub child_tid: usize,
    pub parent_tid: usize,
    pub exit_signal: usize,
    pub stack: usize,
    pub stack_size: usize,
    pub tls: usize,
    pub set_tid: usize,
    pub set_tid_size: usize,
    pub cgroup: usize,
}

impl Pthread {

    pub fn create(&mut self) -> Pthread {
        let tid = Pthread::get_tid();
        let new_thread = self.clone(tid);
        new_thread.unwrap()
    }

    pub fn get_tid() -> usize {

        unsafe {
            syscall0(GETTID) 
        }

    }

    pub fn get_specific(key: u64) -> Pthread {
        Pthread { thread: key }
    }

    pub fn set_specific(key: u64) -> Pthread {
        Pthread { thread: key }
    }

    pub fn destroy(self) -> usize {
        unsafe { 
            let child_thread = self.thread as usize;
            // TKILL instead of TGKILL due to MUSL implementation of pthread_kill
            let ret = syscall2(SYS_TKILL, child_thread, SIGABRT);
            ret
        }
    }


    /*
    (clone, 4, CLONE_CHILD_SETTID | CLONE_CHILD_CLEARTID | SIGCHLD, 0,    
                  NULL, &THREAD_SELF->tid)
                   */
    fn clone (&mut self, tid: usize) -> Result<Pthread, Error> {
        unsafe {
            /*
                       /* Allocate memory to be used for the stack of the child. */

           stack = mmap(NULL, STACK_SIZE, PROT_READ | PROT_WRITE,
                        MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK, -1, 0);
           if (stack == MAP_FAILED)
               errExit("mmap");

           stackTop = stack + STACK_SIZE;  /* Assume stack grows downward */

           /* Create child that has its own UTS namespace;
              child commences execution in childFunc(). */

           pid = clone(childFunc, stackTop, CLONE_NEWUTS | SIGCHLD, argv[1]);
           if (pid == -1)
               errExit("clone");
           printf("clone() returned %jd\n", (intmax_t) pid);

           /* Parent falls through to here */

           sleep(1);           /* Give child time to change its hostname */

           /* Display hostname in parent's UTS namespace. This will be
              different from hostname in child's UTS namespace. */

           if (uname(&uts) == -1)
               errExit("uname");
           printf("uts.nodename in parent: %s\n", uts.nodename);

           if (waitpid(pid, NULL, 0) == -1)    /* Wait for child */
               errExit("waitpid");
           printf("child has terminated\n");

           exit(EXIT_SUCCESS);
            */


            // pid = clone(childFunc, stackTop, CLONE_NEWUTS | SIGCHLD, tid);

            let ret = syscall5(SYS_CLONE, 4, CLONE_CHILD_SETTID | CLONE_CHILD_CLEARTID | SIGCHLD, 0, 0, tid);

            match Error::demux(ret) {
                Ok(value) => Ok(Pthread { thread: value as u64 }),
                Err(value) => Err(value)
            }

        }
    }
}


