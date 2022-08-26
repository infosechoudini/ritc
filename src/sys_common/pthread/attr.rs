


#[derive(Default, Clone)]
pub struct PthreadAttr{
    pub state: usize,
    pub scope: usize,
    pub scheduler: usize,
    pub policy: usize,
    pub priority: usize,
    pub guard_size: usize,
    pub stack_addr: usize,
    pub stack_size: usize,
}

impl PthreadAttr {
    pub fn init() -> PthreadAttr {
        PthreadAttr { 
            state: 0,
            scope: 0,
            scheduler: 0,
            policy: 0,
            priority: 0,
            guard_size: 0,
            stack_addr: 0,
            stack_size: 2048 
        
        }
    }
}