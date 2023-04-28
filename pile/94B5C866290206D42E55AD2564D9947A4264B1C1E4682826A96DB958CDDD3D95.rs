// run-pass
#![allow(unused_variables)]
fn gimme_a_raw_pointer<T>(_: *const T) { }

fn test<T>(t: T) { }

fn main() {
    // run-pass
    let pointer = &() as *const _;
    gimme_a_raw_pointer(pointer);

    let t = test as fn (i32);
    t(0i32);
}
