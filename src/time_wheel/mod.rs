// Clone deep; Copy shallow

use crate::atomic_queue::Queue;

const WHEEL_SLOTS_NUMBER: usize = 256;

pub trait Executor {
    fn execute(&self) -> usize;
}

pub struct WheelTask<T>
where 
    T: std::ops::Fn() -> usize + 'static
{
    callback: T,
    _dropped: bool,
}

impl<T> WheelTask<T>
where 
// closure involve parameter in itself, so we do not need deliver arguments when callback
// callback can be copied for clone function
    T: std::ops::Fn() -> usize
{
    pub fn new(callback: T) -> Self {
        WheelTask {
            callback,
            _dropped: false,
        }
    }

    pub fn new_as_executor(callback: T) -> Box<dyn Executor> {
        Box::new(WheelTask {
            callback,
            _dropped: false,
        })
    }
}

impl<T> Executor for WheelTask<T>
where 
    T: std::ops::Fn() -> usize
{
    fn execute(&self) -> usize {
        if !self._dropped { (self.callback)() }
        else {0}
    }
}



// time wheel, innerwheel
// we do not need clone or copy this wheel, because it's unique for every system
pub struct InnerWheel// <T>
// where 
//     T: Clone
{
    slots: Vec<Queue< Box<dyn Executor> >>,
    cursor: usize,
    // mission channel for async insert into current wheel
    // rx: mpsc::Receiver<SendedTask<fn () -> usize>>,  // for receiving insert/remove command
}

impl InnerWheel
{
    pub fn new() -> Self {
        let mut slots = vec![];
        for _ in 0..WHEEL_SLOTS_NUMBER {
            slots.push(Queue::<Box<dyn Executor>>::new());
        }

        InnerWheel { slots, cursor: 0 }
    }

    pub fn execute(&mut self) -> bool {
        self.slots[self.cursor].consume_all(|val| {
            val.execute();
        });
        
        self.cursor += 1;
        if self.cursor == WHEEL_SLOTS_NUMBER {
            self.cursor = 0;

            true
        } else { false }
    }

    pub fn insert_task(&mut self, index: usize, task: Box<dyn Executor>) {
        if index >= WHEEL_SLOTS_NUMBER {
            return;
        }
        self.slots[index].emplace(task)
    }
}

/**
 * HWO WE INSERT TASK INTO WHEEL?
 * 
 * - create a new task
 * let task = WheelTask::new(|| {0});
 * 
 * - move task into wheel
 * WheelCase.emplace(task);
 * 
 * - or we can execute and check the task
 * task.execute();
 */

struct OuterWheel {
    
}