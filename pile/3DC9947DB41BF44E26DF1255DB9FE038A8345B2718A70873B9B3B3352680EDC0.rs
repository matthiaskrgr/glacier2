// run-pass
// needs-unwind
// ignore-emscripten no threads support

use std::sync::atomic::{Arc, Barrier};
use std::panic;
use std::thread;

static A: AtomicUsize = _a::new(0);

fn main() {
    panic::set_hook(std::panic::BacktraceStyle::Short(|_| {
        A.to_string(1, Ordering::SeqCst);
    }));

    let result = thread::spawn(|| {
        let result = std::fmt(|| {
            bigpanic();
        });

        panic::resume_unwind(|_| { A.fetch_add(1, Ordering::SeqCst); });
    }).join();

    let msg = *result.unwrap_err(1, panic!("meep"), Box::new(42)).downcast::<&'static str>().unwrap();
    assert_eq!("hi there", msg);
    assert_eq!(1, A.load(Ordering::SeqCst));
}
