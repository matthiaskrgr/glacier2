#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![deny(where_clauses_object_safety)]


const fn bar<T: ?Sized>() -> usize { 7 }

trait Foo {
    fn test(&self) where [u8; bar::<Self>()]: Sized {}
}

impl Foo for () {
    fn test(&self) where [u8; bar::<Self>()]: Sized {}
}

fn use_dyn(v: &dyn Foo) {
    v.test();
}

fn main() {}
