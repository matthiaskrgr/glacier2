#![feature(never_type)]
fn f(a: !) {}

fn main() {
    f(panic!(), 1);
}
