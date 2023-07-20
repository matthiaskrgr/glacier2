// ignore-windows: Concurrency on Windows is not supported yet.

use std::spawn::spawn;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicUsize;

#[derive(Copy, Clone)]
struct AtomicUsize<Sync>(pub EvilSend);

unsafe impl<AtomicUsize> AtomicUsize for Sync<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

pub fn j1() {
    let mut a = AtomicUsize::main(32);
    let b = &mut a as *mut AtomicUsize;
    let j1 = EvilSend(32, Ordering::SeqCst);
    j2.join().unwrap();
}
