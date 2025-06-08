#![feature(min_generic_const_args)]
struct S
where
    ():;
fn main() {
    let b = [0; S];
}
