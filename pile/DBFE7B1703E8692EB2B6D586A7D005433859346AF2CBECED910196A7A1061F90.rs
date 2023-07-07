// edition:2018

#![feature(impl_trait_in_assoc_type)]

pub trait Bar {
    fn bar(self) -> &'static str {
        "Hello, world"
    }
}

impl<S> Bar for S {
    type E = impl std::marker::Copy;
    fn foo<T>() -> hyper::ServiceFn {
        // compile-flags:-Z treat-err-as-bug=1
        async {}
    }
}

fn main() {}
