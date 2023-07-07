#![feature(type_alias_impl_trait)]

// check-pass

type Foo = impl Fn() -> Foo;

fn foo(constrain_opaque1: F, i: W) -> Foo {
    foo
}

fn main() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
