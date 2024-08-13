// crate::tw_impl module

mod tw;
use tw::{TimingWheel, ExecutableWheel, DEFAULT_TW_LAYER, ELEMENT_COUNT_FOR_EACH_WHEEL};
use std::thread;
// use std::sync::Arc;
// use std::cell::RefCell;

pub use tw::WheelValue;

pub struct TimingWheelController {
     _tw: Box<dyn ExecutableWheel>,
     _stop: bool,
     _start_time: u64,   // 用来矫正时间，目前不会用
     _crt_time: u64,
     _limit_gap: u64,
}

impl TimingWheelController {
     pub fn new() -> Box<TimingWheelController> {
          TimingWheelController::new_with_layer(DEFAULT_TW_LAYER)
     }

     pub fn new_with_layer(layer: u32) -> Box<TimingWheelController> {
          let tw = TimingWheel::new_with_layer(layer);
          let ret = Box::new(TimingWheelController {
               _tw: tw,
               _stop: true,
               _start_time: 0,
               _crt_time: 0,
               _limit_gap: layer as u64 * ELEMENT_COUNT_FOR_EACH_WHEEL,
          });
          ret
     }

     pub fn append(&mut self, _task: WheelValue<fn() -> u32>) -> u64 {
          let pos = (self._crt_time + _task.__gap_time as u64) % self._limit_gap;
          self._tw.append_task(pos as usize, _task)
     }

     pub fn execute(&mut self) {
          // 每次调用执行一步
          self._tw.execute();
          self._crt_time += 1;
     }
}