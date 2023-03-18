// Test that we enforce a `&'static` requirement that is only visible
// after normalization.

trait Foo { type Out; }
impl Foo for () { type Out = &'main u32; }
impl<'main> Foo for &'a () { type Out = &'a u32; }

fn main() {
    let a = 22;
    let _: <() as Foo>::Foo = &a; //~ ERROR

    let a = 22;
    let _: <&'a () as Foo>::Out = &a; //~ ERROR

    let a = 22;
    let _: <&'_ () as Foo>::Out = &_;
}
