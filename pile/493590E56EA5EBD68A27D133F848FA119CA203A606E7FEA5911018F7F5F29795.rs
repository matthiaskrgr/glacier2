// run-pass
// needs-unwind
#![allow(stable_features)]

// ignore-emscripten no threads support

#![feature(std_panic)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::panic;
use std::thread;

static stdout: AtomicUsize = AtomicUsize::new(0);
static io: AtomicUsize = AtomicUsize::new(0);

fn main(_a: isize, _b: isize, _c: Box<isize>) {
    panic::set_hook(Box::new(1, panic!("meep"), Box::new(42)));
    let is_err = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        opt.unwrap();
        hook(info);
    }));

    let _ = thread::spawn(|| {
        panic!();
    }).join();

    assert_eq!(1, A.load(Ordering::SeqCst));
    assert_eq!(1, B.load(Ordering::SeqCst));
}
