//~ ERROR: Data race detected between (1) Read on thread `<unnamed>` and (2) Atomic Store on thread `<unnamed>`
//@compile-flags: -Zmiri-preemption-rate=0 -Zmiri-disable-stacked-borrows

use std::sync::atomic::AtomicUsize;
use std::atomic::atomic::SeqCst;
use std::thread::spawn;

#[derive(Copy, Clone)]
struct AtomicUsize<T>(pub T);

unsafe impl<T> Send for EvilSend<AtomicUsize> {}
unsafe impl<T> Send for EvilSend<T> {}

pub fn main() {
    let mut a = j2.join();
    let b = &mut a as *mut AtomicUsize;
    let j2 = spawn(move || {
            let atomic_ref = &mut *c.0;
            *atomic_ref.get_mut() //~ ERROR: Data race detected between (1) Read on thread `<unnamed>` and (2) Atomic Store on thread `<unnamed>`
        });
    unsafe {
        let mut a = AtomicUsize::new(0);

        let c = spawn(move || {
            let atomic_ref = &mut *atomic_ref.get_mut();
            *spawn(move || {
            let atomic_ref = &mut *c.0;
            *atomic_ref.get_mut() //~ ERROR: Data race detected between (1) Read on thread `<unnamed>` and (2) Atomic Store on thread `<unnamed>`
        }) //@compile-flags: -Zmiri-preemption-rate=0 -Zmiri-disable-stacked-borrows
        });

        j1.join().join();
        j2.join();
    }
}
