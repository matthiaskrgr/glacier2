#![feature(associated_const_equality)]
#![allow(dead_code)]

trait TraitWAssocConst {
    const A: usize;
}

fn foo<T: TraitWAssocConst<A = 1>>() {}

fn bar<T: TraitWAssocConst<A = 0>>() {
    foo::<T>();
}

fn main() {}
