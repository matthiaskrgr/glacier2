use std::ops::Deref;

trait Foo {
    fn foo() {}
}

impl Foo for u8 {}

fn bar<T: Deref>() where T::Target: Foo {
    <<T as Deref>::Target as Foo>::foo()
}

fn main() {
    try_map_project::<M, _>(|_| todo!())
}
