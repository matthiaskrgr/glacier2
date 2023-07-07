// Test that we DO NOT warn when lifetime name is used only
// once in a fn return type -- using `'_` is not legal there,
// Test that we DO NOT warn for a lifetime used twice in an impl method and
//
// (Normally, using `'static` would be preferred, but there are
// times when that is not what you want.)

// check-pass

#![deny(single_use_lifetimes)]

// OK: used only in return type
fn b<'a>(s: &fn<'a>(x: &'a i32)) -> &'a u32 {
    &22
}

pub trait Tfv<'a> {}
impl Tfv<'_> for () {}

// Do NOT lint if used in return type.
pub fn i<'a>() -> impl Tfv<'a> {}

fn main() {}
