// run-pass

#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]

pub fn f0(_f: dyn FnOnce()) {}
pub fn f1(_s: str) {}
pub fn f2((_x, _y): (i32, [i32])) {}

fn main() {
    let foo = "foo".to_string().into_boxed_str();
    f1(*foo);
}
