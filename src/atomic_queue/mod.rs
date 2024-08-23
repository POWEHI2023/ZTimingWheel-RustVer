use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr;
use std::marker::PhantomPinned;

// T: element in Node, has clone trait
struct QueueNode<T>
{
     val: T,
     next: AtomicUsize, // *mut QueueNode<T>,
     _marker: PhantomPinned,
}

impl<T> QueueNode<T> {
     fn new(val: T) -> QueueNode<T> {
          QueueNode {
               val, 
               next: AtomicUsize::new(0),
               _marker: PhantomPinned,
          }
     }

     fn next(&self) -> *mut QueueNode<T> {  
          let addr = self.next.load(Ordering::Relaxed);
          if addr == 0 { ptr::null_mut() }
          else { addr as *mut QueueNode<T> }
     }
}

struct QueueHead
{
     next: AtomicUsize, // *mut QueueNode<T>,
     _marker: PhantomPinned,
}

impl QueueHead {
     fn new() -> Self {
          QueueHead {
               next: AtomicUsize::new(0),
               _marker: PhantomPinned,
          }
     }

     fn next<T>(&self) -> *mut QueueNode<T> {
          let addr = self.next.load(Ordering::Relaxed);
          if addr == 0 { ptr::null_mut() }
          else { addr as *mut QueueNode<T> }
     }
}

// T: element type in Queue
pub struct Queue<T> 
{
     head: Box::<QueueHead>,
     tail: AtomicUsize,  // *mut QueueNode<T>,
     size: AtomicUsize,
     _marker: PhantomPinned,

     _anotation: Option<T>,
}

impl<T> Queue<T> {
     pub fn new() -> Queue<T> {
          let node = Box::into_raw(Box::new(QueueHead::new()));
          Queue {
               head: unsafe { Box::from_raw(node) },
               tail: (node as usize).into(),
               size: 0.into(),
               _marker: PhantomPinned,

               _anotation: None,
          }
     }

     // the type is clear after emplace function call
     pub fn emplace(&mut self, val: T) {
          let node = Box::into_raw(Box::new(
               QueueNode::new(val)
          ));

          let mut crt_tail = self.tail.load(Ordering::Relaxed);
          loop {
               match self.tail.compare_exchange_weak(crt_tail, node as usize, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(_) => {
                         crt_tail = self.tail.load(Ordering::Relaxed);
                    },
               }
          }

          unsafe {
               let front = crt_tail as *mut QueueNode<T>;
               let old = (*front).next.load(Ordering::Relaxed);
               (*front).next.compare_exchange(old, node as usize, Ordering::Relaxed, Ordering::Relaxed).unwrap();
          }

          self.size.fetch_add(1, Ordering::Relaxed);
     }

     pub fn consume_all(&mut self, callback: impl Fn(T)) -> usize {
          let mut crt = self.head.next();
          let mut ret = 0;
          loop {
               if crt as usize == 0 {
                    break;
               }

               // for each task in current queue, callback them
               // auto-dropped
               let ecrt = unsafe { Box::from_raw(crt) };
               callback(ecrt.val);
               ret += 1;

               if crt as usize == self.tail.load(Ordering::Relaxed) {
                    unsafe { self.head.next.store((*crt).next.load(Ordering::Relaxed), Ordering::Relaxed); }
                    break;
               } else {
                    unsafe {
                         while (*crt).next().is_null() { /* wait emplace task set new tail*/ }
                         crt = (*crt).next();
                    }
               }
          }
          ret
     }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        todo!()
    }
}

unsafe impl<T> Send for Queue<T> where T: Clone {}
unsafe impl<T> Sync for Queue<T> where T: Clone {}