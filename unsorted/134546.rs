#![feature(generic_const_exprs)]

pub struct Foo<const C: usize>();

pub type FooAlias<T> = Foo<{std::mem::size_of::<T>()}>;

fn main() {
    let foo: FooAlias<[()]> = todo!();
}
