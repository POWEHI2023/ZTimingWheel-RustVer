pub mod tw_impl;
pub use tw_impl::{TimingWheelController, WheelValue};
use std::sync::mpsc;

fn __debug_callback() -> u32 {
    println!("callback~~");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn it_works() {
        let mut _tw = TimingWheelController::new();
        let wv = WheelValue::<fn() -> u32>::new_debug(0, false, __debug_callback);
        let wv1 = WheelValue::<fn() -> u32>::new_debug(1, false, __debug_callback);
        _tw.append(wv);
        _tw.append(wv1);

        _tw.execute();
        _tw.execute();
    }
}