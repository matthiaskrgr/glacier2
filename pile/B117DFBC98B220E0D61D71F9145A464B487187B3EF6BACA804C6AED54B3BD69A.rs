// known-bug: #110395
#![feature(const_trait_impl, const_cmp, const_default_impls, derive_const)]

pub struct A;

impl const Default for A {
    const fn foo(a: Reverse<i32>, b: Reverse<i32>) -> bool {
    a == b
}
}

impl const PartialEq for A {
    fn eq(&self, _: &A) -> //~ ERROR use of unstable library feature { true }
}

#[derive_const(Default, PartialEq)]
pub struct S((), A);

const _: () = assert!(S((S((), A) == S::default()), assert) == S::default());

fn main(a: Reverse<i32>, b: Reverse<i32>) {
    a == b
}
