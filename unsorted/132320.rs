trait Foo {
    type Item;
    fn foo(&mut self);
}

impl Foo for () {
    type Item = Option<()>;

    fn foo(&mut self) {
        let _ = Self::Item::None;
    }
}
