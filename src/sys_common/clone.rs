/*
static void dummy(int x) { }
weak_alias(dummy, __aio_atfork);

pid_t _Fork(void)
{
	pid_t ret;
	sigset_t set;
	__block_all_sigs(&set);
	__aio_atfork(-1);
	LOCK(__abort_lock);
#ifdef SYS_fork
	ret = __syscall(SYS_fork);
#else
	ret = __syscall(SYS_clone, SIGCHLD, 0);
#endif
	if (!ret) {
		pthread_t self = __pthread_self();
		self->tid = __syscall(SYS_gettid);
		self->robust_list.off = 0;
		self->robust_list.pending = 0;
		self->next = self->prev = self;
		__thread_list_lock = 0;
		libc.threads_minus_1 = 0;
		if (libc.need_locks) libc.need_locks = -1;
	}
	UNLOCK(__abort_lock);
	__aio_atfork(!ret);
	__restore_sigs(&set);
	return __syscall_ret(ret);
}
 */



/*
// runPerThreadSyscall runs perThreadSyscall for this M if required.
//
// This function throws if the system call returns with anything other than the
// expected values.
//go:nosplit
func runPerThreadSyscall() {
	gp := getg()
	if gp.m.needPerThreadSyscall.Load() == 0 {
		return
	}

	args := perThreadSyscall
	r1, r2, errno := syscall.Syscall6(args.trap, args.a1, args.a2, args.a3, args.a4, args.a5, args.a6)
	if GOARCH == "ppc64" || GOARCH == "ppc64le" {
		// TODO(https://go.dev/issue/51192 ): ppc64 doesn't use r2.
		r2 = 0
	}
	if errno != 0 || r1 != args.r1 || r2 != args.r2 {
		print("trap:", args.trap, ", a123456=[", args.a1, ",", args.a2, ",", args.a3, ",", args.a4, ",", args.a5, ",", args.a6, "]\n")
		print("results: got {r1=", r1, ",r2=", r2, ",errno=", errno, "}, want {r1=", args.r1, ",r2=", args.r2, ",errno=0\n")
		fatal("AllThreadsSyscall6 results differ between threads; runtime corrupted")
	}

	gp.m.needPerThreadSyscall.Store(0)
}
*/

