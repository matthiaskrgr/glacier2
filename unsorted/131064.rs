use std::{mem, ptr};

fn main() {
    basic();
}

fn basic() {
    let x = &42;
    let ptr = x as *const i32;

    let addr_mu: mem::MaybeUninit<usize> = unsafe { mem::transmute(ptr) };
}
