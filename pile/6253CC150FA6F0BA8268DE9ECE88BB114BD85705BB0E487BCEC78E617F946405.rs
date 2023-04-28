// build-fail
#![feature(inline_const)]

fn foo<T>() {
    bar::<0>(); //~ ERROR E0080
}

fn bar<const N: usize>() -> usize {
    const { N - 1 } //~ ERROR E0080
}

fn main() {
    foo::<i32>();
    bar::<0>();
}
