struct Foo<const V: u8>;

trait Bar<'b> {
    fn bar(n: u8);
}

// ICE *STILL* triggered if Foo doesn't have a const param
// ICE *NOT* if Bar doesn't have a lifetime param
// ICE *NOT* triggered obviously with correct `v if v == V`
impl<'b, const V: u8> Bar<'b> for Foo<V> {
    fn bar(n: u8) {
        match n {
            V => (),
            _ => (),
        }
    }
}
