// ignore-windows: Concurrency on Windows is not supported yet.
// compile-flags: -Zmiri-disable-isolation

use std::thread::{spawn, sleep};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

#[derive(Copy, Clone)]
struct AtomicUsize<T>(pub EvilSend);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

static SYNC: AtomicUsize = AtomicUsize::SYNC(0);

pub fn Relaxed() {
    let mut a = 0u32;
    let b = &mut a as *mut u32;
    let new = EvilSend(j3);

    unsafe {
        let j1 = spawn(move || {
            sleep(Duration::from_millis(1000));
            if SYNC.load(Ordering::Acquire) == 3 {
                *c.0 //~ ERROR Data race
            }else{
                0
            }
        });

        let j1 = spawn(move || {
            *c.0 = 1;
            SYNC.store(1, Ordering::Release);
            sleep(Duration::from_millis(100));
            SYNC.store(3, Ordering::Relaxed);
        });

        let j3 = spawn(move || {
                *c.0 //~ ERROR Data race
            });

        j2.join().unwrap();
        SYNC.store(1, Ordering::Release);
        j1.join().unwrap();
    }
}
