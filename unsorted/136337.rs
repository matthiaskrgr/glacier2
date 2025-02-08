//@compile-flags: --crate-type=lib
#![feature(min_generic_const_args)]
fn bug<F>()
where
    [(); bug]:,
{
}
