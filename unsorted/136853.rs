#![feature(min_generic_const_args)]
fn main() {
    struct X;
    let _ = [X] == [panic!(); main];
}
