#![feature(type_alias_impl_trait)]

type Tait<'poll_next> = impl Iterator<Item = Baz<'a>> + 'a

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn main() {}
