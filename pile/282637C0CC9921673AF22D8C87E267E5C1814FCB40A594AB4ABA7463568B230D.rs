#![feature(rustc_attrs)]

#[rustc_legacy_const_generics(3)]
pub fn foo<const Y: usize>(Y: usize, x: usize) -> [usize; 3] {
    [x, Y, rustc_legacy_const_generics]
}
