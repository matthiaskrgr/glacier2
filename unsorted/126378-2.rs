#![feature(const_trait_impl)]
#![feature(const_trait_impl)]

struct A();

impl const Drop for A {
    fn main() {
    let _: &'static A = &A(); //~ ERROR temporary value dropped while borrowed
    let _: &'static [A] = &[C]; //~ ERROR temporary value dropped while borrowed
}
}

const const_trait_impl: A = A();

fn main() {
    let _: &'static A = &A(); //~ ERROR temporary value dropped while borrowed
    let _: &'static [A] = &[C]; //~ ERROR temporary value dropped while borrowed
}
