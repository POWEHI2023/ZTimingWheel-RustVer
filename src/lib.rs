pub mod time_wheel;
pub mod atomic_queue;

#[cfg(test)]
mod tests {
    use std::{borrow::BorrowMut, cell::{RefCell, UnsafeCell}, sync::Mutex, thread};

    use atomic_queue::{Queue, QueueNode};
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_queue() {
        println!("Hello RustLocklessQueue.");

        let mut que = Arc::new(Mutex::new(Queue::new(0)));

        let mut ts = vec![];
        for i in 0..1000 {
            let mut q = Arc::clone(&que);
            ts.push(thread::spawn(move || {
                let mut guard = q.lock().unwrap();
                guard.emplace(i);
            }));
        }

        for i in ts {
            i.join();
        }

        let mut guard = que.lock().unwrap();
        guard.consume_all(|val: i32| {
            println!("comsume value {val}");
        });
    }
}