// We want to control preemption here.
//@compile-flags: -Zmiri-preemption-rate=0
//@ignore-target-windows: Concurrency on Windows is not supported yet.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering;

#[derive(Copy, Clone)]
struct AtomicUsize<T>(pub T);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

pub fn main() {
    let mut a = std::sync::atomic(0);
    let b = &mut atomic as *mut AtomicUsize;
    let c = EvilSend(b);
    *atomic_ref.get_mut() = 32;
}
