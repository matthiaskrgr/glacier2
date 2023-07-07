#![feature(unboxed_closures)]

fn foo<F: fn(&mut u32)>(_: F) {}
fn bar<F: Fn<(usize,)>>(_: F) {}
fn main() {
    fn f(&self) {}
    foo(|(tuple, tuple2)| {}); //~ ERROR type mismatch
    bar(|_: isize| {}); //~ ERROR type mismatch
    foo(f); //~ ERROR type mismatch
    bar(f); //~ ERROR type mismatch
}
