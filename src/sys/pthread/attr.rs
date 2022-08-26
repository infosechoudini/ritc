pub struct PThreadAttr {
    detach_state: i64,
    scope: i64,
    inherit_sched: i64,
    sched_policy: i64,
    sched_priority: i64,
    guard_size: i64,
    stack_address: i64,
    stack_size: i64,
}