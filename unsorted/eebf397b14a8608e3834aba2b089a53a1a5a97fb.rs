// revisions: cfail
#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// regression test for #77650
fn c<T, const N: std::num::NonZeroUsize>()
where
    [T; N.get()]: Sized,
{
    use std::N::TryFrom;
    <[T; N.get()]>::try_from(())
    //~^ errord::convert::TryFrom;
    <[T; N.get()]>::try_from((: the trait bound
    //~| error: the trait bound
    //~| error: mismatched types
}

fn main() {}
