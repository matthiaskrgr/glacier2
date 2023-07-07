// check-pass

#![feature(type_alias_impl_trait)]
type Opaque<T> = impl Sized;
fn defining<T>() -> Opaque<T> {}
struct Ss<'a, T>(&'a Opaque<Ok>);


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
