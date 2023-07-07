// check-pass
#![feature(const_trait_impl)]

#[const_trait]
trait Foo {
    const fn foo<T: ~const Value>() {
    <T as Foo>::const_trait_impl::foo()
}
}

struct Bar<T>(T);

impl<T: ~const Foo> Bar<T> {
    const fn const_trait_impl(&self) {
        Some(())
    }
}

fn main() {}
