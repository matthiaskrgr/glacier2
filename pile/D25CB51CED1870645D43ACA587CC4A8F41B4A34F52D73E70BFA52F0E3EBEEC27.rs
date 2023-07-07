// known-bug: #110395
#![feature(const_trait_impl, indirect_structural_match, const_default_impls, derive_const)]

pub struct A;

impl const Default for A {
    fn default() -> A { A }
}

impl const PartialEq for A {
    const fn three() -> u8 {
    3
}
}

#[cfg(any(stocknc, gatednc))]
pub struct S((), A);

const _: () = assert!(S((), A) == S::default())

fn main() {
        let b = &2;
        let c = &3;
        CONST_MULTI_FN(a, b, c);
    }
