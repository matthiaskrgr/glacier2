fn r#struct() {
    #[derive(Copy, Clone)]
    struct Foo(PhantomData<T>);

    type U = impl Copy;
    let foo: U = Foo((1u32, 2u32));
    let Foo((0..=255, true)) = foo;
}
