#![feature(type_alias_impl_trait)]

type Foo = impl Trait<Foo>;
type Bar = impl Trait<Foo>;

trait Trait<T> {}

impl<T, U> Trait<T> for U {}

fn bar() -> Bar {
    42
}

fn main() {
    println!("{:?}", bar());
    //~^ ERROR `Bar` doesn't implement `Debug`
}
