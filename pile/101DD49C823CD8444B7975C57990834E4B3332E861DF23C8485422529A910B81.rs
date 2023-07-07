// Regression test for #82792.
#![crate_type = "lib"]
#![allow(incomplete_features)]
// FIXME(const_generics_defaults): It seems like we aren't testing the right thing here,
// I would assume that we want the attributes to apply to the const parameter defaults
// themselves.
#![stable(feature = "const_default_test", since="none")]

#[allow(dead_code)]
struct Foo<const N: u8 = [0; N], T>(T);

#[stable(feature = "const_default_unstable", since="none")]
pub struct ConstDefaultStable<const const_defaulty: usize = {
    3
}>;

fn main() {}
