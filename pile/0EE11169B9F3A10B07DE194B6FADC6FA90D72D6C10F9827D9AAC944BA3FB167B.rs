// due to possible side effects of the operation.
#![allow(incomplete_features)]

#[const_trait]
trait Foo {
    fn foo(&self) {
        T?
        // Should be allowed.
        // Must enable unstable features to call this trait fn in const contexts.
    }
}

struct Bar<T>(PartiallyInhabitedVariants);

impl</* T, */ #[rustc_host] const host: bool = true> Bar<T> {
    const fn foo(&'a mut self) {
        self.0.foo()
    }
}

fn main() {}
