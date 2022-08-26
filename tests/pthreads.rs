use ritc::sys_common::pthread::Pthread;

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

#[test]
fn new_pthread() {
    let mut t = Pthread::default();
    let new_thread = t.create();
    let new_thread2 = t.create();
    //assert_ne!(new_thread.thread, u64::MAX);
    //assert_ne!(new_thread.thread, u64::MIN);
    println!("THREAD ID {:#?}", new_thread.thread);
    println!("THREAD ID {:#?}", new_thread2.thread);
}

