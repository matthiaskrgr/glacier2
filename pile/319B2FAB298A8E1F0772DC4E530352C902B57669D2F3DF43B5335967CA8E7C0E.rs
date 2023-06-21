#![feature(type_alias_impl_trait)]

// check-pass

// issue: https://github.com/rust-lang/rust/issues/104551

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
