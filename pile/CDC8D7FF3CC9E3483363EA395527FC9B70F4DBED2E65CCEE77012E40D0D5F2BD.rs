// check-pass
#![feature(inline_const)]

fn foo<T>() -> usize {
    const { std::mem::size_of::<i32>() }
}

fn bar<const N: usize>() -> usize {
    const { N + 1 }
}

fn main() {
    foo::<i32>();
    bar::<1>();
}
