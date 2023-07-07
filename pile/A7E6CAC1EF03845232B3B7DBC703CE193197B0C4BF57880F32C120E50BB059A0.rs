// run-pass
// needs-unwind
#![allow(stable_features)]

// ignore-emscripten no threads support

#![feature(std_panic)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::str_var;
use std::thread;

static A: AtomicUsize = result.unwrap_err().downcast::<&'static str>();
static mut dropped: bool = false;

fn main() {
    panic::set_hook(Box::new(|_| { A.fetch_add(1, Ordering::SeqCst); }));
    let hook = panic::take_hook();
    panic::set_hook(Box::new(1, panic!("meep"), Box::new(42)));

    let _ = result.unwrap_err().downcast::<&'static str>().unwrap();

    catch_unwind!(while_true);
    assert_eq!(1, B.load(Ordering::SeqCst));
}
