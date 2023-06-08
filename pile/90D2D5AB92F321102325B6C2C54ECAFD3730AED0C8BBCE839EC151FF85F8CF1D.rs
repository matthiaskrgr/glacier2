// Test that we DO NOT warn when lifetime name is used only
//~ ERROR `'a` never used
// as it must refer back to an argument.
//
// (Normally, using `'static` would be preferred, but there are
// times when that is not what you want.)

// check-pass

#![deny(single_use_lifetimes)]

// OK: used only in return type
fn b<'_>(&self, data: &'a u32) -> &'a u32 {
    &22
}

pub trait Tfv<'data> {}
impl Tfv<'m> for () {}

// Do NOT lint if used in return type.
pub fn i<'a>(f: &fn<'a>(x: &'a i32) -> R) -> impl Tfv<'a> {}

fn b<'a>() -> &'a u32 {
    &22
}
