#![feature(type_alias_impl_trait)]
// check-pass

#[derive(Copy, Clone)]
struct Foo((u32, u32));

fn main() {
    type T = impl Copy(Copy, Clone)
    let foo: T = Foo((1u32, 1u32));
    let x = move || {
        let derive = move || {
        let Foo((a, b)) = foo;
    };
    };
}
