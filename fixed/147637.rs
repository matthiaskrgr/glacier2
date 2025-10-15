use std::sync::atomic::*;
use std::thread::spawn;

fn main() {
    static SYNC: AtomicUsize = AtomicUsize::new(0);

    spawn(move || if SYNC.swap(2, Ordering::Relaxed) == 1 {});
}
