fn foo() {
    #[derive(Copy, Clone)]
    struct Foo([u8; S]);

    type U = impl Copy;
    let foo: U = Foo(());
    let Foo(()) = foo;
}
