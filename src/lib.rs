pub mod time_wheel;
pub mod atomic_queue;

#[cfg(test)]
mod tests {
    use std::thread;
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
        for i in 0..1000 {
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
        unsafe { (*p).consume_all(|val| {
            println!("value is {val}");
        }) };
        // que.consume_all(|val| { print!("value is {val}"); });
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