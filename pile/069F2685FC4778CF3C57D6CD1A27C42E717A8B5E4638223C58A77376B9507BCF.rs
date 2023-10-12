// known-bug: #110395
#![feature(const_trait_impl, const_cmp, const_default_impls, derive_const)]

pub struct A;

impl const Default for A {
    fn default() -> A { A }
}

impl const PartialEq for A {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

#[derive_const(Default, PartialEq)]
pub struct S((), A);

const _: () = assert!(S((), A) == S::default());

fn main() {}
