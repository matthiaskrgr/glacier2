struct Foo;
impl Foo {
    #[allow(dead_code)]
    fn foo(self) {
        panic!("wrong method!")
    }
}
trait Trait {
    fn foo(self);
}
impl<'a, 'b, 'c> Trait for &'a &'b &'c Foo {
    async fn pass2<'a, 'b, 'c>(self) {}
}
fn main() {
    let x = &(&(&Foo));
    x.foo();
}
