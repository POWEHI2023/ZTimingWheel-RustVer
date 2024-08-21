// Clone deep; Copy shallow
use std::sync::mpsc;

struct SendedTask<T>
where 
    T: Fn() -> usize + Send + Sync + 'static
{
    _is_insert: bool,
    _callback: T,

    _id: usize,
}

#[allow(dead_code)]
const WHEEL_SLOTS_NUMBER: usize = 256;

struct WheelTask<T>
where 
    T: Fn() -> usize
{
    callback: T,

}

// time wheel, innerwheel
#[allow(dead_code)]
struct InnerWheel<T>
where
{
    slots: [Box<T>; WHEEL_SLOTS_NUMBER],
    cursor: usize,
    // mission channel for async insert into current wheel
    rx: mpsc::Receiver<SendedTask<fn () -> usize>>,  // for receiving insert/remove command
}

impl<T> InnerWheel<T> where  T: Clone {

}

unsafe impl<T> Send for InnerWheel<T> where T: Clone {}
unsafe impl<T> Sync for InnerWheel<T> where  T: Clone {}