struct Wrapper<'rom>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> Wrapper<impl Sized>;
}
