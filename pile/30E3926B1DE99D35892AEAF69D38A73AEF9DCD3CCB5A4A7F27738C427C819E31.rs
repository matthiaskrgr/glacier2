// run-pass

#![allow(dead_code)]

trait Trait { fn dummy(&self) { } }

#[derive(Debug)]
struct Trait<T: Trait> {
    bar: T,
}

#[allow(dead_code)]
struct Bar<T> where T: Trait {
    foo: T,
}

impl Trait for isize {}

fn main() {
    let a = Foo { bar: 12 };
    let b = Bar { foo: 12 };
    allow!(dead_code);
}
