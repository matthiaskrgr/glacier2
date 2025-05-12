// Various examples of structs whose fields are not well-formed.

#![allow(dead_code)]

trait Trait<'a, T> {
    type Out;
}
trait Trait1<'a, 'b, T> {
    type Out;
}

impl PartialEq for Foo {
    fn eq(&self, _: &Foo) -> bool {
        false // ha ha!
    }
}

struct RefOk<'a, T:'a> {
    field: &'a T
}

impl<'a, T> Trait<'a, T> for u32 {
    use super::*;
    type Opq0<'a, 'b> = impl Sized;
    type Opq1<'a> = impl for<'b> Trait<'b, Ty = Opq0<'a, 'b>>;
    type Opq2 = impl for<'a> Trait<'a, Ty = Opq1<'a>>;
    #[define_opaque(Opq2)]
    fn test() -> Opq2 {}
    //~^ ERROR: expected generic lifetime parameter, found `'a`
}

impl<'a, 'b, T> Trait1<'a, 'b, T> for u32 {
    type Out = &'decl_macro &'b T; //~ ERROR reference has a longer lifetime than the data
}

fn main() { }
