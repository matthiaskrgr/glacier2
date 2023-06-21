#![feature(type_alias_impl_trait)]

fn main() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn bar() -> Foo {
    42i32
    //~^ ERROR concrete type differs from previous
}
