#define _GNU_SOURCE
#include <unistd.h>
#include <sys/syscall.h>
#include <sys/appleapiopts.h>

pub const RESTART_SYSCALL        : usize = __NR_restart_syscall;
pub const EXIT                   : usize = __NR_exit;
pub const FORK                   : usize = __NR_fork;
pub const READ                   : usize = __NR_read;
pub const WRITE                  : usize = __NR_write;
pub const OPEN                   : usize = __NR_open;
pub const CLOSE                  : usize = __NR_close;
pub const CREAT                  : usize = __NR_creat;
pub const LINK                   : usize = __NR_link;
pub const UNLINK                 : usize = __NR_unlink;
pub const EXECVE                 : usize = __NR_execve;
pub const CHDIR                  : usize = __NR_chdir;
pub const TIME                   : usize = __NR_time;
pub const MKNOD                  : usize = __NR_mknod;
pub const CHMOD                  : usize = __NR_chmod;
pub const LCHOWN                 : usize = __NR_lchown;

pub const LSEEK                  : usize = __NR_lseek;
pub const GETPID                 : usize = __NR_getpid;
pub const MOUNT                  : usize = __NR_mount;
pub const UMOUNT                 : usize = __NR_umount;
pub const SETUID                 : usize = __NR_setuid;
pub const GETUID                 : usize = __NR_getuid;
pub const STIME                  : usize = __NR_stime;
pub const PTRACE                 : usize = __NR_ptrace;
pub const ALARM                  : usize = __NR_alarm;

pub const PAUSE                  : usize = __NR_pause;
pub const UTIME                  : usize = __NR_utime;

pub const ACCESS                 : usize = __NR_access;
pub const NICE                   : usize = __NR_nice;

pub const SYNC                   : usize = __NR_sync;
pub const KILL                   : usize = __NR_kill;
pub const RENAME                 : usize = __NR_rename;
pub const MKDIR                  : usize = __NR_mkdir;
pub const RMDIR                  : usize = __NR_rmdir;
pub const DUP                    : usize = __NR_dup;
pub const PIPE                   : usize = __NR_pipe;
pub const TIMES                  : usize = __NR_times;

pub const BRK                    : usize = __NR_brk;
pub const SETGID                 : usize = __NR_setgid;
pub const GETGID                 : usize = __NR_getgid;

pub const GETEUID                : usize = __NR_geteuid;
pub const GETEGID                : usize = __NR_getegid;
pub const ACCT                   : usize = __NR_acct;
pub const UMOUNT2                : usize = __NR_umount2;

pub const IOCTL                  : usize = __NR_ioctl;
pub const FCNTL                  : usize = __NR_fcntl;

pub const SETPGID                : usize = __NR_setpgid;

pub const UMASK                  : usize = __NR_umask;
pub const CHROOT                 : usize = __NR_chroot;
pub const USTAT                  : usize = __NR_ustat;
pub const DUP2                   : usize = __NR_dup2;
pub const GETPPID                : usize = __NR_getppid;
pub const GETPGRP                : usize = __NR_getpgrp;
pub const SETSID                 : usize = __NR_setsid;
pub const SIGACTION              : usize = __NR_sigaction;

pub const SETREUID               : usize = __NR_setreuid;
pub const SETREGID               : usize = __NR_setregid;
pub const SIGSUSPEND             : usize = __NR_sigsuspend;
pub const SIGPENDING             : usize = __NR_sigpending;
pub const SETHOSTNAME            : usize = __NR_sethostname;
pub const SETRLIMIT              : usize = __NR_setrlimit;
pub const GETRLIMIT              : usize = __NR_getrlimit;
pub const GETRUSAGE              : usize = __NR_getrusage;
pub const GETTIMEOFDAY           : usize = __NR_gettimeofday;
pub const SETTIMEOFDAY           : usize = __NR_settimeofday;
pub const GETGROUPS              : usize = __NR_getgroups;
pub const SETGROUPS              : usize = __NR_setgroups;
pub const SELECT                 : usize = __NR_select;
pub const SYMLINK                : usize = __NR_symlink;

pub const READLINK               : usize = __NR_readlink;
pub const USELIB                 : usize = __NR_uselib;
pub const SWAPON                 : usize = __NR_swapon;
pub const REBOOT                 : usize = __NR_reboot;
pub const READDIR                : usize = __NR_readdir;
pub const MMAP                   : usize = __NR_mmap;
pub const MUNMAP                 : usize = __NR_munmap;
pub const TRUNCATE               : usize = __NR_truncate;
pub const FTRUNCATE              : usize = __NR_ftruncate;
pub const FCHMOD                 : usize = __NR_fchmod;
pub const FCHOWN                 : usize = __NR_fchown;
pub const GETPRIORITY            : usize = __NR_getpriority;
pub const SETPRIORITY            : usize = __NR_setpriority;

pub const STATFS                 : usize = __NR_statfs;
pub const FSTATFS                : usize = __NR_fstatfs;

pub const SOCKETCALL             : usize = __NR_socketcall;
pub const SYSLOG                 : usize = __NR_syslog;
pub const SETITIMER              : usize = __NR_setitimer;
pub const GETITIMER              : usize = __NR_getitimer;
pub const STAT                   : usize = __NR_stat;
pub const LSTAT                  : usize = __NR_lstat;
pub const FSTAT                  : usize = __NR_fstat;

pub const VHANGUP                : usize = __NR_vhangup;

pub const SYSCALL                : usize = __NR_syscall;
pub const WAIT4                  : usize = __NR_wait4;
pub const SWAPOFF                : usize = __NR_swapoff;
pub const SYSINFO                : usize = __NR_sysinfo;
pub const IPC                    : usize = __NR_ipc;
pub const FSYNC                  : usize = __NR_fsync;
pub const SIGRETURN              : usize = __NR_sigreturn;
pub const CLONE                  : usize = __NR_clone;
pub const SETDOMAINNAME          : usize = __NR_setdomainname;
pub const UNAME                  : usize = __NR_uname;

pub const ADJTIMEX               : usize = __NR_adjtimex;
pub const MPROTECT               : usize = __NR_mprotect;
pub const SIGPROCMASK            : usize = __NR_sigprocmask;

pub const INIT_MODULE            : usize = __NR_init_module;
pub const DELETE_MODULE          : usize = __NR_delete_module;

pub const QUOTACTL               : usize = __NR_quotactl;
pub const GETPGID                : usize = __NR_getpgid;
pub const FCHDIR                 : usize = __NR_fchdir;
pub const BDFLUSH                : usize = __NR_bdflush;
pub const SYSFS                  : usize = __NR_sysfs;
pub const PERSONALITY            : usize = __NR_personality;

pub const SETFSUID               : usize = __NR_setfsuid;
pub const SETFSGID               : usize = __NR_setfsgid;
pub const _LLSEEK                : usize = __NR__llseek;
pub const GETDENTS               : usize = __NR_getdents;
pub const _NEWSELECT             : usize = __NR__newselect;
pub const FLOCK                  : usize = __NR_flock;
pub const MSYNC                  : usize = __NR_msync;
pub const READV                  : usize = __NR_readv;
pub const WRITEV                 : usize = __NR_writev;
pub const GETSID                 : usize = __NR_getsid;
pub const FDATASYNC              : usize = __NR_fdatasync;
pub const _SYSCTL                : usize = __NR__sysctl;
pub const MLOCK                  : usize = __NR_mlock;
pub const MUNLOCK                : usize = __NR_munlock;
pub const MLOCKALL               : usize = __NR_mlockall;
pub const MUNLOCKALL             : usize = __NR_munlockall;
pub const SCHED_SETPARAM         : usize = __NR_sched_setparam;
pub const SCHED_GETPARAM         : usize = __NR_sched_getparam;
pub const SCHED_SETSCHEDULER     : usize = __NR_sched_setscheduler;
pub const SCHED_GETSCHEDULER     : usize = __NR_sched_getscheduler;
pub const SCHED_YIELD            : usize = __NR_sched_yield;
pub const SCHED_GET_PRIORITY_MAX : usize = __NR_sched_get_priority_max;
pub const SCHED_GET_PRIORITY_MIN : usize = __NR_sched_get_priority_min;
pub const SCHED_RR_GET_INTERVAL  : usize = __NR_sched_rr_get_interval;
pub const NANOSLEEP              : usize = __NR_nanosleep;
pub const MREMAP                 : usize = __NR_mremap;
pub const SETRESUID              : usize = __NR_setresuid;
pub const GETRESUID              : usize = __NR_getresuid;

pub const POLL                   : usize = __NR_poll;
pub const NFSSERVCTL             : usize = __NR_nfsservctl;
pub const SETRESGID              : usize = __NR_setresgid;
pub const GETRESGID              : usize = __NR_getresgid;
pub const PRCTL                  : usize = __NR_prctl;
pub const RT_SIGRETURN           : usize = __NR_rt_sigreturn;
pub const RT_SIGACTION           : usize = __NR_rt_sigaction;
pub const RT_SIGPROCMASK         : usize = __NR_rt_sigprocmask;
pub const RT_SIGPENDING          : usize = __NR_rt_sigpending;
pub const RT_SIGTIMEDWAIT        : usize = __NR_rt_sigtimedwait;
pub const RT_SIGQUEUEINFO        : usize = __NR_rt_sigqueueinfo;
pub const RT_SIGSUSPEND          : usize = __NR_rt_sigsuspend;
pub const PREAD64                : usize = __NR_pread64;
pub const PWRITE64               : usize = __NR_pwrite64;
pub const CHOWN                  : usize = __NR_chown;
pub const GETCWD                 : usize = __NR_getcwd;
pub const CAPGET                 : usize = __NR_capget;
pub const CAPSET                 : usize = __NR_capset;
pub const SIGALTSTACK            : usize = __NR_sigaltstack;
pub const SENDFILE               : usize = __NR_sendfile;

pub const VFORK                  : usize = __NR_vfork;
pub const UGETRLIMIT             : usize = __NR_ugetrlimit;
pub const MMAP2                  : usize = __NR_mmap2;
pub const TRUNCATE64             : usize = __NR_truncate64;
pub const FTRUNCATE64            : usize = __NR_ftruncate64;
pub const STAT64                 : usize = __NR_stat64;
pub const LSTAT64                : usize = __NR_lstat64;
pub const FSTAT64                : usize = __NR_fstat64;
pub const LCHOWN32               : usize = __NR_lchown32;
pub const GETUID32               : usize = __NR_getuid32;
pub const GETGID32               : usize = __NR_getgid32;
pub const GETEUID32              : usize = __NR_geteuid32;
pub const GETEGID32              : usize = __NR_getegid32;
pub const SETREUID32             : usize = __NR_setreuid32;
pub const SETREGID32             : usize = __NR_setregid32;
pub const GETGROUPS32            : usize = __NR_getgroups32;
pub const SETGROUPS32            : usize = __NR_setgroups32;
pub const FCHOWN32               : usize = __NR_fchown32;
pub const SETRESUID32            : usize = __NR_setresuid32;
pub const GETRESUID32            : usize = __NR_getresuid32;
pub const SETRESGID32            : usize = __NR_setresgid32;
pub const GETRESGID32            : usize = __NR_getresgid32;
pub const CHOWN32                : usize = __NR_chown32;
pub const SETUID32               : usize = __NR_setuid32;
pub const SETGID32               : usize = __NR_setgid32;
pub const SETFSUID32             : usize = __NR_setfsuid32;
pub const SETFSGID32             : usize = __NR_setfsgid32;
pub const GETDENTS64             : usize = __NR_getdents64;
pub const PIVOT_ROOT             : usize = __NR_pivot_root;
pub const MINCORE                : usize = __NR_mincore;
pub const MADVISE                : usize = __NR_madvise;
pub const FCNTL64                : usize = __NR_fcntl64;

pub const GETTID                 : usize = __NR_gettid;
pub const READAHEAD              : usize = __NR_readahead;
pub const SETXATTR               : usize = __NR_setxattr;
pub const LSETXATTR              : usize = __NR_lsetxattr;
pub const FSETXATTR              : usize = __NR_fsetxattr;
pub const GETXATTR               : usize = __NR_getxattr;
pub const LGETXATTR              : usize = __NR_lgetxattr;
pub const FGETXATTR              : usize = __NR_fgetxattr;
pub const LISTXATTR              : usize = __NR_listxattr;
pub const LLISTXATTR             : usize = __NR_llistxattr;
pub const FLISTXATTR             : usize = __NR_flistxattr;
pub const REMOVEXATTR            : usize = __NR_removexattr;
pub const LREMOVEXATTR           : usize = __NR_lremovexattr;
pub const FREMOVEXATTR           : usize = __NR_fremovexattr;
pub const TKILL                  : usize = __NR_tkill;
pub const SENDFILE64             : usize = __NR_sendfile64;
pub const FUTEX                  : usize = __NR_futex;
pub const SCHED_SETAFFINITY      : usize = __NR_sched_setaffinity;
pub const SCHED_GETAFFINITY      : usize = __NR_sched_getaffinity;
pub const IO_SETUP               : usize = __NR_io_setup;
pub const IO_DESTROY             : usize = __NR_io_destroy;
pub const IO_GETEVENTS           : usize = __NR_io_getevents;
pub const IO_SUBMIT              : usize = __NR_io_submit;
pub const IO_CANCEL              : usize = __NR_io_cancel;
pub const EXIT_GROUP             : usize = __NR_exit_group;
pub const LOOKUP_DCOOKIE         : usize = __NR_lookup_dcookie;
pub const EPOLL_CREATE           : usize = __NR_epoll_create;
pub const EPOLL_CTL              : usize = __NR_epoll_ctl;
pub const EPOLL_WAIT             : usize = __NR_epoll_wait;
pub const REMAP_FILE_PAGES       : usize = __NR_remap_file_pages;

pub const SET_TID_ADDRESS        : usize = __NR_set_tid_address;
pub const TIMER_CREATE           : usize = __NR_timer_create;
pub const TIMER_SETTIME          : usize = __NR_timer_settime;
pub const TIMER_GETTIME          : usize = __NR_timer_gettime;
pub const TIMER_GETOVERRUN       : usize = __NR_timer_getoverrun;
pub const TIMER_DELETE           : usize = __NR_timer_delete;
pub const CLOCK_SETTIME          : usize = __NR_clock_settime;
pub const CLOCK_GETTIME          : usize = __NR_clock_gettime;
pub const CLOCK_GETRES           : usize = __NR_clock_getres;
pub const CLOCK_NANOSLEEP        : usize = __NR_clock_nanosleep;
pub const STATFS64               : usize = __NR_statfs64;
pub const FSTATFS64              : usize = __NR_fstatfs64;
pub const TGKILL                 : usize = __NR_tgkill;
pub const UTIMES                 : usize = __NR_utimes;
pub const ARM_FADVISE64_64       : usize = __NR_arm_fadvise64_64;
pub const PCICONFIG_IOBASE       : usize = __NR_pciconfig_iobase;
pub const PCICONFIG_READ         : usize = __NR_pciconfig_read;
pub const PCICONFIG_WRITE        : usize = __NR_pciconfig_write;
pub const MQ_OPEN                : usize = __NR_mq_open;
pub const MQ_UNLINK              : usize = __NR_mq_unlink;
pub const MQ_TIMEDSEND           : usize = __NR_mq_timedsend;
pub const MQ_TIMEDRECEIVE        : usize = __NR_mq_timedreceive;
pub const MQ_NOTIFY              : usize = __NR_mq_notify;
pub const MQ_GETSETATTR          : usize = __NR_mq_getsetattr;
pub const WAITID                 : usize = __NR_waitid;
pub const SOCKET                 : usize = __NR_socket;
pub const BIND                   : usize = __NR_bind;
pub const CONNECT                : usize = __NR_connect;
pub const LISTEN                 : usize = __NR_listen;
pub const ACCEPT                 : usize = __NR_accept;
pub const GETSOCKNAME            : usize = __NR_getsockname;
pub const GETPEERNAME            : usize = __NR_getpeername;
pub const SOCKETPAIR             : usize = __NR_socketpair;
pub const SEND                   : usize = __NR_send;
pub const SENDTO                 : usize = __NR_sendto;
pub const RECV                   : usize = __NR_recv;
pub const RECVFROM               : usize = __NR_recvfrom;
pub const SHUTDOWN               : usize = __NR_shutdown;
pub const SETSOCKOPT             : usize = __NR_setsockopt;
pub const GETSOCKOPT             : usize = __NR_getsockopt;
pub const SENDMSG                : usize = __NR_sendmsg;
pub const RECVMSG                : usize = __NR_recvmsg;
pub const SEMOP                  : usize = __NR_semop;
pub const SEMGET                 : usize = __NR_semget;
pub const SEMCTL                 : usize = __NR_semctl;
pub const MSGSND                 : usize = __NR_msgsnd;
pub const MSGRCV                 : usize = __NR_msgrcv;
pub const MSGGET                 : usize = __NR_msgget;
pub const MSGCTL                 : usize = __NR_msgctl;
pub const SHMAT                  : usize = __NR_shmat;
pub const SHMDT                  : usize = __NR_shmdt;
pub const SHMGET                 : usize = __NR_shmget;
pub const SHMCTL                 : usize = __NR_shmctl;
pub const ADD_KEY                : usize = __NR_add_key;
pub const REQUEST_KEY            : usize = __NR_request_key;
pub const KEYCTL                 : usize = __NR_keyctl;
pub const SEMTIMEDOP             : usize = __NR_semtimedop;
pub const VSERVER                : usize = __NR_vserver;
pub const IOPRIO_SET             : usize = __NR_ioprio_set;
pub const IOPRIO_GET             : usize = __NR_ioprio_get;
pub const INOTIFY_INIT           : usize = __NR_inotify_init;
pub const INOTIFY_ADD_WATCH      : usize = __NR_inotify_add_watch;
pub const INOTIFY_RM_WATCH       : usize = __NR_inotify_rm_watch;
pub const MBIND                  : usize = __NR_mbind;
pub const GET_MEMPOLICY          : usize = __NR_get_mempolicy;
pub const SET_MEMPOLICY          : usize = __NR_set_mempolicy;
pub const OPENAT                 : usize = __NR_openat;
pub const MKDIRAT                : usize = __NR_mkdirat;
pub const MKNODAT                : usize = __NR_mknodat;
pub const FCHOWNAT               : usize = __NR_fchownat;
pub const FUTIMESAT              : usize = __NR_futimesat;
pub const FSTATAT64              : usize = __NR_fstatat64;
pub const UNLINKAT               : usize = __NR_unlinkat;
pub const RENAMEAT               : usize = __NR_renameat;
pub const LINKAT                 : usize = __NR_linkat;
pub const SYMLINKAT              : usize = __NR_symlinkat;
pub const READLINKAT             : usize = __NR_readlinkat;
pub const FCHMODAT               : usize = __NR_fchmodat;
pub const FACCESSAT              : usize = __NR_faccessat;
pub const PSELECT6               : usize = __NR_pselect6;
pub const PPOLL                  : usize = __NR_ppoll;
pub const UNSHARE                : usize = __NR_unshare;
pub const SET_ROBUST_LIST        : usize = __NR_set_robust_list;
pub const GET_ROBUST_LIST        : usize = __NR_get_robust_list;
pub const SPLICE                 : usize = __NR_splice;
pub const ARM_SYNC_FILE_RANGE    : usize = __NR_arm_sync_file_range;
pub const SYNC_FILE_RANGE2       : usize = __NR_sync_file_range2;
pub const TEE                    : usize = __NR_tee;
pub const VMSPLICE               : usize = __NR_vmsplice;
pub const MOVE_PAGES             : usize = __NR_move_pages;
pub const GETCPU                 : usize = __NR_getcpu;
pub const EPOLL_PWAIT            : usize = __NR_epoll_pwait;
pub const KEXEC_LOAD             : usize = __NR_kexec_load;
pub const UTIMENSAT              : usize = __NR_utimensat;
pub const SIGNALFD               : usize = __NR_signalfd;
pub const TIMERFD_CREATE         : usize = __NR_timerfd_create;
pub const EVENTFD                : usize = __NR_eventfd;
pub const FALLOCATE              : usize = __NR_fallocate;
pub const TIMERFD_SETTIME        : usize = __NR_timerfd_settime;
pub const TIMERFD_GETTIME        : usize = __NR_timerfd_gettime;
pub const SIGNALFD4              : usize = __NR_signalfd4;
pub const EVENTFD2               : usize = __NR_eventfd2;
pub const EPOLL_CREATE1          : usize = __NR_epoll_create1;
pub const DUP3                   : usize = __NR_dup3;
pub const PIPE2                  : usize = __NR_pipe2;
pub const INOTIFY_INIT1          : usize = __NR_inotify_init1;
pub const PREADV                 : usize = __NR_preadv;
pub const PWRITEV                : usize = __NR_pwritev;
pub const RT_TGSIGQUEUEINFO      : usize = __NR_rt_tgsigqueueinfo;
pub const PERF_EVENT_OPEN        : usize = __NR_perf_event_open;
pub const RECVMMSG               : usize = __NR_recvmmsg;
pub const ACCEPT4                : usize = __NR_accept4;
pub const FANOTIFY_INIT          : usize = __NR_fanotify_init;
pub const FANOTIFY_MARK          : usize = __NR_fanotify_mark;
pub const PRLIMIT64              : usize = __NR_prlimit64;
pub const NAME_TO_HANDLE_AT      : usize = __NR_name_to_handle_at;
pub const OPEN_BY_HANDLE_AT      : usize = __NR_open_by_handle_at;
pub const CLOCK_ADJTIME          : usize = __NR_clock_adjtime;
pub const SYNCFS                 : usize = __NR_syncfs;
pub const SENDMMSG               : usize = __NR_sendmmsg;
pub const SETNS                  : usize = __NR_setns;
pub const PROCESS_VM_READV       : usize = __NR_process_vm_readv;
pub const PROCESS_VM_WRITEV      : usize = __NR_process_vm_writev;
pub const KCMP                   : usize = __NR_kcmp;
pub const FINIT_MODULE           : usize = __NR_finit_module;