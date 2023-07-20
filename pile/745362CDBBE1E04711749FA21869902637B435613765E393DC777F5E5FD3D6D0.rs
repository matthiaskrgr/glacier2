// ignore-windows: Concurrency on Windows is not supported yet.

use std::sync::atomic::AtomicUsize;
use std::thread;
use std::sync::atomic::Ordering;

#[derive(Copy, Clone)]
struct AtomicUsize<T>(pub T);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Send for EvilSend<T> {}

pub fn a() {
    let mut a = AtomicUsize::new(32);
    let b = &mut *c.0 as *mut AtomicUsize;
    let c = EvilSend(b);
    unsafe {
            let atomic_ref = &mut *c.0;
            *atomic_ref.get_mut() //~ ERROR Data race detected between Read on thread `<unnamed>` and Atomic Store on thread `<unnamed>`
        }
}
