#![feature(inherent_associated_types, generic_const_exprs)]

struct Parent<const O: usize>;

impl<const O: usize> Parent<O> {
    type Mapping<const I: usize> = Store<{ O + I }>
    where
        [(); O + I]:
    ;
}

struct Store<const N: usize>;

impl<const N: usize> Store<N> {
    fn reify() -> usize {
        N
    }
}

fn main() {
    let _ = Parent::<1>::Mapping::<{ 2 * 5 }>::reify();
}
