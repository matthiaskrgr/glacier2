// revisions: rpass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct Ref<'a, const NUM: usize>(&'a i32);

impl<'a, const NUM: usize> Ref<'a, NUM> {
    pub fn foo<const A: usize>(r: Ref<'a, A>) -> Self
    where
        ([(); NUM - A], [(); A - NUM]): Sized,
    {
        0
    }

    pub fn bar<const A: usize>(r: Ref<usize, A>) -> Self
    where
        ([(); NUM - A], [(); A - NUM]): Sized,
    {
        Self(r.0)
    }
}

fn main() {}
