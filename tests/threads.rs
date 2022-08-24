use librs::sys_common::pthread::Pthread;

#[test]
fn clone_thread() {
    let mut t = Pthread::default();
    let new_thread = t.create();
    assert_ne!(new_thread.thread, u64::MAX);
    assert_ne!(new_thread.thread, u64::MIN);
    println!("THREAD ID {:#?}", new_thread.thread);

}

#[test]
fn destroy_thread() {
    let mut t = Pthread::default();
    let new_thread = t.create();
    let destroy_ret = new_thread.destroy();

    assert_eq!(destroy_ret, 0);
}

