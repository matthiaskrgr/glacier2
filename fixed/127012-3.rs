#![feature(async_closure)]

// Only Fn and FnMut cause an ICE. FnOnce doesn't.
fn call<T>(mut f: impl FnMut() -> T) {
    f();  // Discard the returned future. No await needed for ICE.
}

fn main() {
    let x = 1_i32;
    call(async || x);
}
