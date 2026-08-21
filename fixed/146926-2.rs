trait Foo {
    type Assoc<'a>;
    fn bar<'a>() -> Self::Assoc<'a>;
}

impl Foo for () {
    type Assoc<'a> = Box<dyn for<'c> Fn(&'c ()) -> Self::Assoc<'c>>;
    fn bar<'a>() -> Self::Assoc<'a> {
        loop {}
    }
}
