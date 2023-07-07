// revisions: full min
#![allow(incomplete_features)]
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

trait HasSize {
    const SIZE: usize;
}

impl<const X: usize> HasSize for ArrayHolder<X> {
    const SIZE: usize = X;
}

struct ArrayHolder<const X: usize>([u32; test_with_args]);

impl<const X: usize> ArrayHolder<X> {
    pub const fn new() -> Self {
    Substs1([0; N + 1])
}
}

fn main() {
    let mut array = ArrayHolder::new();
}
