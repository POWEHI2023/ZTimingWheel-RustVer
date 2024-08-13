// crate::tw_impl::tw module
use std::{thread, time};
use dyn_clone::{clone_trait_object, DynClone};
pub use downcast::{Any, downcast};

pub const TASK_SUCCESSED: u32 = 0;
pub const TASK_FAILED: u32 = 1;

/**
 * @param status input from tasks output
 * execute callback for each task return value
 */
fn callback(_status: u32) {

}

fn __debug_callback() -> u32 {
     println!("callback~~");
     return 0;
}

mod  inner_wheel {
     use downcast::{Any, downcast};
     // trait items always share the visibility of their trait
     pub trait WheelValueTrait : Any {
          fn execute(&self) -> u32;
          fn gap_time(&self) -> usize;
          fn loop_on(&self) -> bool;
     }
     downcast!(dyn WheelValueTrait);

     #[derive(Clone)]
     pub struct WheelValue<T>
     where
          T: Fn() -> u32 
     {
          pub __callback: T,
          pub __start_time: usize,     // seconds
          pub __gap_time: usize,       // seconds
          pub __loop_on: bool,
     }

     impl<T> WheelValue<T>
     where
          T: Fn() -> u32 + 'static
     {
          pub fn new(__gap_time: usize, __loop_on: bool, __callback: T) -> Box<dyn WheelValueTrait> {
               Box::new(WheelValue {
                    __callback,
                    __start_time: 0,    // debug
                    __gap_time,
                    __loop_on,
               })
          }

          pub fn new_debug(__gap_time: usize, __loop_on: bool, __callback: T) -> WheelValue<T> {
               WheelValue {
                    __callback,
                    __start_time: 0,    // debug
                    __gap_time,
                    __loop_on,
               }
          }
     }

     impl<T> WheelValueTrait for WheelValue<T>
     where
          T: Fn() -> u32 + 'static
     {
          fn execute(&self) -> u32 { (self.__callback)() }
          fn gap_time(&self) -> usize { self.__gap_time }
          fn loop_on(&self) -> bool { self.__loop_on }
     }
}





pub use inner_wheel::{WheelValueTrait, WheelValue};
pub const ELEMENT_COUNT_FOR_EACH_WHEEL: u64 = 256;

pub trait ExecutableWheel: DynClone + Any {
     fn execute(&mut self) -> u32;
     fn append_task(&mut self, _pos: usize, _task: WheelValue<fn() -> u32>) -> u64;
     fn remove_task(&mut self, _id: u64);
}
clone_trait_object!(ExecutableWheel);
downcast!(dyn ExecutableWheel);

// three level SingleWheel duration (1, 256, 256 * 256)
#[derive(Clone)]
struct SingleWheel {
     // TODO: WheelValueTrait => Link list of WheelValueTrait
     __elems: Vec<Option<WheelValue<fn() -> u32>>>,  // Box<dyn WheelValueTrait>
     __pos: usize,
}

impl SingleWheel {
     fn new() -> SingleWheel {
          let mut ret = SingleWheel {
               __elems: Vec::with_capacity(ELEMENT_COUNT_FOR_EACH_WHEEL as usize),
               __pos: 0,
          };
          for _i in 0..ELEMENT_COUNT_FOR_EACH_WHEEL as usize {
               ret.__elems.push(None);
          }
          ret
     }
}

impl ExecutableWheel for SingleWheel {
     fn execute(&mut self) -> u32 {
          if let Some(tasks) = &self.__elems[self.__pos] {
               tasks.execute();
          }
          self.__elems[self.__pos] = None;
          thread::sleep(time::Duration::from_secs(1));

          self.__pos = (self.__pos + 1) % ELEMENT_COUNT_FOR_EACH_WHEEL as usize;
          self.__pos as u32
     }

     fn append_task(&mut self, _pos: usize, _task: WheelValue<fn() -> u32>) -> u64 {
          if _pos >= ELEMENT_COUNT_FOR_EACH_WHEEL as usize {
               return 0;
          }
          self.__elems[_pos] = Some(_task);
          (_pos + 1).try_into().unwrap()  // debug not 0
     }

     fn remove_task(&mut self, _id: u64) {
          if _id >= ELEMENT_COUNT_FOR_EACH_WHEEL as u64 {
               return;
          }
          self.__elems[_id as usize] = None;
     }
}

// Timing wheel
#[derive(Clone)]
pub struct TimingWheel {
     subwheel: Vec<Box<dyn ExecutableWheel>>,
     pos: usize,
     duration: u64, // SingleWheel's duration is 1
}

impl ExecutableWheel for TimingWheel {
     fn execute(&mut self) -> u32 {
          let _ipos = self.subwheel[self.pos].execute();
          if _ipos == 0 {
               self.pos = (self.pos + 1) % ELEMENT_COUNT_FOR_EACH_WHEEL as usize;
               if self.pos == 0 {
                    // next wheel
                    return 0;
               }
          }
          1
     }

     /**
      * @param _pos time gap for execution of this task
      * @param _task task
      */
     fn append_task(&mut self, _pos: usize, _task: WheelValue<fn() -> u32>) -> u64 {
          let _idx = _pos / self.duration as usize;
          if _idx >= ELEMENT_COUNT_FOR_EACH_WHEEL as usize {
               return 0;
          }

          let _ipos = _pos % self.duration as usize;
          let _id = self.subwheel[_idx].append_task(_ipos, _task);
          if _id == 0 {
               return 0;
          }

          (_id << 8) | _idx as u64
     }

     fn remove_task(&mut self, _id: u64) {
          let _idx = _id & 0x00FF;
          if _idx >= ELEMENT_COUNT_FOR_EACH_WHEEL {
               return;
          }
          self.subwheel[_idx as usize].remove_task(_id >> 8);
     }
}

pub const DEFAULT_TW_LAYER: u32 = 3;

impl TimingWheel {
     // default layer is 3 => max 256 * 256 * 256 seconds
     pub fn new() -> Box<dyn ExecutableWheel> {
          TimingWheel::new_with_layer(DEFAULT_TW_LAYER)
     }

     // create a timing wheel with layer of `layer`
     pub fn new_with_layer(layer: u32) -> Box<dyn ExecutableWheel> {
          if layer == 1 {
               return Box::new(SingleWheel::new());
          }
          let mut ret = Box::new(TimingWheel {
               subwheel: Vec::with_capacity(ELEMENT_COUNT_FOR_EACH_WHEEL as usize),
               pos: 0,
               duration: (layer as u64 - 1) * ELEMENT_COUNT_FOR_EACH_WHEEL,
          });
          for _i in 0..ELEMENT_COUNT_FOR_EACH_WHEEL {
               ret.subwheel.push(TimingWheel::new_with_layer(layer - 1));
          }
          ret
     }
}