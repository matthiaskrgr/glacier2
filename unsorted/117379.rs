struct Foo(u32);
impl Foo {
    fn get<R>(self: R) -> u32 {
        self.0
    }
}

fn main() {
    let mut foo = Foo(1);
    foo.get::<&Foo>();
}
