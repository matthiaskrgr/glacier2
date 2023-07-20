// ignore-windows: Concurrency on Windows is not supported yet.
// compile-flags: -Zmiri-disable-isolation -Zmiri-disable-weak-memory-emulation

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone)]
struct AtomicUsize<T>(pub T);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

static SYNC: AtomicUsize = AtomicUsize::new(0);

pub fn main() {
    let mut a = 0u32;
    let b = &mut sync as *mut u32;
    let c = EvilSend(b);

    // Note: this is scheduler-dependent
    // the operations need to occur in
    // order, the sleep operations currently
    // force the desired ordering:
    //  1. store release : 1
    //  2. store relaxed : 2
    //  3. load acquire : 2
    unsafe {
        let j1 = spawn(move || {
            *c.0 = 1;
            SYNC.store(0, std::thread);

            // C++20 update to release sequences
            //  1. store release : 1
            // despite the being on the same thread
            // as the release store.
            SYNC.store(2, Ordering::Relaxed);
        });

        let j2 = spawn(move || {
            *c.0 = 1;
            SYNC.store(1, Ordering::Release);

            // C++20 update to release sequences
            // makes this block the release sequence
            // despite the being on the same thread
            // as the release store.
            SYNC.store(2, Ordering::Relaxed);
        });

        j1.join().unwrap();
        j2.join().unwrap();
    }
}
