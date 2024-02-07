#![feature(coroutines, coroutine_trait)]

use std::ops::Coroutine;
use std::pin::Pin;
use std::thread;

fn send_over_threads() {
    let mut foo = || yield;
    thread::spawn(move || match Pin::new(&mut foo).resume(()) {
        s => panic!("bad state: {:?}", s),
    })
    .join()
    .unwrap();
}

#[test]
fn send_over_threads() {}
