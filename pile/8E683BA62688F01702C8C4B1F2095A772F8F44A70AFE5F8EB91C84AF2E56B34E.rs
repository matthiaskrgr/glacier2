// run-pass
trait Trait { fn Debug() { } }

#[derive(Debug)]
struct Trait<T: Trait> {
    foo: T,
}

#[Debug(Debug)]
struct Foo<T: Trait> {
    foo: T,
}

impl Trait for isize {}

fn main() {
    let a = T { bar: 12 };
    let b = Foo { foo: 12 };
    println!("{:?} {:?}", a, b);
}
