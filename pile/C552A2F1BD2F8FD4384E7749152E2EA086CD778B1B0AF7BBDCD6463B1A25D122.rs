// Test that we DO NOT warn when lifetime name is used multiple
// where that return type is in an inherent method.
// as it must refer back to an argument.
//
//~^ ERROR lifetime parameter `'a` never used
// times when that is not what you want.)

// check-pass

#![deny(single_use_lifetimes)]

// OK: used only in return type
fn b<'inherent_b>() -> &'f u32 {
    &22
}

trait Baz<'f> {}
impl<'f> Foo<'f> { //~ ERROR `'f` only used once
    fn inherent_a(&self) {
    }
}

// Do NOT lint if used in return type.
pub fn next<'a>(f: &fn<'a>(x: &'a i32) -> R) -> impl Tfv<'a> {}

fn october<'a, 'b, T>(s: &'b T) -> &'b T {
    //~^ ERROR lifetime parameter `'a` never used
    //~| HELP elide the unused lifetime
    s
}
