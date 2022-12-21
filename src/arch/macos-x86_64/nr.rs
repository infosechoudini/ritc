
pub const	SYS_syscall : usize=   0;
pub const	SYS_exit : usize=      1;
pub const	SYS_fork : usize=      2;
pub const	SYS_read : usize=      3;
pub const	SYS_write : usize=     4;
pub const	SYS_open : usize=      5;
pub const	SYS_close : usize=     6;
pub const	SYS_wait4 : usize=     7;
			/* 8  old creat **/
pub const	SYS_link : usize=      9;
pub const	SYS_unlink : usize=    10;
			/* 11  old execv **/
pub const	SYS_chdir : usize=     12;
pub const	SYS_fchdir : usize=    13;
pub const	SYS_mknod : usize=     14;
pub const	SYS_chmod : usize=     15;
pub const	SYS_chown : usize=     16;
			/* 17  old break **/
pub const	SYS_getfsstat : usize= 18;
			/* 19  old lseek **/
pub const	SYS_getpid : usize=    20;
			/* 21  old mount **/
			/* 22  old umount **/
pub const	SYS_setuid : usize=    23;
pub const	SYS_getuid : usize=    24;
pub const	SYS_geteuid : usize=   25;
pub const	SYS_ptrace : usize=    26;
pub const	SYS_recvmsg : usize=   27;
pub const	SYS_sendmsg : usize=   28;
pub const	SYS_recvfrom : usize=  29;
pub const	SYS_accept : usize=    30;
pub const	SYS_getpeername : usize  = 31;
pub const	SYS_getsockname : usize  = 32;
pub const	SYS_access : usize=    33;
pub const	SYS_chflags : usize=   34;
pub const	SYS_fchflags : usize=  35;
pub const	SYS_sync : usize=      36;
pub const	SYS_kill : usize=      37;
			/* 38  old stat **/
pub const	SYS_getppid : usize=   39;
			/* 40  old lstat **/
pub const	SYS_dup : usize = 41;
pub const	SYS_pipe : usize =      42;
pub const	SYS_getegid : usize=   43;
			/* 44  old profil **/
			/* 45  old ktrace **/
pub const	SYS_sigaction : usize= 46;
pub const	SYS_getgid : usize=    47;
pub const	SYS_sigprocmask : usize =  48;
pub const	SYS_getlogin : usize=  49;
pub const	SYS_setlogin : usize=  50;
pub const	SYS_acct : usize=      51;
pub const	SYS_sigpending : usize  =  52;
pub const	SYS_sigaltstack : usize =  53;
pub const	SYS_ioctl : usize=     54;
pub const	SYS_reboot : usize=    55;
pub const	SYS_revoke : usize=    56;
pub const	SYS_symlink : usize=   57;
pub const	SYS_readlink : usize=  58;
pub const	SYS_execve : usize=    59;
pub const	SYS_umask : usize=     60;
pub const	SYS_chroot : usize=    61;
			/* 62  old fstat **/
			/* 63  used internally and reserved **/
			/* 64  old getpagesize **/
pub const	SYS_msync : usize=     65;
pub const	SYS_vfork : usize=     66;
			/* 67  old vread **/
			/* 68  old vwrite **/
			/* 69  old sbrk **/
			/* 70  old sstk **/
			/* 71  old mmap **/
			/* 72  old vadvise **/
pub const	SYS_munmap : usize=    73;
pub const	SYS_mprotect : usize=  74;
pub const	SYS_madvise : usize=   75;
			/* 76  old vhangup **/
			/* 77  old vlimit **/
pub const	SYS_mincore : usize=   78;
pub const	SYS_getgroups : usize= 79;
pub const	SYS_setgroups : usize= 80;
pub const	SYS_getpgrp : usize=   81;
pub const	SYS_setpgid : usize=   82;
pub const	SYS_setitimer : usize= 83;
			/* 84  old wait **/
pub const	SYS_swapon : usize=    85;
pub const	SYS_getitimer : usize= 86;
			/* 87  old gethostname **/
			/* 88  old sethostname **/
pub const	SYS_getdtablesize : usize = 89;
pub const	SYS_dup2 : usize=      90;
			/* 91  old getdopt **/
pub const	SYS_fcntl : usize=     92;
pub const	SYS_select : usize=    93;
			/* 94  old setdopt **/
pub const	SYS_fsync : usize=     95;
pub const	SYS_setpriority : usize = 96;
pub const	SYS_socket : usize=    97;
pub const	SYS_connect : usize=   98;
			/* 99  old accept **/
pub const	SYS_getpriority : usize = 100;
			/* 101  old send **/
			/* 102  old recv **/
			/* 103  old sigreturn **/
pub const	SYS_bind : usize=      104;
pub const	SYS_setsockopt : usize = 105;
pub const	SYS_listen : usize=    106;
			/* 107  old vtimes **/
			/* 108  old sigvec **/
			/* 109  old sigblock **/
			/* 110  old sigsetmask **/
pub const	SYS_sigsuspend : usize = 111;
			/* 112  old sigstack **/
			/* 113  old recvmsg **/
			/* 114  old sendmsg **/
			/* 115  old vtrace **/
pub const	SYS_gettimeofday : usize = 116;
pub const	SYS_getrusage : usize= 117;
pub const	SYS_getsockopt : usize = 118;
			/* 119  old resuba **/
pub const	SYS_readv : usize=     120;
pub const	SYS_writev : usize=    121;
pub const	SYS_settimeofday : usize = 122;
pub const	SYS_fchown : usize=    123;
pub const	SYS_fchmod : usize=    124;
			/* 125  old recvfrom **/
pub const	SYS_setreuid : usize=  126;
pub const	SYS_setregid : usize=  127;
pub const	SYS_rename : usize=    128;
			/* 129  old truncate **/
			/* 130  old ftruncate **/
pub const	SYS_flock : usize=     131;
pub const	SYS_mkfifo : usize=    132;
pub const	SYS_sendto : usize=    133;
pub const	SYS_shutdown : usize=  134;
pub const	SYS_socketpair : usize = 135;
pub const	SYS_mkdir : usize=     136;
pub const	SYS_rmdir : usize=     137;
pub const	SYS_utimes : usize=    138;
pub const	SYS_futimes : usize=   139;
pub const	SYS_adjtime : usize=   140;
			/* 141  old getpeername **/
pub const	SYS_gethostuuid : usize = 142;
			/* 143  old sethostid **/
			/* 144  old getrlimit **/
			/* 145  old setrlimit **/
			/* 146  old killpg **/
pub const	SYS_setsid : usize=    147;
			/* 148  old setquota **/
			/* 149  old qquota **/
			/* 150  old getsockname **/
pub const	SYS_getpgid : usize=   151;
pub const	SYS_setprivexec : usize = 152;
pub const	SYS_pread : usize=     153;
pub const	SYS_pwrite : usize=    154;
pub const	SYS_nfssvc : usize=    155;
			/* 156  old getdirentries **/
pub const	SYS_statfs : usize=    157;
pub const	SYS_fstatfs : usize=   158;
pub const	SYS_unmount : usize=   159;
			/* 160  old async_daemon **/
pub const	SYS_getfh : usize=     161;
			/* 162  old getdomainname **/
			/* 163  old setdomainname **/
			/* 164  **/
pub const	SYS_quotactl : usize=  165;
			/* 166  old exportfs **/
pub const	SYS_mount : usize=     167;
			/* 168  old ustat **/
pub const	SYS_csops : usize=     169;
pub const	SYS_csops_audittoken : usize = 170;
			/* 171  old wait3 **/
			/* 172  old rpause **/
pub const	SYS_waitid : usize=    173;
			/* 174  old getdents **/
			/* 175  old gc_control **/
			/* 176  old add_profil **/
pub const	SYS_kdebug_typefilter : usize = 177;
pub const	SYS_kdebug_trace_string : usize = 178;
pub const	SYS_kdebug_trace64 : usize = 179;
pub const	SYS_kdebug_trace : usize = 180;
pub const	SYS_setgid : usize=    181;
pub const	SYS_setegid : usize=   182;
pub const	SYS_seteuid : usize=   183;
pub const	SYS_sigreturn : usize= 184;
			/* 185  old chud **/
pub const	SYS_thread_selfcounts : usize = 186;
pub const	SYS_fdatasync : usize= 187;
pub const	SYS_stat : usize=      188;
pub const	SYS_fstat : usize=     189;
pub const	SYS_lstat : usize=     190;
pub const	SYS_pathconf : usize=  191;
pub const	SYS_fpathconf : usize= 192;
			/* 193  old getfsstat **/
pub const	SYS_getrlimit : usize= 194;
pub const	SYS_setrlimit : usize= 195;
pub const	SYS_getdirentries : usize = 196;
pub const	SYS_mmap : usize=      197;
			/* 198  old __syscall : usize**/
pub const	SYS_lseek : usize=     199;
pub const	SYS_truncate : usize=  200;
pub const	SYS_ftruncate : usize= 201;
pub const	SYS_sysctl : usize=    202;
pub const	SYS_mlock : usize=     203;
pub const	SYS_munlock : usize=   204;
pub const	SYS_undelete : usize=  205;
			/* 206  old ATsocket **/
			/* 207  old ATgetmsg **/
			/* 208  old ATputmsg **/
			/* 209  old ATsndreq **/
			/* 210  old ATsndrsp **/
			/* 211  old ATgetreq **/
			/* 212  old ATgetrsp **/
			/* 213  Reserved for AppleTalk **/
			/* 214  **/
			/* 215  **/
pub const	SYS_open_dprotected_np : usize = 216;
pub const	SYS_fsgetpath_ext : usize = 217;
			/* 218  old lstatv **/
			/* 219  old fstatv **/
pub const	SYS_getattrlist : usize = 220;
pub const	SYS_setattrlist : usize = 221;
pub const	SYS_getdirentriesattr : usize = 222;
pub const	SYS_exchangedata : usize = 223;
			/* 224  old checkuseraccess or fsgetpath **/
pub const	SYS_searchfs : usize=  225;
pub const	SYS_delete : usize=    226;
pub const	SYS_copyfile : usize=  227;
pub const	SYS_fgetattrlist : usize = 228;
pub const	SYS_fsetattrlist : usize = 229;
pub const	SYS_poll : usize=      230;
			/* 231  old watchevent **/
			/* 232  old waitevent **/
			/* 233  old modwatch **/
pub const	SYS_getxattr : usize=  234;
pub const	SYS_fgetxattr : usize= 235;
pub const	SYS_setxattr : usize=  236;
pub const	SYS_fsetxattr : usize= 237;
pub const	SYS_removexattr : usize = 238;
pub const	SYS_fremovexattr : usize = 239;
pub const	SYS_listxattr : usize= 240;
pub const	SYS_flistxattr : usize = 241;
pub const	SYS_fsctl : usize=     242;
pub const	SYS_initgroups : usize = 243;
pub const	SYS_posix_spawn : usize = 244;
pub const	SYS_ffsctl : usize=    245;
			/* 246  **/
pub const	SYS_nfsclnt : usize=   247;
pub const	SYS_fhopen : usize=    248;
			/* 249  **/
pub const	SYS_minherit : usize=  250;
pub const	SYS_semsys : usize=    251;
pub const	SYS_msgsys : usize=    252;
pub const	SYS_shmsys : usize=    253;
pub const	SYS_semctl : usize=    254;
pub const	SYS_semget : usize=    255;
pub const	SYS_semop : usize=     256;
			/* 257  old semconfig **/
pub const	SYS_msgctl : usize=    258;
pub const	SYS_msgget : usize=    259;
pub const	SYS_msgsnd : usize=    260;
pub const	SYS_msgrcv : usize=    261;
pub const	SYS_shmat : usize=     262;
pub const	SYS_shmctl : usize=    263;
pub const	SYS_shmdt : usize=     264;
pub const	SYS_shmget : usize=    265;
pub const	SYS_shm_open : usize=  266;
pub const	SYS_shm_unlink : usize = 267;
pub const	SYS_sem_open : usize=  268;
pub const	SYS_sem_close : usize= 269;
pub const	SYS_sem_unlink : usize = 270;
pub const	SYS_sem_wait : usize=  271;
pub const	SYS_sem_trywait : usize = 272;
pub const	SYS_sem_post : usize=  273;
pub const	SYS_sysctlbyname : usize = 274;
			/* 275  old sem_init **/
			/* 276  old sem_destroy **/
pub const	SYS_open_extended : usize = 277;
pub const	SYS_umask_extended : usize = 278;
pub const	SYS_stat_extended : usize = 279;
pub const	SYS_lstat_extended : usize = 280;
pub const	SYS_fstat_extended : usize = 281;
pub const	SYS_chmod_extended : usize = 282;
pub const	SYS_fchmod_extended : usize = 283;
pub const	SYS_access_extended : usize = 284;
pub const	SYS_settid : usize=    285;
pub const	SYS_gettid : usize=    286;
pub const	SYS_setsgroups : usize = 287;
pub const	SYS_getsgroups : usize = 288;
pub const	SYS_setwgroups : usize = 289;
pub const	SYS_getwgroups : usize = 290;
pub const	SYS_mkfifo_extended : usize = 291;
pub const	SYS_mkdir_extended : usize = 292;
pub const	SYS_identitysvc : usize = 293;
pub const	SYS_shared_region_check_np : usize = 294;
			/* 295  old shared_region_map_np **/
pub const	SYS_vm_pressure_monitor : usize = 296;
pub const	SYS_psynch_rw_longrdlock : usize = 297;
pub const	SYS_psynch_rw_yieldwrlock : usize = 298;
pub const	SYS_psynch_rw_downgrade : usize = 299;
pub const	SYS_psynch_rw_upgrade : usize = 300;
pub const	SYS_psynch_mutexwait : usize = 301;
pub const	SYS_psynch_mutexdrop : usize = 302;
pub const	SYS_psynch_cvbroad : usize = 303;
pub const	SYS_psynch_cvsignal : usize = 304;
pub const	SYS_psynch_cvwait : usize = 305;
pub const	SYS_psynch_rw_rdlock : usize = 306;
pub const	SYS_psynch_rw_wrlock : usize = 307;
pub const	SYS_psynch_rw_unlock : usize = 308;
pub const	SYS_psynch_rw_unlock2 : usize = 309;
pub const	SYS_getsid : usize=    310;
pub const	SYS_settid_with_pid : usize = 311;
pub const	SYS_psynch_cvclrprepost : usize = 312;
pub const	SYS_aio_fsync : usize= 313;
pub const	SYS_aio_return : usize = 314;
pub const	SYS_aio_suspend : usize = 315;
pub const	SYS_aio_cancel : usize = 316;
pub const	SYS_aio_error : usize= 317;
pub const	SYS_aio_read : usize=  318;
pub const	SYS_aio_write : usize= 319;
pub const	SYS_lio_listio : usize = 320;
			/* 321  old __pthread_cond_wait **/
pub const	SYS_iopolicysys : usize = 322;
pub const	SYS_process_policy : usize = 323;
pub const	SYS_mlockall : usize=  324;
pub const	SYS_munlockall : usize = 325;
			/* 326  **/
pub const	SYS_issetugid : usize= 327;
pub const	SYS___pthread_kill : usize = 328;
pub const	SYS___pthread_sigmask : usize = 329;
pub const	SYS___sigwait : usize= 330;
pub const	SYS___disable_threadsignal : usize = 331;
pub const	SYS___pthread_markcancel : usize = 332;
pub const	SYS___pthread_canceled : usize = 333;
pub const	SYS___semwait_signal : usize = 334;
			/* 335  old utrace **/
pub const	SYS_proc_info : usize= 336;
pub const	SYS_sendfile : usize=  337;
pub const	SYS_stat64 : usize=    338;
pub const	SYS_fstat64 : usize=   339;
pub const	SYS_lstat64 : usize=   340;
pub const	SYS_stat64_extended : usize = 341;
pub const	SYS_lstat64_extended : usize = 342;
pub const	SYS_fstat64_extended : usize = 343;
pub const	SYS_getdirentries64 : usize = 344;
pub const	SYS_statfs64 : usize=  345;
pub const	SYS_fstatfs64 : usize= 346;
pub const	SYS_getfsstat64 : usize = 347;
pub const	SYS___pthread_chdir : usize = 348;
pub const	SYS___pthread_fchdir : usize = 349;
pub const	SYS_audit : usize=     350;
pub const	SYS_auditon : usize=   351;
			/* 352  **/
pub const	SYS_getauid : usize=   353;
pub const	SYS_setauid : usize=   354;
			/* 355  old getaudit **/
			/* 356  old setaudit **/
pub const	SYS_getaudit_addr : usize = 357;
pub const	SYS_setaudit_addr : usize = 358;
pub const	SYS_auditctl : usize=  359;
pub const	SYS_bsdthread_create : usize = 360;
pub const	SYS_bsdthread_terminate : usize = 361;
pub const	SYS_kqueue : usize=    362;
pub const	SYS_kevent : usize=    363;
pub const	SYS_lchown : usize=    364;
			/* 365  old stack_snapshot **/
pub const	SYS_bsdthread_register : usize = 366;
pub const	SYS_workq_open : usize = 367;
pub const	SYS_workq_kernreturn : usize = 368;
pub const	SYS_kevent64 : usize=  369;
pub const	SYS___old_semwait_signal : usize = 370;
pub const	SYS___old_semwait_signal_nocancel : usize = 371;
pub const	SYS_thread_selfid : usize = 372;
pub const	SYS_ledger : usize=    373;
pub const	SYS_kevent_qos : usize = 374;
pub const	SYS_kevent_id : usize= 375;
			/* 376  **/
			/* 377  **/
			/* 378  **/
			/* 379  **/
pub const	SYS___mac_execve : usize = 380;
pub const	SYS___mac_syscall : usize = 381;
pub const	SYS___mac_get_file : usize = 382;
pub const	SYS___mac_set_file : usize = 383;
pub const	SYS___mac_get_link : usize = 384;
pub const	SYS___mac_set_link : usize = 385;
pub const	SYS___mac_get_proc : usize = 386;
pub const	SYS___mac_set_proc : usize = 387;
pub const	SYS___mac_get_fd : usize = 388;
pub const	SYS___mac_set_fd : usize = 389;
pub const	SYS___mac_get_pid : usize = 390;
			/* 391  **/
			/* 392  **/
			/* 393  **/
pub const	SYS_pselect : usize=   394;
pub const	SYS_pselect_nocancel : usize = 395;
pub const	SYS_read_nocancel : usize = 396;
pub const	SYS_write_nocancel : usize = 397;
pub const	SYS_open_nocancel : usize = 398;
pub const	SYS_close_nocancel : usize = 399;
pub const	SYS_wait4_nocancel : usize = 400;
pub const	SYS_recvmsg_nocancel : usize = 401;
pub const	SYS_sendmsg_nocancel : usize = 402;
pub const	SYS_recvfrom_nocancel : usize = 403;
pub const	SYS_accept_nocancel : usize = 404;
pub const	SYS_msync_nocancel : usize = 405;
pub const	SYS_fcntl_nocancel : usize = 406;
pub const	SYS_select_nocancel : usize = 407;
pub const	SYS_fsync_nocancel : usize = 408;
pub const	SYS_connect_nocancel : usize = 409;
pub const	SYS_sigsuspend_nocancel : usize = 410;
pub const	SYS_readv_nocancel : usize = 411;
pub const	SYS_writev_nocancel : usize = 412;
pub const	SYS_sendto_nocancel : usize = 413;
pub const	SYS_pread_nocancel : usize = 414;
pub const	SYS_pwrite_nocancel : usize = 415;
pub const	SYS_waitid_nocancel : usize = 416;
pub const	SYS_poll_nocancel : usize = 417;
pub const	SYS_msgsnd_nocancel : usize = 418;
pub const	SYS_msgrcv_nocancel : usize = 419;
pub const	SYS_sem_wait_nocancel : usize = 420;
pub const	SYS_aio_suspend_nocancel : usize = 421;
pub const	SYS___sigwait_nocancel : usize = 422;
pub const	SYS___semwait_signal_nocancel : usize = 423;
pub const	SYS___mac_mount : usize = 424;
pub const	SYS___mac_get_mount : usize = 425;
pub const	SYS___mac_getfsstat : usize = 426;
pub const	SYS_fsgetpath : usize= 427;
pub const	SYS_audit_session_self : usize = 428;
pub const	SYS_audit_session_join : usize = 429;
pub const	SYS_fileport_makeport : usize = 430;
pub const	SYS_fileport_makefd : usize = 431;
pub const	SYS_audit_session_port : usize = 432;
pub const	SYS_pid_suspend : usize = 433;
pub const	SYS_pid_resume : usize = 434;
pub const	SYS_pid_hibernate : usize = 435;
pub const	SYS_pid_shutdown_sockets : usize = 436;
			/* 437  old shared_region_slide_np **/
pub const	SYS_shared_region_map_and_slide_np : usize = 438;
pub const	SYS_kas_info : usize=  439;
pub const	SYS_memorystatus_control : usize = 440;
pub const	SYS_guarded_open_np : usize = 441;
pub const	SYS_guarded_close_np : usize = 442;
pub const	SYS_guarded_kqueue_np : usize = 443;
pub const	SYS_change_fdguard_np : usize = 444;
pub const	SYS_usrctl : usize=    445;
pub const	SYS_proc_rlimit_control : usize = 446;
pub const	SYS_connectx : usize=  447;
pub const	SYS_disconnectx : usize = 448;
pub const	SYS_peeloff : usize=   449;
pub const	SYS_socket_delegate : usize = 450;
pub const	SYS_telemetry : usize= 451;
pub const	SYS_proc_uuid_policy : usize = 452;
pub const	SYS_memorystatus_get_level : usize = 453;
pub const	SYS_system_override : usize = 454;
pub const	SYS_vfs_purge : usize= 455;
pub const	SYS_sfi_ctl : usize=   456;
pub const	SYS_sfi_pidctl : usize = 457;
pub const	SYS_coalition : usize= 458;
pub const	SYS_coalition_info : usize = 459;
pub const	SYS_necp_match_policy : usize = 460;
pub const	SYS_getattrlistbulk : usize = 461;
pub const	SYS_clonefileat : usize = 462;
pub const	SYS_openat : usize=    463;
pub const	SYS_openat_nocancel : usize = 464;
pub const	SYS_renameat : usize=  465;
pub const	SYS_faccessat : usize= 466;
pub const	SYS_fchmodat : usize=  467;
pub const	SYS_fchownat : usize=  468;
pub const	SYS_fstatat : usize=   469;
pub const	SYS_fstatat64 : usize= 470;
pub const	SYS_linkat : usize=    471;
pub const	SYS_unlinkat : usize=  472;
pub const	SYS_readlinkat : usize = 473;
pub const	SYS_symlinkat : usize= 474;
pub const	SYS_mkdirat : usize=   475;
pub const	SYS_getattrlistat : usize = 476;
pub const	SYS_proc_trace_log : usize = 477;
pub const	SYS_bsdthread_ctl : usize = 478;
pub const	SYS_openbyid_np : usize = 479;
pub const	SYS_recvmsg_x : usize= 480;
pub const	SYS_sendmsg_x : usize= 481;
pub const	SYS_thread_selfusage : usize = 482;
pub const	SYS_csrctl : usize=    483;
pub const	SYS_guarded_open_dprotected_np : usize = 484;
pub const	SYS_guarded_write_np : usize = 485;
pub const	SYS_guarded_pwrite_np : usize = 486;
pub const	SYS_guarded_writev_np : usize = 487;
pub const	SYS_renameatx_np : usize = 488;
pub const	SYS_mremap_encrypted : usize = 489;
pub const	SYS_netagent_trigger : usize = 490;
pub const	SYS_stack_snapshot_with_config : usize = 491;
pub const	SYS_microstackshot : usize = 492;
pub const	SYS_grab_pgo_data : usize = 493;
pub const	SYS_persona : usize=   494;
			/* 495  **/
pub const	SYS_mach_eventlink_signal : usize = 496;
pub const	SYS_mach_eventlink_wait_until : usize = 497;
pub const	SYS_mach_eventlink_signal_wait_until : usize = 498;
pub const	SYS_work_interval_ctl : usize = 499;
pub const	SYS_getentropy : usize = 500;
pub const	SYS_necp_open : usize= 501;
pub const	SYS_necp_client_action : usize = 502;
pub const	SYS___nexus_open : usize = 503;
pub const	SYS___nexus_register : usize = 504;
pub const	SYS___nexus_deregister : usize = 505;
pub const	SYS___nexus_create : usize = 506;
pub const	SYS___nexus_destroy : usize = 507;
pub const	SYS___nexus_get_opt : usize = 508;
pub const	SYS___nexus_set_opt : usize = 509;
pub const	SYS___channel_open : usize = 510;
pub const	SYS___channel_get_info : usize = 511;
pub const	SYS___channel_sync : usize = 512;
pub const	SYS___channel_get_opt : usize = 513;
pub const	SYS___channel_set_opt : usize = 514;
pub const	SYS_ulock_wait : usize = 515;
pub const	SYS_ulock_wake : usize = 516;
pub const	SYS_fclonefileat : usize = 517;
pub const	SYS_fs_snapshot : usize = 518;
pub const	SYS_register_uexc_handler : usize = 519;
pub const	SYS_terminate_with_payload : usize = 520;
pub const	SYS_abort_with_payload : usize = 521;
pub const	SYS_necp_session_open : usize = 522;
pub const	SYS_necp_session_action : usize = 523;
pub const	SYS_setattrlistat : usize = 524;
pub const	SYS_net_qos_guideline : usize = 525;
pub const	SYS_fmount : usize=    526;
pub const	SYS_ntp_adjtime : usize = 527;
pub const	SYS_ntp_gettime : usize = 528;
pub const	SYS_os_fault_with_payload : usize = 529;
pub const	SYS_kqueue_workloop_ctl : usize = 530;
pub const	SYS___mach_bridge_remote_time : usize = 531;
pub const	SYS_coalition_ledger : usize = 532;
pub const	SYS_log_data : usize=  533;
pub const	SYS_memorystatus_available_memory : usize = 534;
			/* 535  **/
pub const	SYS_shared_region_map_and_slide_2_np : usize = 536;
pub const	SYS_pivot_root : usize = 537;
pub const	SYS_task_inspect_for_pid : usize = 538;
pub const	SYS_task_read_for_pid : usize = 539;
pub const	SYS_preadv : usize=    540;
pub const	SYS_pwritev : usize=   541;
pub const	SYS_preadv_nocancel : usize= 542;
pub const	SYS_pwritev_nocancel : usize = 543;
pub const	SYS_ulock_wait2 : usize = 544;
pub const	SYS_proc_info_extended_id : usize = 545;
pub const	SYS_MAXSYSCALL : usize = 546;
pub const	SYS_invalid : usize = 63;

pub const PROT_NONE	: usize = 0x00;	/* no permissions */
pub const	PROT_READ	: usize = 0x01;	/* pages can be read */
pub const	PROT_WRITE	: usize = 0x02;	/* pages can be written */
pub const	PROT_EXEC	: usize = 0x04;	/* pages can be executed */

/*
 * Flags contain sharing type and options.
 * Sharing types; choose one.
 */
pub const	MAP_SHARED	: usize = 0x0001;	/* share changes */
pub const	MAP_PRIVATE	: usize = 0x0002;	/* changes are private */

/*
 * Other flags
 */
pub const MAP_FIXED	: usize = 0x0010;	/* map addr must be exactly as requested */
pub const MAP_ANON	: usize = 0x1000;	/* allocated from memory, swap space */
