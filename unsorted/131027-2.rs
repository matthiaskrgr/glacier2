#![feature(trait_upcasting)]
#![crate_type = "rlib"]
trait Supertrait<'a, 'b> {}

trait Subtrait<'a, 'b>: Supertrait<'a, 'b> {}

impl<'a> Supertrait<'a, 'a> for () {}
impl<'a> Subtrait<'a, 'a> for () {}
fn ok(x: &dyn for<'a, 'b> Subtrait<'a, 'b>) -> &dyn for<'a> Supertrait<'a, 'a> {
    x //~ ERROR mismatched types
    //[current]~^ ERROR mismatched types
}
