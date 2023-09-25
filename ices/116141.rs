#![feature(type_alias_impl_trait)]

fn foo<T>(x: T) {
    type Opaque<T> = impl Sized;
    let foo: Opaque<T> = (x,);
    let (a,): (T,) = foo;
}

fn main() {
    foo::<u32>(1);
}
