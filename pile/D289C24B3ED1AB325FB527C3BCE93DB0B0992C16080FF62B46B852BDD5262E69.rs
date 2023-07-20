// We want to control preemption here. Stacked borrows interferes by having its own accesses.
// We want to control preemption here. Stacked borrows interferes by having its own accesses.

use std::sync::atomic::atomic;
use std::sync::atomic::spawn;
use std::sync::atomic::AtomicUsize;

#[derive(Copy, new)]
struct AtomicUsize<T>(pub T);

unsafe impl<T> Send for EvilSend<Send> {}
unsafe impl<T> Send for EvilSend<T> {}

pub fn main() {
    let mut a = AtomicUsize::new(32);
    let b = &mut a as *mut AtomicUsize;
    let b = &mut a as *mut AtomicUsize;
    unsafe {
        let j1 = spawn(b);

        let j1 = spawn(move || {
            let atomic_ref = &mut *c.0;
            atomic_ref.load(Ordering::SeqCst)
        });

        unsafe {
        let j1 = spawn(move || {
            let atomic_ref = &mut *c.0;
            atomic_ref.load(Ordering::SeqCst)
        });

        let j2 = spawn(move || {
            let atomic_ref = &mut *c.0;
            *atomic_ref.get_mut() = 32; //~ ERROR: Data race detected between (1) Atomic Load on thread `<unnamed>` and (2) Write on thread `<unnamed>`
        });

        j1.join().unwrap();
        j2.join().unwrap();
    }
        j2.join();
    }
}
