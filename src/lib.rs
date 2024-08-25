pub mod time_wheel;
pub mod atomic_queue;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashMap, sync::Mutex, thread};
    use atomic_queue::Queue;
    use time_wheel::{Executor, InnerWheel, WheelTask};
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_queue() {
        println!("Hello RustLocklessQueue.");

        // todo
        // 1. can not multi-mutable borrow
        // 2. can borrow from arc as mutable

        let que = Arc::new(Queue::new());

        let mut ts = vec![];
        for i in 0..5000 {
            let q = Arc::clone(&que);
            ts.push(thread::spawn(move || {
                let ptr = Arc::into_raw(q).cast_mut();
                unsafe { (*ptr).emplace(i) };
            }));
        }

        for i in ts {
            let _ = i.join();
        }

        // let mut guard = que.lock().unwrap();
        // guard.consume_all(|val: i32| {
        //    println!("comsume value {val}");
        // });
        let p = Arc::into_raw(que).cast_mut();

        // let map: Arc<Mutex<HashMap<i32, usize>>> = Arc::new(Mutex::new(HashMap::new()));
        let map: RefCell<HashMap<i32, i32>> = RefCell::new(HashMap::new());
        unsafe { (*p).consume_all(|val| {
            // let mut map = map.lock().unwrap();
            let mut map = map.borrow_mut();
            let count = map.entry(val).or_insert(0);
            *count += 1;
        }) };

        for i in 0..5000 {
            let mut map = map.borrow_mut();
            let x = map.entry(i).or_insert(0);
            assert_eq!(*x, 1, "ERROR: {x} is not 1, but it should be");
        }
        // que.consume_all(|val| { print!("value is {val}"); });
    }

    trait __TestBaseTrait {
        fn test(&self);
    }
    struct __TestStruct(i32);
    impl __TestStruct {
        fn new(val: i32) -> Self {
            __TestStruct(val)
        }
    }
    impl __TestBaseTrait for __TestStruct {
        fn test(&self) {
            println!("{}", self.0);
        }
    }

    #[test]
    fn test_queue_pop() {
        let mut que = Queue::<Box<dyn __TestBaseTrait>>::new();
        que.emplace(Box::new(__TestStruct::new(99)));

        if let Some(ret) = que.pop() {
            ret.test();
        }

        // todo!("Finish parallel test cases for pop function first!");
    }

    #[test]
    fn test_inner_wheel() {
        let num = 0;
        let task = WheelTask::new(move || {num});
        let out = task.execute();
        println!("{out}");

        let mut wheel = InnerWheel::new();

        wheel.insert_task(0, Box::new(WheelTask::new(|| {
            println!("Hello Timing Wheel!");
            0
        })));

        wheel.insert_task(0, Box::new(WheelTask::new(|| {
            println!("Hello Timing Wheel Again!");
            0
        })));

        wheel.insert_task(1, Box::new(WheelTask::new(|| {
            println!("Hello Timing Wheel In Position 1!");
            0
        })));

        wheel.insert_task(2, Box::new(WheelTask::new(|| {
            println!("Hello Timing Wheel In Position 2!");
            0
        })));

        wheel.execute();
        
        println!("# Execute next slot.");
        wheel.execute();
    }
}