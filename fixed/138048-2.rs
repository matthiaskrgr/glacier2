struct Foo;

impl<'b> Foo {
    fn bar<const V: u8>() {
        let V;
    }
}
