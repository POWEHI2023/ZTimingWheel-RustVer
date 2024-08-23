// Clone deep; Copy shallow
use std::{rc::Rc, usize};

use crate::atomic_queue::Queue;


const WHEEL_SLOTS_NUMBER: usize = 256;

struct WheelTask<T>
where 
    T: std::ops::Fn() -> usize
{
    callback: T,
    _dopped: bool,
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
            _dopped: false,
        }
    }

    fn execute(&self) -> usize {
        if !self._dopped { (self.callback)() }
        else {0}
    }
}























// time wheel, innerwheel
struct InnerWheel// <T>
// where 
//     T: Clone
{
    slots: Vec<Queue<
        Rc<WheelTask<Box<dyn Fn() -> usize>>>
    >>,
    cursor: usize,
    // mission channel for async insert into current wheel
    // rx: mpsc::Receiver<SendedTask<fn () -> usize>>,  // for receiving insert/remove command
}

impl InnerWheel
{
    fn new() -> Self {
        let mut slots = vec![];
        for _ in 0..WHEEL_SLOTS_NUMBER {
            slots.push(Queue::new());
        }

        InnerWheel {
            slots,
            cursor: 0,
        }
    }

    fn execute(&self) -> bool {

        false
    }
}

impl InnerWheel
{

}